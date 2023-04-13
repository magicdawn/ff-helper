#![allow(dead_code)]
#![allow(unused_assignments)]
#![deny(clippy::all)]

mod helper;
mod screengen;

use helper::{ff, ffsys, to_napi_err, VideoInfo};
use napi::bindgen_prelude::*;
use napi::*;
use napi_derive::{module_exports, napi};
use std::{ffi::CStr, str::from_utf8_unchecked};

#[module_exports]
fn init(_: JsObject) -> napi::Result<()> {
  env_logger::init();
  ff::init().map_err(to_napi_err)?;
  Ok(())
}

/**
 * Return the libavutil build-time configuration.
 */
#[napi]
fn configuration() -> &'static str {
  ff::util::configuration()
}

/**
 * Return the LIBAVUTIL_VERSION_INT constant.
 */
#[napi]
fn version() -> u32 {
  ff::util::version()
}

/**
 * Return the libavutil license.
 */
#[napi]
fn license() -> &'static str {
  ff::util::license()
}

/**
 * Return an informative version string.
 * This usually is the actual release version number or a git commit description.
 * This string has no fixed format and can change any time.
 * It should never be parsed by code.
 */
#[napi]
fn version_info() -> &'static str {
  unsafe { from_utf8_unchecked(CStr::from_ptr(ffsys::av_version_info()).to_bytes()) }
}

struct GetVideoDuration {
  file: String,
}
#[napi]
impl Task for GetVideoDuration {
  type Output = i64;
  type JsValue = JsNumber;
  fn compute(&mut self) -> Result<Self::Output> {
    helper::get_duration(&helper::open(&self.file)?)
  }
  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int64(output)
  }
}

/**
 * get video duration synchronous, return number as ms
 */
#[napi]
fn get_video_duration_sync(file: String) -> napi::Result<i64> {
  GetVideoDuration { file }.compute()
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
    helper::get_rotation(&helper::open(&self.file)?)
  }
  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_int32(output)
  }
}

/**
 * get video rotation synchronous, in degrees (0-360), counterclockwise
 */
#[napi]
fn get_video_rotation_sync(file: String) -> napi::Result<i32> {
  GetVideoRotation { file }.compute()
}

/**
 * get video rotation, in degrees (0-360), counterclockwise
 */
#[napi]
fn get_video_rotation(file: String, signal: Option<AbortSignal>) -> AsyncTask<GetVideoRotation> {
  AsyncTask::with_optional_signal(GetVideoRotation { file }, signal)
}

#[napi]
fn get_metadata(file: String) -> napi::Result<()> {
  let input = ff::format::input(&file).map_err(to_napi_err)?;
  let format_metadata = input.metadata();
  println!("format metadata {:#?}", format_metadata);

  let video_stream = input
    .streams()
    .best(ff::media::Type::Video)
    .ok_or(helper::NO_VIDEO_STREAM_ERR.clone())?;
  let video_metadata = video_stream.metadata();
  println!("video metadata {:#?}", video_metadata);

  Ok(())
}

struct GetVideoInfo {
  file: String,
}

#[napi]
impl Task for GetVideoInfo {
  type Output = helper::VideoInfo;
  type JsValue = helper::VideoInfo;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = helper::open(&self.file)?;
    helper::get_info(&input)
  }

  // https://github.com/swc-project/swc/blob/v1.3.49/bindings/binding_core_node/src/transform.rs#L41-L42
  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

/**
 * get video information synchronous. (width, height, duration, rotation etc)
 */
#[napi]
fn get_video_info_sync(file: String) -> napi::Result<VideoInfo> {
  GetVideoInfo { file }.compute()
}

/**
 * get video information. (width, height, duration, rotation etc)
 */
#[napi]
fn get_video_info(file: String, signal: Option<AbortSignal>) -> AsyncTask<GetVideoInfo> {
  AsyncTask::with_optional_signal(GetVideoInfo { file }, signal)
}
