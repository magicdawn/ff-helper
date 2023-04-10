export * from '../addon'
import * as addon from '../addon'

import dayjs from 'dayjs'
import duration from 'dayjs/plugin/duration'
dayjs.extend(duration)

export function getVideoDurationDisplaySync(file: string) {
  return displayMs(addon.getVideoDurationSync(file))
}
export async function getVideoDurationDisplay(file: string) {
  return displayMs(await addon.getVideoDuration(file))
}

function displayMs(ms: number) {
  return dayjs.duration(ms, 'milliseconds').format('HH:mm:ss')
}
