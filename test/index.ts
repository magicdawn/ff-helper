import 'should'
import {
  configuration,
  getVideoDuration,
  getVideoDurationDisplay,
  getVideoDurationDisplaySync,
  getVideoDurationSync,
  getVideoRotation,
  getVideoRotationSync,
} from '../ts-src'

import { duration, durationDisplay, file, fileRotated } from './setup'

// TODO: add a sample video file
// const file = '/Users/magicdawn/Movies/曼达洛人.The.Mandalorian.S03E06.1080p.H265-NEW字幕组.mp4'

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
})
