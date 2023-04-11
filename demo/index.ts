import addon, { configuration, license, version, versionInfo } from '../addon'

const file = '/Users/magicdawn/Movies/曼达洛人.The.Mandalorian.S03E06.1080p.H265-NEW字幕组.mp4'

void (async () => {
  console.log(await addon.getVideoInfo(file))

  console.log(configuration())
  console.log(version())
  console.log(versionInfo())
  console.log(license())

  // try {
  // } catch (e) {
  //   console.error('error:')
  //   console.error('typeof e = ', typeof e)
  //   console.error('typeof e.stack = ', typeof e.stack)
  //   console.log('e.message = %s', e.message)
  //   console.error(e.stack || e)
  // }

  console.log(111)
})()
