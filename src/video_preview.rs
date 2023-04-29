use crate::{helper, screengen::_get_screenshot_raw};
use image::{GenericImage, RgbaImage};
use log::debug;
use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask, Buffer},
  Task,
};
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
    let vec = _get_video_preview_raw(
      &self.file,
      self.rows,
      self.cols,
      self.frame_width,
      self.frame_height,
    )?;
    Ok(Buffer::from(vec))
  }
  fn resolve(&mut self, _: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn get_video_preview_raw_sync(
  file: String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
) -> napi::Result<Buffer> {
  GetVideoPreviewRaw {
    file,
    rows,
    cols,
    frame_width,
    frame_height,
  }
  .compute()
}

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

  let imgs: Vec<napi::Result<RgbaImage>> = frames
    .par_iter()
    .map(|pos| -> napi::Result<RgbaImage> {
      let x = pos.x;
      let y = pos.y;
      let index = pos.index;

      let ts = ((duration as f64 / count as f64) * (index as f64)).round() as i64;
      debug!("creating frame ({x},{y}) of (grid {cols}x{rows}) index={index} ts={ts}");

      let (vec, width, height) =
        _get_screenshot_raw(None, Some(file), ts, Some(frame_width), Some(frame_height))?;
      let img = RgbaImage::from_raw(width, height, vec)
        .ok_or_else(|| napi::Error::from_reason("can not create RgbaImage"))?;

      Ok(img)
    })
    .collect();

  // check errors
  for img in &imgs {
    if img.is_err() {
      return Err(img.clone().err().unwrap());
    }
  }

  /**
   * draw frame image
   * 耗时很长, 暂未找到怎么并行 put pixel 到 whole_img
   */
  debug!("start overlay {count} frame imgs");
  let start = Instant::now();
  for (index, img) in imgs.iter().enumerate() {
    let frame = &frames[index];
    let fx = frame.x;
    let fy = frame.y;

    let _img = img.as_ref().unwrap();

    // use overlay
    // imageops::overlay(
    //   &mut whole_img,
    //   _img,
    //   (fx * frame_width).into(),
    //   (fy * frame_height).into(),
    // );

    // manual put
    let mut sub_image = whole_img.sub_image(
      fx * frame_width,
      fy * frame_height,
      frame_width,
      frame_height,
    );
    for y in 0..frame_height {
      for x in 0..frame_width {
        sub_image.put_pixel(x, y, _img.get_pixel(x, y).clone())
      }
    }
  }
  let elapsed = start.elapsed();
  debug!("overlay {count} frame imgs cost {elapsed:?}");

  Ok(whole_img.to_vec())
}
