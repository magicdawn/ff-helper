# ff-helper

> ffmpeg helper by napi-rs binding

[![Build Status](https://img.shields.io/github/actions/workflow/status/magicdawn/ff-helper/ci.yml?style=flat-square&branch=main)](https://github.com/magicdawn/ff-helper/actions/workflows/ci.yml)
[![Coverage Status](https://img.shields.io/codecov/c/github/magicdawn/ff-helper.svg?style=flat-square)](https://codecov.io/gh/magicdawn/ff-helper)
[![npm version](https://img.shields.io/npm/v/ff-helper.svg?style=flat-square)](https://www.npmjs.com/package/ff-helper)
[![npm downloads](https://img.shields.io/npm/dm/ff-helper.svg?style=flat-square)](https://www.npmjs.com/package/ff-helper)
[![npm license](https://img.shields.io/npm/l/ff-helper.svg?style=flat-square)](http://magicdawn.mit-license.org)

## Install rust toolchain

This binding will link against system installed ffmpeg(libavcodec libav\*). <br />
It will run cargo build when install, make sure you have rust toolchain installed.

## Install ffmpeg lib

### macOS

```sh
brew install pkg-config ffmpeg
```

### On Debian-based systems

```sh
apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config
```

more see https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building

## Install

```sh
$ pnpm add ff-helper
```

## API

see typedoc https://magicdawn.github.io/ff-helper/

### get video info

```ts
export declare async funciton getVideoInfo(file: string): Promise<VideoInfo>;

export interface VideoInfo {
  /** degress, 0-360, counterclockwise  */
  rotation: number
  /** millseconds  */
  duration: number
  width: number
  height: number
}
```

## Changelog

[CHANGELOG.md](CHANGELOG.md)

## License

the MIT License http://magicdawn.mit-license.org
