#[derive(Default)]
pub struct RGBA8Buffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

pub fn decode(bytes: &[u8]) -> Option<RGBA8Buffer> {
    if bytes.len() >= 2 && bytes[0..2] == [0xFF, 0xD8] {
        use zune_core::colorspace::ColorSpace;
        use zune_core::options::DecoderOptions;
        use zune_jpeg::JpegDecoder;

        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
        let mut decoder = JpegDecoder::new_with_options(options, bytes);
        decoder.decode_headers().ok()?;
        let info = decoder.info()?;
        let pixels = decoder.decode().ok()?;
        Some(RGBA8Buffer {
            width: info.width as _,
            height: info.height as _,
            data: pixels,
        })
    } else if bytes.len() >= 8 && bytes[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        use zune_core::options::DecoderOptions;
        use zune_png::PngDecoder;
        let options = DecoderOptions::default().png_set_add_alpha_channel(true);
        let mut decoder = PngDecoder::new_with_options(bytes, options);

        decoder.decode_headers().ok()?;
        let (width, height) = decoder.get_dimensions()?;

        let pixels = decoder.decode_raw().ok()?;
        assert!(pixels.len() == width * height * 4); // and deal with u16 png later
        Some(RGBA8Buffer {
            width: width as _,
            height: height as _,
            data: pixels,
        })
    } else {
        None
    }
}
