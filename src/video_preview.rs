use crate::{helper, screengen::_get_screenshot_at};
use image::{imageops, RgbaImage};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi]
pub fn video_preview(
  file: String,
  rows: u32,
  cols: u32,
  frame_width: u32,
  frame_height: u32,
) -> napi::Result<Buffer> {
  let vec = _video_preview(&file, rows, cols, frame_width, frame_height)?;
  Ok(Buffer::from(vec))
}

pub fn _video_preview(
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

  let mut whole_img = RgbaImage::new(frame_width * cols, frame_height * rows);

  let mut index = 0;
  for i in 0..rows {
    for j in 0..cols {
      index += 1; // 1 based
      let ts = ((duration as f64 / count as f64) * ((index - 1) as f64)).round() as i64;

      let (vec, width, height) =
        _get_screenshot_at(file, ts, Some(frame_width), Some(frame_height))?;
      let img = RgbaImage::from_raw(width, height, vec)
        .ok_or_else(|| napi::Error::from_reason("can not create RgbaImage"))?;

      // let x =
      imageops::overlay(
        &mut whole_img,
        &img,
        (j * frame_width).into(),
        (i * frame_height).into(),
      );
    }
  }

  Ok(whole_img.to_vec())
}
