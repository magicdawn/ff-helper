import 'should'
import {
  VideoInfo,
  configuration,
  getVideoDuration,
  getVideoDurationDisplay,
  getVideoDurationDisplaySync,
  getVideoDurationSync,
  getVideoInfo,
  getVideoRotation,
  getVideoRotationSync,
} from '../ts-src'

import should from 'should'
import { duration, durationDisplay, file, fileRotated } from './setup'

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
      duration: 13000,
      rotation: 0,
      width: 3840,
      height: 2160,
    } satisfies VideoInfo)

    // 有 rotate 时, width/height 不变, 需要 user 转换
    ;(await getVideoInfo(fileRotated)).should.deepEqual({
      duration: 13000,
      rotation: 270,
      width: 3840,
      height: 2160,
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
