use blaze_rs::texture::*;

const MINIMAL_PNG: [u8; 67] = [
    0x89u8, 0x50u8, 0x4eu8, 0x47u8, 0x0du8, 0x0au8, 0x1au8, 0x0au8, 0x00u8, 0x00u8, 0x00u8, 0x0du8,
    0x49u8, 0x48u8, 0x44u8, 0x52u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8,
    0x08u8, 0x06u8, 0x00u8, 0x00u8, 0x00u8, 0x1fu8, 0x15u8, 0xc4u8, 0x89u8, 0x00u8, 0x00u8, 0x00u8,
    0x0au8, 0x49u8, 0x44u8, 0x41u8, 0x54u8, 0x78u8, 0x9cu8, 0x63u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8,
    0x05u8, 0x00u8, 0x01u8, 0x0du8, 0x0au8, 0x2du8, 0xb4u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x49u8,
    0x45u8, 0x4eu8, 0x44u8, 0xaeu8, 0x42u8, 0x60u8, 0x82u8,
];

#[cfg(test)]
pub fn test_png_loading() {
    assert!(Texture::from_file(
        "tests/pnggrad8rgb.png",
        ImageChannels::Auto,
        None,
        ImageFlags::None
    )
    .is_ok());
    assert!(
        Texture::from_file("does/not.exist", ImageChannels::Auto, None, ImageFlags::None).is_err()
    );
    let png_bytes = bytes::Bytes::from(&MINIMAL_PNG as &[u8]);
    assert!(Texture::from_memory(&png_bytes, ImageChannels::Auto, None, ImageFlags::None).is_ok());
    assert!(Texture::from_memory(
        &png_bytes.slice_to(1),
        ImageChannels::Auto,
        None,
        ImageFlags::None
    )
    .is_err());
}
