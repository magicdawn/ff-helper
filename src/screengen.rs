// port of https://gitlab.com/opennota/screengen

use crate::helper::{self, *};
use ff::Rescale;
use log::debug;
use napi::bindgen_prelude::*;
use napi_derive::napi;

/**
 * @param ts: timestamp in millseconds
 */

pub struct GetScreenshotAt {
  file: String,
  ts: i64,
}

#[napi]
impl Task for GetScreenshotAt {
  type JsValue = Buffer;
  type Output = Buffer;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    _get_screenshot_at(&self.file, self.ts)
  }
  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}
#[napi]
pub fn get_screenshot_at_sync(file: String, ts: i64) -> napi::Result<Buffer> {
  GetScreenshotAt { file, ts }.compute()
}
#[napi]
pub fn get_screenshot_at(
  file: String,
  ts: i64,
  signal: Option<AbortSignal>,
) -> AsyncTask<GetScreenshotAt> {
  AsyncTask::with_optional_signal(GetScreenshotAt { file, ts }, signal)
}

//
// some notes
//
// iterator calls `av_read_frame`
// send_packet calls `avcodec_send_packet`
// decoder.receive_frame calls `avcodec_receive_frame`
//
pub fn _get_screenshot_at(file: &String, ts: i64) -> NapiResult<Buffer> {
  let mut input = helper::open(file)?;
  let info = helper::get_info(&input)?;
  // target size, current use 1.0
  let width = info.width;
  let height = info.height;

  let video_stream = input
    .streams()
    .best(MediaType::Video)
    .ok_or(NO_VIDEO_STREAM_ERR.clone())?;
  let video_stream_index = video_stream.index();
  let video_stream_time_base = video_stream.time_base();
  debug!(
    "video-stream: time_base={:?} duration={:?}, calc duration={}(s); get_video_duration={}(ms)",
    video_stream_time_base,
    video_stream.duration(),
    (video_stream.duration() as f64) * f64::from(video_stream_time_base),
    info.duration
  );

  let codec =
    ff::codec::context::Context::from_parameters(video_stream.parameters()).map_err(to_napi_err)?;
  let mut decoder = codec.decoder().video().map_err(to_napi_err)?;

  // 不采用 stream 时间, seek 不太准确, 总是差 2s or 1s
  // 1/1000 aka 1ms -> 1/1000000 aka 1μs
  let seek_ts = (ts as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
  let acceptable_range: i64 = 1000;
  let seek_min_ts = (ts - acceptable_range as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
  let seek_max_ts = (ts + acceptable_range as i64).rescale((1, 1000), ff::rescale::TIME_BASE);
  log::debug!(
    "[seek]: ts={} seek_min_ts={} seek_ts={} seek_max_ts={} video-stream-index={}",
    ts,
    seek_min_ts,
    seek_ts,
    seek_max_ts,
    video_stream_index,
  );
  input
    .seek(seek_ts, seek_min_ts..seek_max_ts)
    .map_err(|e| napi::Error::from_reason(format!("can't seek to timestamp: {:?}", e)))?;

  // seek 不准确
  // let seek_ts = (ts as i64).rescale((1, 1000), video_stream_time_base);
  // let seek_min_ts: i64 = 0;
  // let seek_max_ts = (ts + 1000 as i64).rescale((1, 1000), video_stream_time_base);
  // log::debug!(
  //   "[seek]: ts={} seek_min_ts={} seek_ts={} seek_max_ts={} video-stream-index={}",
  //   ts,
  //   seek_min_ts,
  //   seek_ts,
  //   seek_max_ts,
  //   video_stream_index,
  // );
  // unsafe {
  //   let seek_result = match ffsys::avformat_seek_file(
  //     input.as_mut_ptr(),
  //     video_stream_index.try_into().unwrap(),
  //     seek_min_ts,
  //     seek_ts,
  //     seek_max_ts,
  //     // ffsys::AVSEEK_FLAG_ANY,
  //     // 0: no flag
  //     0,
  //   ) {
  //     s if s >= 0 => Ok(()),
  //     e => Err(FFError::from(e)),
  //   };
  //   seek_result
  //     .map_err(|e| napi::Error::from_reason(format!("can't seek to timestamp -> {:?}", e)))?
  // }

  // `avcodec_flush_buffers`
  decoder.flush();

  // let mut img = image::RgbaImage::new(0, 0);
  // img = image::RgbaImage::from_raw(width, height, buf.to_vec()).ok_or_else(|| {
  //   napi::Error::from_reason(format!("failed to convert &[u8] to image::RgbaImage"))
  // })?

  /* decode a frame */
  let mut decoded_frame = ff::frame::Video::empty();
  for (stream, packet) in input.packets() {
    if stream.index() == video_stream_index {
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
        "decoder.send_packet: dts={:?} pts={:?} ts={:?} video_stream_time_base={:?} byte-position={:?} is_corrupt={:?}",
        packet.dts(),
        packet.pts(),
        (packet.pts().unwrap_or(0) as f64) * f64::from(video_stream_time_base),
        video_stream_time_base,
        packet.position(),
        packet.is_corrupt(),
      );
      decoder.send_packet(&packet).map_err(to_napi_err)?;

      let receive_result = decoder.receive_frame(&mut decoded_frame);
      match receive_result {
        Err(err) => match err {
          FFError::Other { errno } => {
            log::trace!("receive_frame error errno={} pretty -> {:?}", errno, err);
            if errno == ff::error::EAGAIN {
              continue;
            }
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

  let mut scaler = ff::software::scaling::Context::get(
    decoder.format(),
    decoder.width(),
    decoder.height(),
    ff::format::Pixel::RGBA,
    width,
    height,
    ff::software::scaling::Flags::BICUBIC,
  )
  .map_err(to_napi_err)?;

  let mut decoded_frame_scaled = ff::frame::Video::new(ff::format::Pixel::RGBA, width, height);
  scaler
    .run(&decoded_frame, &mut decoded_frame_scaled)
    .map_err(to_napi_err)?;

  debug!(
    "decoded_frame: timestamp={:?} pts={:?}",
    decoded_frame.timestamp(),
    decoded_frame.pts()
  );

  let buf = decoded_frame_scaled.data(0);
  debug!(
    "decoded_frame_scaled: planes={} stride={} width={} height={} buf.size={}",
    decoded_frame_scaled.planes(),
    decoded_frame_scaled.stride(0),
    width,
    height,
    buf.len()
  );

  let js_buf = Buffer::from(buf);
  Ok(js_buf)
}
