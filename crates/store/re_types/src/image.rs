//! Image-related utilities.

use smallvec::{smallvec, SmallVec};

use crate::{
    datatypes::ChannelDatatype,
    datatypes::{Blob, TensorBuffer, TensorData, TensorDimension},
};

// ----------------------------------------------------------------------------

/// The kind of image data, either color, segmentation, or depth image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageKind {
    /// A normal grayscale or color image ([`crate::archetypes::Image`]).
    Color,

    /// A depth map ([`crate::archetypes::DepthImage`]).
    Depth,

    /// A segmentation image ([`crate::archetypes::SegmentationImage`]).
    ///
    /// The data is a [`crate::components::ClassId`] which should be
    /// looked up using the appropriate [`crate::components::AnnotationContext`]
    Segmentation,
}

// ----------------------------------------------------------------------------

/// Errors when converting images from the [`image`] crate to an [`crate::archetypes::Image`].
#[cfg(feature = "image")]
#[derive(thiserror::Error, Clone, Debug)]
pub enum ImageConversionError {
    /// Unknown color type from the image crate.
    ///
    /// This should only happen if you are using a newer `image` crate than the one Rerun was built for,
    /// because `image` can add new color types without it being a breaking change,
    /// so we cannot exhaustively match on all color types.
    #[error("Unsupported color type: {0:?}. We support 8-bit, 16-bit, and f32 images, and RGB, RGBA, Luminance, and Luminance-Alpha.")]
    UnsupportedImageColorType(image::ColorType),
}

/// Errors when loading image files.
#[cfg(feature = "image")]
#[derive(thiserror::Error, Clone, Debug)]
pub enum ImageLoadError {
    /// e.g. failed to decode a JPEG file.
    #[error(transparent)]
    Image(std::sync::Arc<image::ImageError>),

    /// e.g. failed to find a file on disk.
    #[error("Failed to load file: {0}")]
    ReadError(std::sync::Arc<std::io::Error>),

    /// Failure to convert the loaded image to a [`crate::archetypes::Image`].
    #[error(transparent)]
    ImageConversionError(#[from] ImageConversionError),

    /// The encountered MIME type is not supported for decoding images.
    #[error("MIME type '{0}' is not supported for images")]
    UnsupportedMimeType(String),
}

#[cfg(feature = "image")]
impl From<image::ImageError> for ImageLoadError {
    #[inline]
    fn from(err: image::ImageError) -> Self {
        Self::Image(std::sync::Arc::new(err))
    }
}

#[cfg(feature = "image")]
impl From<std::io::Error> for ImageLoadError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::ReadError(std::sync::Arc::new(err))
    }
}

// ----------------------------------------------------------------------------

/// Error returned when trying to interpret a tensor as an image.
#[derive(thiserror::Error, Clone, Debug)]
pub enum ImageConstructionError<T: TryInto<TensorData>>
where
    T::Error: std::error::Error,
{
    /// Could not convert source to [`TensorData`].
    #[error("Could not convert source to TensorData: {0}")]
    TensorDataConversion(T::Error),

    /// The tensor did not have the right shape for an image (e.g. had too many dimensions).
    #[error("Could not create Image from TensorData with shape {0:?}")]
    BadImageShape(Vec<TensorDimension>),

    /// Happens if you try to cast `NV12` or `YUY2` to a depth image or segmentation image.
    #[error("Chroma downsampling is not supported for this image type (e.g. DepthImage or SegmentationImage)")]
    ChromaDownsamplingNotSupported,
}

/// Converts it to what is useful for the image API.
pub fn blob_and_datatype_from_tensor(tensor_buffer: TensorBuffer) -> (Blob, ChannelDatatype) {
    match tensor_buffer {
        TensorBuffer::U8(buffer) => (Blob(buffer), ChannelDatatype::U8),
        TensorBuffer::U16(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::U16),
        TensorBuffer::U32(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::U32),
        TensorBuffer::U64(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::U64),
        TensorBuffer::I8(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::I8),
        TensorBuffer::I16(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::I16),
        TensorBuffer::I32(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::I32),
        TensorBuffer::I64(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::I64),
        TensorBuffer::F16(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::F16),
        TensorBuffer::F32(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::F32),
        TensorBuffer::F64(buffer) => (Blob(buffer.cast_to_u8()), ChannelDatatype::F64),
    }
}

// ----------------------------------------------------------------------------

/// Types that implement this can be used as image channel types.
///
/// Implemented for `u8, u16, u32, u64, i8, i16, i32, i64, f16, f32, f64`.
pub trait ImageChannelType: bytemuck::Pod {
    /// The [`ChannelDatatype`] for this type.
    const CHANNEL_TYPE: ChannelDatatype;
}

impl ImageChannelType for u8 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::U8;
}

impl ImageChannelType for u16 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::U16;
}

impl ImageChannelType for u32 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::U32;
}

impl ImageChannelType for u64 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::U64;
}

impl ImageChannelType for i8 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::I8;
}

impl ImageChannelType for i16 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::I16;
}

impl ImageChannelType for i32 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::I32;
}

impl ImageChannelType for i64 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::I64;
}

impl ImageChannelType for half::f16 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::F16;
}

impl ImageChannelType for f32 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::F32;
}

impl ImageChannelType for f64 {
    const CHANNEL_TYPE: ChannelDatatype = ChannelDatatype::F64;
}

// ----------------------------------------------------------------------------

/// Returns the indices of an appropriate set of dimensions.
///
/// Ignores leading and trailing 1-sized dimensions.
///
/// For instance: `[1, 480, 640, 3, 1]` would return `[1, 2, 3]`,
/// the indices of the `[480, 640, 3]` dimensions.
pub fn find_non_empty_dim_indices(shape: &[TensorDimension]) -> SmallVec<[usize; 4]> {
    match shape.len() {
        0 => return smallvec![],
        1 => return smallvec![0],
        2 => return smallvec![0, 1],
        _ => {}
    }

    // Find a range of non-unit dimensions.
    // [1, 1, 1, 480, 640, 3, 1, 1, 1]
    //           ^---------^   goal range

    let mut non_unit_indices =
        shape
            .iter()
            .enumerate()
            .filter_map(|(ind, dim)| if dim.size != 1 { Some(ind) } else { None });

    // 0 is always a valid index.
    let mut min = non_unit_indices.next().unwrap_or(0);
    let mut max = non_unit_indices.last().unwrap_or(min);

    // Note, these are inclusive ranges.

    // First, empty inner dimensions are more likely to be intentional than empty outer dimensions.
    // Grow to a min-size of 2.
    // (1x1x3x1) -> 3x1 mono rather than 1x1x3 RGB
    while max == min && max + 1 < shape.len() {
        max += 1;
    }

    // Next, consider empty outer dimensions if we still need them.
    // Grow up to 3 if the inner dimension is already 3 or 4 (Color Images)
    // Otherwise, only grow up to 2.
    // (1x1x3) -> 1x1x3 rgb rather than 1x3 mono
    let target_len = match shape[max].size {
        3 | 4 => 3,
        _ => 2,
    };

    while max - min + 1 < target_len && 0 < min {
        min -= 1;
    }

    (min..=max).collect()
}

#[test]
fn test_find_non_empty_dim_indices() {
    fn expect(shape: &[u64], expected: &[usize]) {
        let dim: Vec<_> = shape
            .iter()
            .map(|s| TensorDimension {
                size: *s,
                name: None,
            })
            .collect();
        let got = find_non_empty_dim_indices(&dim);
        assert!(
            got.as_slice() == expected,
            "Input: {shape:?}, got {got:?}, expected {expected:?}"
        );
    }

    expect(&[], &[]);
    expect(&[0], &[0]);
    expect(&[1], &[0]);
    expect(&[100], &[0]);

    expect(&[480, 640], &[0, 1]);
    expect(&[480, 640, 1], &[0, 1]);
    expect(&[480, 640, 1, 1], &[0, 1]);
    expect(&[480, 640, 3], &[0, 1, 2]);
    expect(&[1, 480, 640], &[1, 2]);
    expect(&[1, 480, 640, 3, 1], &[1, 2, 3]);
    expect(&[1, 3, 480, 640, 1], &[1, 2, 3]);
    expect(&[1, 1, 480, 640], &[2, 3]);
    expect(&[1, 1, 480, 640, 1, 1], &[2, 3]);

    expect(&[1, 1, 3], &[0, 1, 2]);
    expect(&[1, 1, 3, 1], &[2, 3]);
}

// ----------------------------------------------------------------------------

/// Returns sRGB from YUV color.
///
/// This conversion mirrors the function of the same name in `crates/viewer/re_renderer/shader/decodings.wgsl`
///
/// Specifying the color standard should be exposed in the future [#3541](https://github.com/rerun-io/rerun/pull/3541)
pub fn rgb_from_yuv(y: u8, u: u8, v: u8) -> [u8; 3] {
    let (y, u, v) = (y as f32, u as f32, v as f32);

    // rescale YUV values
    let y = (y - 16.0) / 219.0;
    let u = (u - 128.0) / 224.0;
    let v = (v - 128.0) / 224.0;

    // BT.601 (aka. SDTV, aka. Rec.601). wiki: https://en.wikipedia.org/wiki/YCbCr#ITU-R_BT.601_conversion
    let r = y + 1.402 * v;
    let g = y - 0.344 * u - 0.714 * v;
    let b = y + 1.772 * u;

    // BT.709 (aka. HDTV, aka. Rec.709). wiki: https://en.wikipedia.org/wiki/YCbCr#ITU-R_BT.709_conversion
    // let r = y + 1.575 * v;
    // let g = y - 0.187 * u - 0.468 * v;
    // let b = y + 1.856 * u;

    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}
