# CHANGELOG

## v0.6.1

- 7a7af0b chore: update rs & js deps

## v0.6.0

- bundle ff-helper.darwin-arm64.node compiled from Apple M2
- update cargo deps

## v0.5.0 2023-05-10

- feat: use rust mozjpeg crate, rm npm sharp dep. requires nasm

## v0.4.2 2023-05-01

- ca050f1 chore: tweak rustfmt options
- 262a535 chore: simplify rs code

## v0.4.1 2023-04-30

- chore: ffmpeg seek & image overlay tweak

## v0.4.0 2023-04-29

- feat: add `getVideoPreview`, rename rust exported to raw api, `getScreenshotRaw` / `getVideoPreviewRaw`

## v0.3.0 2023-04-28

- feat: screengen, add displayWidth & displayHeight to rust layer.
- feat: screengen, handle rotation inside screengen
- feat: screengen, add `screengenScale`

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
