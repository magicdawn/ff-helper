// lib alias
pub use ffmpeg_next as ff;
pub use ffmpeg_sys_next as ffsys;

// type shortcut
pub use ff::Error as FFError;
pub use ff::codec::packet::side_data::Type as SideDataType;
pub use ff::format::context::Input;
pub use ff::media::Type as MediaType;

use ff::Rescale;
use napi_derive::napi;
use once_cell::sync::Lazy;
use std::{
  any::Any,
  panic::{UnwindSafe, catch_unwind},
};

pub type NapiResult<T> = napi::Result<T>;

pub const NO_VIDEO_STREAM: &str = "can not find any video stream in file";

pub static NO_VIDEO_STREAM_ERR: Lazy<napi::Error> =
  Lazy::new(|| napi::Error::from_reason(NO_VIDEO_STREAM));

pub fn to_napi_err(err: impl std::fmt::Debug) -> napi::Error {
  napi::Error::from_reason(format!("{:?}", err))
}

pub fn open(file: &String) -> NapiResult<Input> {
  ff::format::input(file).map_err(to_napi_err)
}

pub fn get_duration(input: &Input) -> NapiResult<i64> {
  // AVFormatContext.duration: in AV_TIME_BASE fractional seconds
  // target: scale to 1/1000s aka ms
  let duration = input.duration().rescale(ff::rescale::TIME_BASE, (1, 1000));
  Ok(duration)
}

pub fn get_rotation(input: &Input) -> NapiResult<i32> {
  let mut rotation: i32 = 0;

  let video_stream = input
    .streams()
    .best(MediaType::Video)
    .ok_or(NO_VIDEO_STREAM_ERR.clone())?;

  let display_matrix = video_stream
    .side_data()
    .find(|side| side.kind() == SideDataType::DisplayMatrix);

  if let Some(matrix_side_data) = display_matrix {
    let buf = matrix_side_data.data();
    let ptr = buf.as_ptr() as *const i32;

    let mut _rotation: f64 = 0.0;
    unsafe {
      // @return the angle (in degrees) by which the transformation rotates the frame counterclockwise.
      // The angle will be in range -180.0, 180.0, or NaN if the matrix is singular.
      _rotation = ffsys::av_display_rotation_get(ptr);
    }

    rotation = _rotation.round() as i32
  }

  // 0-360
  rotation = (rotation + 360) % 360;
  Ok(rotation)
}

#[napi(object)]
#[derive(Debug, Default)]
pub struct VideoInfo {
  /** degress, 0-360, counterclockwise */
  pub rotation: i32,
  /** check if rotation = 90 | 270 */
  pub should_swap: bool,

  /** millseconds */
  pub duration: i64,

  /** raw width, before apply rotation  */
  pub width: u32,
  /** raw height, before apply rotation  */
  pub height: u32,

  /** display width, after apply rotation  */
  pub display_width: u32,
  /** display height, after apply rotation  */
  pub display_height: u32,
}

pub fn get_info(input: &Input) -> NapiResult<VideoInfo> {
  let duration = get_duration(input)?;
  let rotation = get_rotation(input)?;

  let video_stream = input
    .streams()
    .best(MediaType::Video)
    .ok_or(NO_VIDEO_STREAM_ERR.clone())?;

  let codec =
    ff::codec::context::Context::from_parameters(video_stream.parameters()).map_err(to_napi_err)?;
  let decoder = codec.decoder().video().map_err(to_napi_err)?;

  let width = decoder.width();
  let height = decoder.height();

  let should_swap = match rotation {
    90 | 270 => true,
    _ => false,
  };

  let (display_width, display_height) = if should_swap {
    (height, width)
  } else {
    (width, height)
  };

  let info = VideoInfo {
    duration,
    rotation,
    width,
    height,
    should_swap,
    display_width,
    display_height,
  };
  Ok(info)
}

/**
 * catch_unwind + napi::Result
 */
pub fn napi_catch_unwind<F, R>(f: F) -> napi::Result<R>
where
  F: FnOnce() -> R + UnwindSafe,
{
  let result = catch_unwind(f);
  match result {
    Ok(inner) => Ok(inner),
    Err(cause) => Err(panic_cause_to_napi_error(cause)),
  }
}

pub fn panic_cause_to_napi_error(cause: Box<dyn Any + Send>) -> napi::Error {
  if let Ok(cause_string) = cause.downcast::<String>() {
    napi::Error::from_reason(format!("rust panic: {}", cause_string))
  } else {
    napi::Error::from_reason(format!("rust panic: {}", "uknown reason"))
  }
}
