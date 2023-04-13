import fs from 'fs'
import path from 'path'
import sharp from 'sharp'
import { getScreenshotAtSync, getVideoInfoSync } from '../addon'

const file = path.join(__dirname, '../test/sample-videos/sample-rotated-90.mp4')

void (async () => {
  // console.log(configuration())
  // console.log(version())
  // console.log(versionInfo())
  // console.log(license())

  // try {
  // } catch (e) {
  //   console.error('error:')
  //   console.error('typeof e = ', typeof e)
  //   console.error('typeof e.stack = ', typeof e.stack)
  //   console.log('e.message = %s', e.message)
  //   console.error(e.stack || e)
  // }

  const info = getVideoInfoSync(file)
  // console.log(info)
  const b = getScreenshotAtSync(file, 8 * 1000)
  // console.log(b.byteLength)
  const jpegBuf = await sharp(b, {
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
  fs.writeFileSync(path.join(__dirname, '../test/sample-videos/sample-rotated-90-1s.jpg'), jpegBuf)
})()
