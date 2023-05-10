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

function validateScale(scale?: number): number {
  // fallback scale=1.0
  scale ||= 1
  if (scale > 1) scale = 1
  if (scale <= 0) throw new Error('scale <= 0 not supported')
  return scale
}

/**
 * screenshot for video, with scale
 *
 * @param file the video file
 * @param ts the given timestamp, in millseconds
 * @param scale scale of video width & height
 *
 * @returns Buffer encoded with mozjpeg
 */
export async function getScreenshotScale(file: string, ts: number, scale?: number) {
  scale = validateScale(scale)
  const info = await addon.getVideoInfo(file)
  const width = info.displayWidth * scale
  const height = info.displayHeight * scale
  return addon.getScreenshot(file, ts, width, height)
}

// alias to screengen
export const screengen = addon.getScreenshot
export const screengenScale = getScreenshotScale

/**
 * generate preview for video, with scale
 */
export async function getVideoPreviewScale(
  file: string,
  rows: number,
  cols: number,
  scale?: number
) {
  scale = validateScale(scale)
  const info = await addon.getVideoInfo(file)
  const frameWidth = info.displayWidth * scale
  const frameHeight = info.displayHeight * scale
  return addon.getVideoPreview(file, rows, cols, frameWidth, frameHeight)
}
