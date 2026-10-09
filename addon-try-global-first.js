/**
 * a proxy layer on addon.js, with try globalThis first
 */

import { createRequire } from 'node:module'
const require = createRequire(import.meta.url)
const nativeBinding = globalThis[Symbol.for('ff-helper.node')] || require('./addon.js')

// below are copied from ../addon.js
// After adding new methods, please update here
const {
  configuration,
  getMetadata,
  getScreenshot,
  getScreenshotRaw,
  getVideoDuration,
  getVideoDurationSync,
  getVideoInfo,
  getVideoInfoSync,
  getVideoPreview,
  getVideoPreviewRaw,
  getVideoRotation,
  getVideoRotationSync,
  license,
  version,
  versionInfo,
} = nativeBinding
export { configuration }
export { getMetadata }
export { getScreenshot }
export { getScreenshotRaw }
export { getVideoDuration }
export { getVideoDurationSync }
export { getVideoInfo }
export { getVideoInfoSync }
export { getVideoPreview }
export { getVideoPreviewRaw }
export { getVideoRotation }
export { getVideoRotationSync }
export { license }
export { version }
export { versionInfo }
