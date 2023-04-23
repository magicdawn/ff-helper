# CHANGELOG

## v0.2.2 2023-04-23

- rebundle ff-helper.darwin-x64.node for ffmpeg v6, brew has ffmpeg v6 released.

## v0.2.1 2023-04-14

- feat: port go version screengen https://gitlab.com/opennota/screengen
- fix: getDuration lost ms part

## v0.2.0 2023-04-13

- 45758b7 api: add getVideoInfoSync
- cff016c rs: eliminate unwrap
- 4acacaf rs: make `&[u8]` to `* const i32`

## v0.1.0 2023-04-11

- test: use network video
- rs: version / license etc
- feat: catch_unwind, mod helper, get_video_info

## v0.0.3 2023-04-11

- utilize napi-rs `struct AsyncTask` & `trait Task` to support async version

## v0.0.2 2023-04-11

- bundle ff-helper.darwin-x64.node
- when install, try `addon.js`, if fail, will fallback to `napi build`

## v0.0.1 2023-04-10

- first release
