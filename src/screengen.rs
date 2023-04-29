// port of https://gitlab.com/opennota/screengen

use crate::helper::{self, *};
use ff::Rescale;
use image::{imageops, RgbaImage};
use log::debug;
use napi::bindgen_prelude::*;
use napi_derive::napi;

pub struct GetScreenshotRaw {
  file: String,
  ts: i64, // timestamp in millseconds
  width: Option<u32>,
  height: Option<u32>,
}

#[napi]
impl Task for GetScreenshotRaw {
  type JsValue = Buffer;
  type Output = Buffer;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    let vec = _get_screenshot_raw(None, Some(&self.file), self.ts, self.width, self.height)?.0;
    Ok(Buffer::from(vec))
  }
  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

/**
 * synchronous get screenshot raw pixel buffer at [ts] for [file],
 * optional [width] & [height] fallback to video width & height
 */
#[napi]
pub fn get_screenshot_raw_sync(
  file: String,
  ts: i64,
  width: Option<u32>,
  height: Option<u32>,
) -> napi::Result<Buffer> {
  GetScreenshotRaw {
    file,
    ts,
    width,
    height,
  }
  .compute()
}

/**
 * get screenshot raw pixel buffer at [ts] for [file],
 * optional [width] & [height] fallback to video width & height
 */
#[napi]
pub fn get_screenshot_raw(
  file: String,
  ts: i64,
  width: Option<u32>,
  height: Option<u32>,
  signal: Option<AbortSignal>,
) -> AsyncTask<GetScreenshotRaw> {
  AsyncTask::with_optional_signal(
    GetScreenshotRaw {
      file,
      ts,
      width,
      height,
    },
    signal,
  )
}

//
// some notes
//
// iterator calls `av_read_frame`
// send_packet calls `avcodec_send_packet`
// decoder.receive_frame calls `avcodec_receive_frame`
//
pub fn _get_screenshot_raw(
  input: Option<&mut Input>,
  file: Option<&String>,
  ts: i64,
  display_width: Option<u32>,
  display_height: Option<u32>,
) -> napi::Result<(Vec<u8>, u32, u32)> {
  let mut open_result: helper::Input;
  let input = if input.is_some() {
    input.unwrap()
  } else {
    open_result =
      helper::open(file.ok_or_else(|| to_napi_err("input & file can not be both empty"))?)?;
    &mut open_result
  };

  let info = helper::get_info(&input)?;

  /**
   * width & height, fallback to scale=1.0
   * display_width & display_height is display dimension, after rotation
   * width & height is frame dimension
   *
   * e.g 1920x1080 rotate 90 counterclockwise
   * display_width=1080 display_height=1920
   * width=1920 height=1080
   * frame_extract = 1920x1080
   * use image-rs to rotate to 1080x1920, same as (display_width x display_height)
   */
  let mut width = display_width.unwrap_or(info.display_width);
  let mut height = display_height.unwrap_or(info.display_height);
  if info.should_swap {
    (width, height) = (height, width)
  }

  let stream = input
    .streams()
    .best(MediaType::Video)
    .ok_or(NO_VIDEO_STREAM_ERR.clone())?;
  let stream_index = stream.index();
  let stream_time_base = stream.time_base();
  debug!(
    "video-stream: time_base={:?} duration={:?}, calc duration={}(s); get_video_duration={}(ms)",
    stream_time_base,
    stream.duration(),
    (stream.duration() as f64) * f64::from(stream_time_base),
    info.duration
  );

  let codec =
    ff::codec::context::Context::from_parameters(stream.parameters()).map_err(to_napi_err)?;
  let mut decoder = codec.decoder().video().map_err(to_napi_err)?;

  let use_input_seek = false;

  // 使用μs AV_TIME_BASE
  // seek 不太准确, 总是差 2s or 1s
  // 1/1000 aka 1ms -> 1/1000000 aka 1μs
  if use_input_seek {
    let seek_ts = (ts as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
    let acceptable_range: i64 = 1000;
    let seek_min_ts = (ts - acceptable_range as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
    let seek_max_ts = (ts + acceptable_range as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
    log::debug!(
      "[seek]: use input.seek ts={ts} seek_min_ts={seek_min_ts} seek_ts={seek_ts} seek_max_ts={seek_max_ts} stream_index={stream_index}");
    input
      .seek(seek_ts, seek_min_ts..seek_max_ts)
      .map_err(|e| napi::Error::from_reason(format!("can't seek to timestamp: {:?}", e)))?;
  }
  // 使用 stream time_base
  else {
    let seek_ts = (ts as i64).rescale((1, 1000), stream_time_base);
    let seek_min_ts: i64 = 0;
    let seek_max_ts = (ts + 1000 as i64).rescale((1, 1000), stream_time_base);
    log::debug!(
      "[seek]: use stream time_base({stream_time_base:?}) ts={ts} seek_min_ts={seek_min_ts} seek_ts={seek_ts} seek_max_ts={seek_max_ts} stream-index={stream_index}");
    unsafe {
      let seek_result = match ffsys::avformat_seek_file(
        input.as_mut_ptr(),
        stream_index as i32,
        seek_min_ts,
        seek_ts,
        seek_max_ts,
        // 0: no flag
        // ffsys::AVSEEK_FLAG_ANY enable non-keyframes, ffmpeg 未知错误 co located POCs unavailable
        // ffsys::AVSEEK_FLAG_ANY,
        0,
      ) {
        s if s >= 0 => Ok(()),
        e => Err(FFError::from(e)),
      };
      seek_result.map_err(|e| napi::Error::from_reason(format!("can't seek to timestamp {e:?}")))?
    }
  }

  // `avcodec_flush_buffers`
  decoder.flush();

  /* decode a frame */
  let mut decoded_frame = ff::frame::Video::empty();
  for (stream, packet) in input.packets() {
    if stream.index() == stream_index {
      // time check
      // https://gitlab.com/opennota/screengen/-/blob/v1.0.1/screengen.go?ref_type=tags#L284
      // let dts = packet.dts();
      // match dts {
      //   Some(val) => {
      //     if val < seek_ts {
      //       debug!("skip packet for dts < seek_ts");
      //       continue;
      //     }
      //   }
      //   None => {
      //     continue;
      //   }
      // }

      debug!(
        "decoder.send_packet: dts={:?} pts={:?} ts={:?} video_stream_time_base={:?} byte-position={:?}",
        packet.dts(),
        packet.pts(),
        (packet.pts().unwrap_or(0) as f64) * f64::from(stream_time_base),
        stream_time_base,
        packet.position(),
      );
      decoder.send_packet(&packet).map_err(to_napi_err)?;

      let receive_result = decoder.receive_frame(&mut decoded_frame);
      match receive_result {
        Err(err) => match err {
          FFError::Other { errno } => {
            if errno == ff::error::EAGAIN {
              log::trace!("receive_frame EAGAIN");
              continue;
            }
            log::trace!("receive_frame error: errno={errno} {err:?}");
          }
          _ => {}
        },
        _ => {}
      }
      receive_result.map_err(to_napi_err)?; // other error: early return

      unsafe {
        if decoded_frame.is_empty() {
          debug!("continue for empty frame");
          continue;
        }
      }

      break;
    }
  }
  debug!(
    "decoded_frame: timestamp={:?} pts={:?} expect_ts={:?} ms frame_ts={:?} s",
    decoded_frame.timestamp(),
    decoded_frame.pts(),
    ts,
    (decoded_frame.timestamp().unwrap_or(0) as f64) * f64::from(stream_time_base),
  );

  // using iamge::iamgeops::resize can also resize dimensions
  // but sws_scale can resize between different pixel formats. e.g YUV420p -> RGBA
  let mut scaler = ff::software::scaling::Context::get(
    decoder.format(),
    decoder.width(),
    decoder.height(),
    ff::format::Pixel::RGBA,
    width,
    height,
    // https://blog.csdn.net/leixiaohua1020/article/details/12029505
    // https://stackoverflow.com/questions/29743648/which-flag-to-use-for-better-quality-with-sws-scale
    // https://github.com/mutschler/mt/blob/master/mt.go
    // mt used lanczos
    ff::software::scaling::Flags::LANCZOS,
  )
  .map_err(to_napi_err)?;
  let mut decoded_frame_scaled = ff::frame::Video::new(ff::format::Pixel::RGBA, width, height);
  scaler
    .run(&decoded_frame, &mut decoded_frame_scaled)
    .map_err(to_napi_err)?;

  let buf = decoded_frame_scaled.data(0);
  debug!(
    "decoded_frame_scaled: planes={} stride={} width={} height={} buf.size={}",
    decoded_frame_scaled.planes(),
    decoded_frame_scaled.stride(0),
    width,
    height,
    buf.len()
  );

  // no rotation
  if info.rotation == 0 {
    return Ok((buf.to_vec(), width, height));
  }

  // rotate image
  let mut image = RgbaImage::from_raw(width, height, buf.to_vec())
    .ok_or_else(|| napi::Error::from_reason("can not create RgbaImage"))?;
  match info.rotation {
    90 => {
      image = imageops::rotate270(&image);
    }
    180 => {
      imageops::rotate180_in_place(&mut image);
    }
    270 => {
      image = imageops::rotate90(&image);
    }
    _ => {
      return Err(napi::Error::from_reason(format!(
        "unsupported rotation {}",
        info.rotation
      )));
    }
  }

  let img_vec = image.to_vec();
  Ok((img_vec, width, height))
}
