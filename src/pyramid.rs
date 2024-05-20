//! # Image Pyramid Module
//!
//! This module re-exports the `image-pyramid` crate, with permission from the author.
//! 
//! ```rust
//! use image::DynamicImage;
//! use imageproc::pyramid::ImagePyramid;
//!
//! let image = DynamicImage::new_rgba8(640, 480); // Or load from file
//! let pyramid = ImagePyramid::create(&image, None).unwrap();
//! ```

use image::DynamicImage;

pub use image_pyramid::{
    ImagePyramid,
    ImagePyramidError,
    ImagePyramidType,
    SmoothingType,
    ImagePyramidParams,
    UnitIntervalValue,
    IntoUnitInterval
};

/// Trait for computing an image pyramid.
/// 
/// Use this trait to compute an image pyramid for any [`image::DynamicImage`].
pub trait ComputeImagePyramid {
    /// Computes an image pyramid, returning it as a [`ImagePyramid`].
    /// 
    /// See the [`image_pyramid`] crate for more information.
    fn compute_image_pyramid(&self, params: Option<&ImagePyramidParams>) -> Result<ImagePyramid, ImagePyramidError>;
}

impl ComputeImagePyramid for DynamicImage {
    fn compute_image_pyramid(&self, params: Option<&ImagePyramidParams>) -> Result<ImagePyramid, ImagePyramidError> {
        ImagePyramid::create(self, params)
    }
}