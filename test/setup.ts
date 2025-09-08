import { execSync } from 'node:child_process'
import { existsSync } from 'node:fs'
import { beforeAll } from 'vitest'

export const file = import.meta.dirname + '/fixtures/sample-videos/sample.mp4'
export const durationDisplay = '00:00:13'
export const duration = 13960 // ms
export const fileRotated = import.meta.dirname + '/fixtures/sample-videos/sample-rotated-90.mp4'

const url = 'https://download.pexels.com/vimeo/371817283/pexels-pressmaster-3195394.mp4?width=3840'

beforeAll(() => {
  // HEAD not allowed
  // 2024-04-14 不能下载了
  // if (!existsSync(file)) {
  //   await dl({ url, file })
  // }

  // 90 clockwise
  // ffmpeg -y -i 'sample.mp4' -c copy -metadata:s:v:0 rotate=270 'sample-rotated-90.mp4'
  if (!existsSync(fileRotated)) {
    execSync(`ffmpeg -y -i '${file}' -c copy -metadata:s:v:0 rotate=270 '${fileRotated}'`)
  }
})
