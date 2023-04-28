/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface VideoInfo {
  /** degress, 0-360, counterclockwise  */
  rotation: number
  /** check if rotation = 90 | 270  */
  shouldSwap: boolean
  /** millseconds  */
  duration: number
  /** raw width, before apply rotation   */
  width: number
  /** raw height, before apply rotation   */
  height: number
  /** display width, after apply rotation   */
  displayWidth: number
  /** display height, after apply rotation   */
  displayHeight: number
}
/**
 * synchronous get screenshot at [ts] for [file], optional [width] & [height] fallback to video width & height
*/
export function getScreenshotAtSync(file: string, ts: number, width?: number | undefined | null, height?: number | undefined | null): Buffer
/**
 * get screenshot at [ts] for [file], optional [width] & [height] fallback to video width & height
*/
export function getScreenshotAt(file: string, ts: number, width?: number | undefined | null, height?: number | undefined | null, signal?: AbortSignal | undefined | null): Promise<Buffer>
export function videoPreview(file: string, rows: number, cols: number, frameWidth: number, frameHeight: number): Buffer
/**
 * Return the libavutil build-time configuration.
*/
export function configuration(): string
/**
 * Return the LIBAVUTIL_VERSION_INT constant.
*/
export function version(): number
/**
 * Return the libavutil license.
*/
export function license(): string
/**
 * Return an informative version string.
 * This usually is the actual release version number or a git commit description.
 * This string has no fixed format and can change any time.
 * It should never be parsed by code.
*/
export function versionInfo(): string
/**
 * get video duration synchronous, return number as ms
*/
export function getVideoDurationSync(file: string): number
/**
 * get video duration, return number as ms
*/
export function getVideoDuration(file: string, signal?: AbortSignal | undefined | null): Promise<number>
/**
 * get video rotation synchronous, in degrees (0-360), counterclockwise
*/
export function getVideoRotationSync(file: string): number
/**
 * get video rotation, in degrees (0-360), counterclockwise
*/
export function getVideoRotation(file: string, signal?: AbortSignal | undefined | null): Promise<number>
export function getMetadata(file: string): void
/**
 * get video information synchronous. (width, height, duration, rotation etc)
*/
export function getVideoInfoSync(file: string): VideoInfo
/**
 * get video information. (width, height, duration, rotation etc)
*/
export function getVideoInfo(file: string, signal?: AbortSignal | undefined | null): Promise<VideoInfo>
