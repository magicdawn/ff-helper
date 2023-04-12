use napi_derive::napi;
use std::panic::{catch_unwind, UnwindSafe};

// lib alias
pub use ffmpeg_next as ff;
pub use ffmpeg_sys_next as ffsys;

// type shortcut
pub use ff::codec::packet::side_data::Type as SideDataType;
pub use ff::format::context::Input;
pub use ff::media::Type as MediaType;
pub use ff::Error as FFError;

pub fn open(file: &String) -> Input {
  ff::format::input(file).unwrap()
}

pub fn get_duration(input: &Input) -> i64 {
  // AVFormatContext.duration: in AV_TIME_BASE fractional seconds
  let duration = input.duration() / (ffsys::AV_TIME_BASE as i64) * 1000;
  duration
}

pub fn get_rotation(input: &Input) -> i32 {
  let mut rotation: i32 = 0;

  let video_stream = input.streams().best(MediaType::Video).unwrap();

  let display_matrix = video_stream
    .side_data()
    .find(|side| side.kind() == SideDataType::DisplayMatrix);

  if let Some(matrix_side_data) = display_matrix {
    let buf = matrix_side_data.data();

    let matrix = buf
      .chunks(4)
      .map(|c| i32::from_ne_bytes(c.try_into().unwrap()))
      .collect::<Vec<_>>();

    let mut _rotation: f64 = 0.0;
    unsafe {
      // @return the angle (in degrees) by which the transformation rotates the frame counterclockwise.
      // The angle will be in range -180.0, 180.0, or NaN if the matrix is singular.
      _rotation = ffsys::av_display_rotation_get(matrix.as_ptr());
    }
    rotation = _rotation.round() as i32
  }

  // 0-360
  rotation = (rotation + 360) % 360;
  rotation
}

#[napi(object)]
#[derive(Debug, Default)]
pub struct VideoInfo {
  /** degress, 0-360, counterclockwise */
  pub rotation: i32,
  /** millseconds */
  pub duration: i64,
  pub width: u32,
  pub height: u32,
}

pub fn get_info(input: &Input) -> VideoInfo {
  let duration = get_duration(input);
  let rotation = get_rotation(input);

  let video_stream = input.streams().best(MediaType::Video).unwrap();

  let codec = ff::codec::context::Context::from_parameters(video_stream.parameters()).unwrap();
  let decoder = codec.decoder().video().unwrap();

  let width = decoder.width();
  let height = decoder.height();

  let info = VideoInfo {
    duration,
    rotation,
    width,
    height,
  };
  info
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
    Err(cause) => {
      if let Ok(cause_string) = cause.downcast::<String>() {
        Err(napi::Error::from_reason(format!(
          "rust panic: {}",
          cause_string
        )))
      } else {
        Err(napi::Error::from_reason(format!(
          "rust panic: {}",
          "uknown reason"
        )))
      }
    }
  }
}
