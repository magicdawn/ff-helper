#![allow(dead_code)]
#![allow(unused_assignments)]
#![deny(clippy::all)]

mod helper;

use std::{ffi::CStr, str::from_utf8_unchecked};

use helper::{ff, ffsys, napi_catch_unwind};
use napi::bindgen_prelude::*;
use napi::*;
use napi_derive::{module_exports, napi};

#[module_exports]
fn init(_: JsObject) -> Result<()> {
  ff::init().unwrap();
  Ok(())
}

#[napi]
fn configuration() -> &'static str {
  ff::util::configuration()
}
#[napi]
fn version() -> u32 {
  ff::util::version()
}
#[napi]
fn license() -> &'static str {
  ff::util::license()
}
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
    napi_catch_unwind(|| helper::get_duration(&helper::open(&self.file)))
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
    napi_catch_unwind(|| helper::get_rotation(&helper::open(&self.file)))
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

#[napi]
fn get_metadata(file: String) {
  let input = ff::format::input(&file).unwrap();
  let format_metadata = input.metadata();
  println!("format metadata {:#?}", format_metadata);

  let video_stream = input.streams().best(ff::media::Type::Video).unwrap();
  let video_metadata = video_stream.metadata();
  println!("video metadata {:#?}", video_metadata);
}

struct GetVideoInfo {
  file: String,
}

#[napi]
impl Task for GetVideoInfo {
  type Output = helper::VideoInfo;
  type JsValue = helper::VideoInfo;

  fn compute(&mut self) -> Result<Self::Output> {
    napi_catch_unwind(|| {
      let input = helper::open(&self.file);
      helper::get_info(&input)
    })
  }

  // https://github.com/swc-project/swc/blob/v1.3.49/bindings/binding_core_node/src/transform.rs#L41-L42
  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
fn get_video_info(file: String, signal: Option<AbortSignal>) -> AsyncTask<GetVideoInfo> {
  AsyncTask::with_optional_signal(GetVideoInfo { file }, signal)
}
