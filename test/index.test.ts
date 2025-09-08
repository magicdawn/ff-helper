import { writeFile } from 'node:fs/promises'
import fse from 'fs-extra'
import sharp from 'sharp'
import { describe, expect, it } from 'vitest'
import {
  configuration,
  getVideoDuration,
  getVideoDurationDisplay,
  getVideoDurationDisplaySync,
  getVideoDurationSync,
  getVideoInfo,
  getVideoInfoSync,
  getVideoPreview,
  getVideoPreviewScale,
  getVideoRotation,
  getVideoRotationSync,
  screengen,
  screengenScale,
  type VideoInfo,
} from '../ts-src'
import { duration, durationDisplay, file, fileRotated } from './setup'

async function checkImg(file: string, width: number, height: number) {
  const metadata = await sharp(file).metadata()
  expect(metadata.width!).toBe(width)
  expect(metadata.height!).toBe(height)
}

describe('ff-helper', () => {
  it('.configuration', () => {
    expect(configuration().includes('--')).toBe(true)
  })

  it('.getVideoDurationSync', () => {
    expect(Math.abs(getVideoDurationSync(file) - duration)).toBeLessThanOrEqual(1000)
  })
  it('.getVideoDuration', async () => {
    expect(Math.abs((await getVideoDuration(file)) - duration)).toBeLessThanOrEqual(1000)
  })

  it('.getVideoDurationDisplaySync', () => {
    expect(getVideoDurationDisplaySync(file)).toEqual(durationDisplay)
  })
  it('.getVideoDurationDisplay', async () => {
    expect(await getVideoDurationDisplay(file)).toEqual(durationDisplay)
  })

  it('.getVideoRotationSync', () => {
    expect(getVideoRotationSync(file)).toBe(0)
    expect(getVideoRotationSync(fileRotated)).toBe(270)
  })
  it('.getVideoRotation', async () => {
    expect(await getVideoRotation(file)).toBe(0)
    expect(await getVideoRotation(fileRotated)).toBe(270)
  })

  it('.getVideoInfo', async () => {
    expect(await getVideoInfo(file)).toEqual({
      duration,
      rotation: 0,
      width: 3840,
      height: 2160,
      shouldSwap: false,
      displayWidth: 3840,
      displayHeight: 2160,
    } satisfies VideoInfo)
  })
  it('.getVideoInfoSync', () => {
    // 有 rotate 时, width/height 不变, 需要 user 转换
    expect(getVideoInfoSync(fileRotated)).toEqual({
      duration,
      rotation: 270,
      width: 3840,
      height: 2160,
      shouldSwap: true,
      displayWidth: 2160,
      displayHeight: 3840,
    } satisfies VideoInfo)
  })
})

describe('error tolerant', () => {
  it('no panic exit', async () => {
    try {
      await getVideoInfo(__filename)
    } catch (e: any) {
      expect(e).toBeInstanceOf(Error)
      expect(e.message).toContain('ffmpeg')
      expect(e.message).toContain('Invalid data found when processing input')
      expect(e.code).toBe('GenericFailure')
    }
  })
})

describe('screengen', () => {
  it('.screengen', async () => {
    {
      const info = await getVideoInfo(file)
      const imgBuf = await screengenScale(file, 1000, 0.5)
      const img = __dirname + '/fixtures/file-screenshot-1000ms-x0.5.jpg'
      await fse.writeFile(img, imgBuf)
      await checkImg(img, Math.trunc(info.width * 0.5), Math.trunc(info.height * 0.5))
    }

    // float will be truncated by napi-rs
    {
      const imgBuf = await screengen(file, 1000, 200.85, 100.45)
      const img = __dirname + '/fixtures/file-getScreenshotJpeg-float-1000ms-200x100.jpg'
      await fse.writeFile(img, imgBuf)
      await checkImg(img, 200, 100)
    }

    // rotated
    {
      const info = await getVideoInfo(fileRotated)
      const imgBuf = await screengenScale(fileRotated, 1000, 0.5)
      const img = __dirname + '/fixtures/fileRotated-screenshot-1000ms-x0.5.jpg'
      await fse.writeFile(img, imgBuf)
      await checkImg(img, info.displayWidth * 0.5, info.displayHeight * 0.5)
    }
  })
})

describe('video-preview', () => {
  it('.getVideoPreviewScale', async () => {
    const buf = await getVideoPreviewScale(file, 4, 4, 0.6)
    await writeFile(__dirname + '/fixtures/video-preview-scalex0.6-4x4.jpg', buf)
  })

  it('.getVideoPreview', async () => {
    const { displayWidth, displayHeight } = await getVideoInfo(file)
    const buf = await getVideoPreview(file, 4, 4, displayWidth * 0.5, displayHeight * 0.5)
    await writeFile(__dirname + '/fixtures/video-preview-mozjpeg-scalex0.6-4x4.jpg', buf)
  })
})
