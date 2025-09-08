// not working, use
// export RUST_LOG='ff_helper=debug'
import fs from 'node:fs'
import path from 'node:path'
import { configuration, getVideoInfoSync, license, screengen, version, versionInfo } from '../ts-src'

process.env.RUST_LOG = 'ff_helper=debug'

const file = path.join(__dirname, '../test/sample-videos/sample-rotated-90.mp4')

void (async () => {
  console.log(configuration())
  console.log(version())
  console.log(versionInfo())
  console.log(license())

  const info = getVideoInfoSync(file)
  console.log(info)

  const bufferSnapshot = await screengen(file, 1000)
  fs.writeFileSync(path.join(__dirname, '../test/sample-videos/sample-rotated-90-1s.jpg'), bufferSnapshot)
})()
