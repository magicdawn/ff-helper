use super::moz;
use crate::{helper, screengen::_get_screenshot_raw};
use image::RgbaImage;
use log::debug;
use napi::Task;
use napi::bindgen_prelude::{AbortSignal, AsyncTask, Buffer};
use napi_derive::napi;
use rayon::prelude::*;
use std::time::Instant;

pub struct GetVideoPreviewRaw {
  file: String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
}
#[napi]
impl Task for GetVideoPreviewRaw {
  type Output = Buffer;
  type JsValue = Buffer;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(Buffer::from(_get_video_preview_raw(
      &self.file,
      self.rows,
      self.cols,
      self.frame_width,
      self.frame_height,
    )?))
  }
  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

/**
 * get video preview raw pixel buffer
 */
#[napi]
pub fn get_video_preview_raw(
  file: String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
  signal: Option<AbortSignal>,
) -> AsyncTask<GetVideoPreviewRaw> {
  AsyncTask::with_optional_signal(
    GetVideoPreviewRaw {
      file,
      rows,
      cols,
      frame_width,
      frame_height,
    },
    signal,
  )
}

///---------------------------------------------
/// jpeg
///---------------------------------------------

pub struct GetVideoPreview(GetVideoPreviewRaw);

#[napi]
impl Task for GetVideoPreview {
  type Output = Buffer;
  type JsValue = Buffer;
  fn compute(&mut self) -> napi::Result<Self::Output> {
    Ok(Buffer::from(_get_video_preview_jpeg(
      &self.0.file,
      self.0.rows,
      self.0.cols,
      self.0.frame_width,
      self.0.frame_height,
    )?))
  }
  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

/**
 * get video preview jpeg Buffer
 */
#[napi]
pub fn get_video_preview(
  file: String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
  signal: Option<AbortSignal>,
) -> AsyncTask<GetVideoPreview> {
  AsyncTask::with_optional_signal(
    GetVideoPreview(GetVideoPreviewRaw {
      file,
      rows,
      cols,
      frame_width,
      frame_height,
    }),
    signal,
  )
}

///------------------------------------------
/// impl details
///------------------------------------------

struct FramePos {
  x: u32,
  y: u32,
  index: u32, // 0 based
}

pub fn _get_video_preview_raw(
  file: &String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
) -> napi::Result<Vec<u8>> {
  let input = helper::open(file)?;
  let info = helper::get_info(&input)?;

  let count = rows * cols;
  let duration = info.duration;

  let mut frames: Vec<FramePos> = Vec::new();
  let mut whole_img = RgbaImage::new(frame_width * cols, frame_height * rows);

  let mut index: u32 = 0;
  for y in 0..rows {
    for x in 0..cols {
      frames.push(FramePos { x, y, index });
      index += 1;
    }
  }

  let start = Instant::now();
  let imgs: Vec<napi::Result<RgbaImage>> = frames
    .par_iter()
    .map(|pos| -> napi::Result<RgbaImage> {
      let x = pos.x;
      let y = pos.y;
      let index = pos.index;

      let ts = ((duration as f64 / count as f64) * (index as f64)).round() as i64;
      debug!("creating frame ({x},{y}) of (grid {cols}x{rows}) index={index} ts={ts}");

      let (vec, _, _) =
        _get_screenshot_raw(None, Some(file), ts, Some(frame_width), Some(frame_height))?;
      let img = RgbaImage::from_raw(frame_width, frame_height, vec)
        .ok_or_else(|| napi::Error::from_reason("can not create RgbaImage"))?;

      Ok(img)
    })
    .collect();
  debug!("create {count} frames cost {:?}", start.elapsed());

  // check errors
  for img in &imgs {
    if img.is_err() {
      return Err(img.clone().err().unwrap());
    }
  }

  /**
   * draw frame image
   * debug 耗时很长, 暂未找到怎么并行 put pixel 到 whole_img
   * release 耗时大大缩短
   */
  debug!("start overlay {count} frame imgs");
  let start = Instant::now();
  for (index, img) in imgs.iter().enumerate() {
    let frame = &frames[index];
    let fx = frame.x;
    let fy = frame.y;

    let _img = img.as_ref().unwrap();

    // use overlay
    image::imageops::overlay(
      &mut whole_img,
      _img,
      (fx * frame_width).into(),
      (fy * frame_height).into(),
    );
  }
  let elapsed = start.elapsed();
  debug!("overlay {count} frame imgs cost {elapsed:?}");

  Ok(whole_img.to_vec())
}

pub fn _get_video_preview_jpeg(
  file: &String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
) -> napi::Result<Vec<u8>> {
  let buf = _get_video_preview_raw(file, rows, cols, frame_width, frame_height)?;
  let img = RgbaImage::from_raw(frame_width * cols, frame_height * rows, buf)
    .ok_or_else(|| napi::Error::from_reason("can not construct RgbaImage"))?;
  let encoded = moz::mozjpeg_encode(&img, None)?;
  Ok(encoded)
}
