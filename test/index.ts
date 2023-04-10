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

// TODO: add a sample video file
const file = '/Users/magicdawn/Movies/曼达洛人.The.Mandalorian.S03E06.1080p.H265-NEW字幕组.mp4'

describe('ff-helper', () => {
  it('.configuration', () => {
    configuration().includes('--').should.ok()
  })

  it('.getVideoDurationSync', () => {
    getVideoDurationSync(file).should.eql(2654000)
  })
  it('.getVideoDuration', async () => {
    ;(await getVideoDuration(file)).should.eql(2654000)
  })

  it('.getVideoDurationDisplaySync', async () => {
    getVideoDurationDisplaySync(file).should.eql('00:44:14')
  })
  it('.getVideoDurationDisplay', async () => {
    ;(await getVideoDurationDisplay(file)).should.eql('00:44:14')
  })

  it('.getVideoRotationSync', () => {
    getVideoRotationSync(file).should.eql(0)
  })
  it('.getVideoRotation', async () => {
    ;(await getVideoRotation(file)).should.eql(0)
  })
})
