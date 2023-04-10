#![allow(dead_code)]
#![allow(unused_assignments)]
#![deny(clippy::all)]

use ffmpeg_next as ff;
use ffmpeg_sys_next as ffsys;
use napi::bindgen_prelude::*;
use napi::*;
use napi_derive::napi;

#[napi]
fn configuration() -> &'static str {
  ff::util::configuration()
}

struct GetVideoDuration {
  file: String,
}
#[napi]
impl Task for GetVideoDuration {
  type Output = i64;
  type JsValue = JsNumber;
  fn compute(&mut self) -> Result<Self::Output> {
    let input = ff::format::input(&self.file).unwrap();
    // AVFormatContext.duration: in AV_TIME_BASE fractional seconds
    let duration = input.duration() / (ffsys::AV_TIME_BASE as i64) * 1000;
    Ok(duration)
  }
  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int64(output)
  }
}

/**
 * get video duration sync, return number as ms
 */
#[napi]
fn get_video_duration_sync(file: String) -> i64 {
  GetVideoDuration { file }.compute().unwrap()
}

/**
 * get video duration, return number as ms
 */
#[napi]
fn get_video_duration(file: String, signal: Option<AbortSignal>) -> AsyncTask<GetVideoDuration> {
  AsyncTask::with_optional_signal(GetVideoDuration { file }, signal)
}

struct GetVideoRotation {
  file: String,
}
#[napi]
impl Task for GetVideoRotation {
  type Output = i32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut rotation: i32 = 0;

    let input = ff::format::input(&self.file).unwrap();
    let video_stream = input.streams().best(ff::media::Type::Video).unwrap();
    let display_matrix = video_stream
      .side_data()
      .find(|side| side.kind() == ff::codec::packet::side_data::Type::DisplayMatrix);

    match display_matrix {
      Some(side_data) => {
        let buf = side_data.data();

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
      None => {
        //
      }
    }

    // 0-360
    rotation = (rotation + 360) % 360;
    Ok(rotation)
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int32(output)
  }
}

/**
 * get video rotation sync, in degrees (0-360), counterclockwise
 */
#[napi]
fn get_video_rotation_sync(file: String) -> i32 {
  GetVideoRotation { file }.compute().unwrap()
}

/**
 * get video rotation, in degrees (0-360), counterclockwise
 */
#[napi]
fn get_video_rotation(file: String, signal: Option<AbortSignal>) -> AsyncTask<GetVideoRotation> {
  AsyncTask::with_optional_signal(GetVideoRotation { file }, signal)
}
