import fse from 'fs-extra'
import { writeFile } from 'fs/promises'
import sharp from 'sharp'
import 'should'
import should from 'should'
import {
  VideoInfo,
  configuration,
  getVideoDuration,
  getVideoDurationDisplay,
  getVideoDurationDisplaySync,
  getVideoDurationSync,
  getVideoInfo,
  getVideoInfoSync,
  getVideoPreviewJpeg,
  getVideoPreviewScale,
  getVideoRotation,
  getVideoRotationSync,
  screengen,
  screengenScale,
} from '../ts-src'
import { duration, durationDisplay, file, fileRotated } from './setup'

async function checkImg(file: string, width: number, height: number) {
  const metadata = await sharp(file).metadata()
  metadata.width!.should.equal(width)
  metadata.height!.should.equal(height)
}

describe('ff-helper', () => {
  it('.configuration', () => {
    configuration().includes('--').should.ok()
  })

  it('.getVideoDurationSync', () => {
    getVideoDurationSync(file).should.approximately(duration, 1000)
  })
  it('.getVideoDuration', async () => {
    ;(await getVideoDuration(file)).should.approximately(duration, 1000)
  })

  it('.getVideoDurationDisplaySync', async () => {
    getVideoDurationDisplaySync(file).should.eql(durationDisplay)
  })
  it('.getVideoDurationDisplay', async () => {
    ;(await getVideoDurationDisplay(file)).should.eql(durationDisplay)
  })

  it('.getVideoRotationSync', () => {
    getVideoRotationSync(file).should.eql(0)
    getVideoRotationSync(fileRotated).should.eql(270)
  })
  it('.getVideoRotation', async () => {
    ;(await getVideoRotation(file)).should.eql(0)
    ;(await getVideoRotation(fileRotated)).should.eql(270)
  })

  it('.getVideoInfo', async () => {
    ;(await getVideoInfo(file)).should.deepEqual({
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
    getVideoInfoSync(fileRotated).should.deepEqual({
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
    } catch (e) {
      should.ok(e instanceof Error)
      e.message.includes('ffmpeg').should.ok()
      e.message.includes('Invalid data found when processing input').should.ok()
      e.code.should.equal('GenericFailure')
    }
  })
})

describe('screengen', () => {
  it('.screengen', async () => {
    {
      const info = await getVideoInfo(file)
      const imgBuf = await screengenScale(file, 1000, 0.5)
      await fse.writeFile(__dirname + '/sample-videos/file-screenshot-1000ms-x0.5.jpg', imgBuf)
      await checkImg(
        __dirname + '/sample-videos/file-screenshot-1000ms-x0.5.jpg',
        Math.trunc(info.width * 0.5),
        Math.trunc(info.height * 0.5)
      )
    }

    // float
    {
      const imgBuf = await screengen(file, 1000, 200.45, 100.45)
      await fse.writeFile(__dirname + '/sample-videos/file-screenshot-1000ms-200x100.jpg', imgBuf)
      await checkImg(__dirname + '/sample-videos/file-screenshot-1000ms-200x100.jpg', 200, 100)
    }

    // rotated
    {
      const info = await getVideoInfo(fileRotated)
      const imgBuf = await screengenScale(fileRotated, 1000, 0.5)
      const imgFile = __dirname + '/sample-videos/fileRotated-screenshot-1000ms-x0.5.jpg'
      await fse.writeFile(imgFile, imgBuf)
      await checkImg(imgFile, info.displayWidth * 0.5, info.displayHeight * 0.5)
    }
  })
})

describe('video-preview', () => {
  it('.getVideoPreviewScale', async () => {
    const buf = await getVideoPreviewScale(file, 4, 4, 0.6)
    writeFile(__dirname + '/sample-videos/video-preview-scalex0.6-4x4.jpg', buf)
  })

  it('.getVideoPreviewJpeg', async () => {
    const { displayWidth, displayHeight } = await getVideoInfo(file)
    const buf = await getVideoPreviewJpeg(file, 4, 4, displayWidth * 0.5, displayHeight * 0.5)
    writeFile(__dirname + '/sample-videos/video-preview-mozjpeg-scalex0.6-4x4.jpg', buf)
  })
})
