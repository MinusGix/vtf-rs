use crate::header::VTFHeader;
use crate::image::ImageFormat;
use crate::Error;

pub fn get_offset(
    header: &VTFHeader,
    image_format: &ImageFormat,
    frame: u32,
    face: u32,
    slice: u32,
    mip_level: i32,
) -> Result<u32, Error> {
    let mut offset: u32 = 0;

    for i in (mip_level + 1..(header.mipmap_count) as i32).rev() {
        offset += get_mip_size(header, image_format, i as u32, header.depth)?;
    }

    offset *= header.frames as u32;

    let volume_bytes: u32 = get_mip_size(header, image_format, mip_level as u32, header.depth)?;
    let slice_bytes: u32 = get_mip_size(header, image_format, mip_level as u32, 1)?;

    offset += volume_bytes * (frame + face);
    offset += slice_bytes * slice;

    Ok(offset)
}

pub fn get_mip_size(
    header: &VTFHeader,
    image_format: &ImageFormat,
    mip_level: u32,
    depth: u16,
) -> Result<u32, Error> {
    let mut mip_width = header.width.wrapping_shr(mip_level);
    let mut mip_height = header.height.wrapping_shr(mip_level);
    let mut mip_depth = depth.wrapping_shr(mip_level);

    if mip_width < 1 {
        mip_width = 1;
    }

    if mip_height < 1 {
        mip_height = 1;
    }
    if mip_depth < 1 {
        mip_depth = 1;
    }

    Ok(image_format.frame_size(mip_width as u32, mip_height as u32)? * mip_depth as u32)
}

// We could have an in-place version but there's not much benefit since it would need to be
// converted to a vec anyway, at least with the current code-base
pub fn bgra_to_rgba(bgra: &[u8]) -> Vec<u8> {
    let mut dest = vec![0; bgra.len()];
    for (src, dst) in bgra.chunks_exact(4).zip(dest.chunks_exact_mut(4)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
        dst[3] = src[3];
    }

    dest
}

pub fn bgr_to_rgb(bgr: &[u8]) -> Vec<u8> {
    let mut dest = vec![0; bgr.len()];
    for (src, dst) in bgr.chunks_exact(3).zip(dest.chunks_exact_mut(3)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
    }

    dest
}
