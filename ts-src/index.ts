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
 * take a screenshot for video at given timestamp. \n
 * node+rust version of https://gitlab.com/opennota/screengen
 *
 * @param file the video file
 * @param ts the given timestamp, in millseconds
 * @param displayWidth expected image width
 * @param displayHeight expected image height
 *
 * @returns Buffer encoded with mozjpeg, just write to a jpg or jpeg file
 * @remarks the rust exported `getScreenshotAtSync` & `getScreenshotAt` returns raw RGBA pixel Buffer
 */

export async function screengen(
  file: string,
  ts: number,
  displayWidth?: number,
  displayHeight?: number
) {
  const info = await addon.getVideoInfo(file)

  // fallback scale=1.0
  displayWidth ||= info.displayWidth
  displayHeight ||= info.displayHeight

  // to int
  displayWidth = Math.trunc(displayWidth)
  displayHeight = Math.trunc(displayHeight)

  const pixelBuf = await addon.getScreenshotAt(file, ts, displayWidth, displayHeight)
  const jpegBuf = await sharp(pixelBuf, {
    raw: {
      channels: 4,
      width: displayWidth,
      height: displayHeight,
    },
  })
    .jpeg({ mozjpeg: true, quality: 85 })
    .withMetadata()
    .toBuffer()
  return jpegBuf
}

/**
 * take a screenshot for video at given timestamp.
 *
 * @param file the video file
 * @param ts the given timestamp, in millseconds
 * @param scale scale of video width & height
 *
 * @returns Buffer encoded with mozjpeg, just write to a jpg or jpeg file
 */

export async function screengenScale(file: string, ts: number, scale?: number) {
  const info = await addon.getVideoInfo(file)

  // fallback scale=1.0
  scale ||= 1
  if (scale > 1) scale = 1
  if (scale <= 0) throw new Error('scale <= 0 not supported')

  const width = info.displayWidth * scale
  const height = info.displayHeight * scale
  return screengen(file, ts, width, height)
}
