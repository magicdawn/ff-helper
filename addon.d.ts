/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface VideoInfo {
  /** degress, 0-360, counterclockwise  */
  rotation: number
  /** millseconds  */
  duration: number
  width: number
  height: number
}
export function configuration(): string
export function version(): number
export function license(): string
export function versionInfo(): string
/**
 * get video duration sync, return number as ms
 */
export function getVideoDurationSync(file: string): number
/**
 * get video duration, return number as ms
 */
export function getVideoDuration(
  file: string,
  signal?: AbortSignal | undefined | null
): Promise<number>
/**
 * get video rotation sync, in degrees (0-360), counterclockwise
 */
export function getVideoRotationSync(file: string): number
/**
 * get video rotation, in degrees (0-360), counterclockwise
 */
export function getVideoRotation(
  file: string,
  signal?: AbortSignal | undefined | null
): Promise<number>
export function getMetadata(file: string): void
export function getVideoInfo(
  file: string,
  signal?: AbortSignal | undefined | null
): Promise<VideoInfo>