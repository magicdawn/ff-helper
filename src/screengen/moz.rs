use crate::helper;
use image::RgbaImage;
use mozjpeg;
use mozjpeg::ColorSpace;

#[derive(Clone, Copy, Debug)]
pub struct MozConfig {
  quality: u32,
  color_space: ColorSpace,
}

pub fn mozjpeg_encode(img: &RgbaImage, config: Option<MozConfig>) -> napi::Result<Vec<u8>> {
  helper::napi_catch_unwind(|| {
    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);

    let (width, height) = (img.width() as usize, img.height() as usize);
    comp.set_size(width, height);
    comp.set_mem_dest();

    /**
     * options
     */
    let config = config.unwrap_or_else(|| MozConfig {
      quality: 85,
      color_space: ColorSpace::JCS_YCbCr,
    });

    comp.set_quality(config.quality as f32);
    comp.set_progressive_mode();
    comp.set_color_space(config.color_space);

    comp.start_compress();
    // replace with your image data
    // let pixels = img.as_raw().as_slice();
    let pixels = img.as_ref();
    assert!(comp.write_scanlines(pixels));

    comp.finish_compress();
    let jpeg_bytes = comp
      .data_to_vec()
      .map_err(|_| napi::Error::from_reason("mozjpeg data_to_vec errored"))?;
    Ok(jpeg_bytes)
  })?
}
