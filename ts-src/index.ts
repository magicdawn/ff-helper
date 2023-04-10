export * from '../addon'
import * as addon from '../addon'

import dayjs from 'dayjs'
import duration from 'dayjs/plugin/duration'
dayjs.extend(duration)

export function getVideoDurationDisplay(file: string) {
  const duration = addon.getVideoDuration(file)
  const display = dayjs.duration(duration, 'milliseconds').format('HH:mm:ss')
  return display
}
