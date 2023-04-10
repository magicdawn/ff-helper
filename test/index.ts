import 'should'
import {
  configuration,
  getVideoDuration,
  getVideoDurationDisplay,
  getVideoRotation,
} from '../ts-src'

// TODO: add a sample video file
const file = '/Users/magicdawn/Movies/曼达洛人.The.Mandalorian.S03E06.1080p.H265-NEW字幕组.mp4'

describe('ff-helper', () => {
  it('.configuration', () => {
    configuration().includes('--').should.ok()
  })

  it('.getVideoDuration', () => {
    getVideoDuration(file).should.eql(2654000)
  })

  it('.getVideoDurationDisplay', () => {
    getVideoDurationDisplay(file).should.eql('00:44:14')
  })

  it('.getVideoRotation', () => {
    getVideoRotation(file).should.eql(0)
  })
})
