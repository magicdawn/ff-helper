import sharp from 'sharp'

import dayjs from 'dayjs'
import duration from 'dayjs/plugin/duration'
dayjs.extend(duration)

import * as addon from '../addon'
export * from '../addon'

/**
 * synchronous get humanized video duration for display, like `00:10:30` mean 10 minutes 30 seconds
 */
export function getVideoDurationDisplaySync(file: string) {
  return displayMs(addon.getVideoDurationSync(file))
}

/**
 * get humanized video duration for display, like `00:10:30` mean 10 minutes 30 seconds
 */
export async function getVideoDurationDisplay(file: string) {
  return displayMs(await addon.getVideoDuration(file))
}

function displayMs(ms: number) {
  return dayjs.duration(ms, 'milliseconds').format('HH:mm:ss')
}

/**
 * take a screenshot for video at given timestamp
 * node+rust version of https://gitlab.com/opennota/screengen
 *
 * @param file - the video file
 * @param ts - the given timestamp, in millseconds
 *
 * @returns Buffer encoded with mozjpeg, just write to a jpg or jpeg file
 *
 * @remarks the rust exported `getScreenshotAtSync` & `getScreenshotAt` returns raw pixel Buffer
 */

export async function screengen(file: string, ts: number) {
  const info = await addon.getVideoInfo(file)
  const pixelBuf = await addon.getScreenshotAt(file, ts)
  const jpegBuf = await sharp(pixelBuf, {
    raw: {
      channels: 4,
      width: info.width,
      height: info.height,
    },
  })
    .jpeg({ mozjpeg: true, quality: 85 })
    .withMetadata()
    .rotate(360 - info.rotation)
    .toBuffer()
  return jpegBuf
}
