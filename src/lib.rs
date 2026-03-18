#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod common;
mod exceptions;

#[cfg(feature = "client_support")]
pub mod client;

use std::{collections::HashMap, sync::Arc};

pub use exceptions::Exceptions;

#[cfg(feature = "image")]
mod buffered_image_luminance_source;

#[cfg(feature = "image")]
pub use buffered_image_luminance_source::*;

#[cfg(test)]
mod PlanarYUVLuminanceSourceTestCase;

#[cfg(test)]
mod rgb_luminance_source_test_case;

pub type MetadataDictionary = HashMap<RXingResultMetadataType, RXingResultMetadataValue>;

mod barcode_format;
pub use barcode_format::*;

/// Callback which is invoked when a possible result point (significant
/// point in the barcode image such as a corner) is found.
pub type PointCallback = Arc<dyn Fn(Point) + Send + Sync>;

/** Temporary type to ease refactoring and keep backwards-compatibility */
pub type RXingResultPointCallback = PointCallback;


mod dimension;
pub use dimension::*;

// The core encoding and decoding machinery lives in these submodules.
#[cfg(feature = "aztec")]
pub mod aztec;
#[cfg(feature = "maxicode")]
pub mod maxicode;
#[cfg(feature = "qrcode")]
pub mod qrcode;
#[cfg(feature = "datamatrix")]
pub mod datamatrix;
#[cfg(feature = "oned")]
pub mod oned;
#[cfg(feature = "pdf417")]
pub mod pdf417;

#[cfg(feature = "multi_barcode_readers")]
pub mod multi;

// Simple methods to help detect barcodes in common situations
pub mod helpers;

#[cfg(feature = "svg_read")]
mod svg_luminance_source;
#[cfg(feature = "svg_read")]
pub use svg_luminance_source::*;


// Reading
#[cfg(feature = "decoders")]
mod decode_hints;
#[cfg(feature = "decoders")]
pub use decode_hints::*;

#[cfg(feature = "decoders")]
mod multi_use_multi_format_reader;
#[cfg(feature = "decoders")]
pub use multi_use_multi_format_reader::*;

#[cfg(feature = "decoders")]
mod multi_format_reader;
#[cfg(feature = "decoders")]
pub use multi_format_reader::*;

#[cfg(feature = "decoders")]
mod reader;
#[cfg(feature = "decoders")]
pub use reader::*;

#[cfg(feature = "decoders")]
mod rxing_result_metadata;
#[cfg(feature = "decoders")]
pub use rxing_result_metadata::*;

#[cfg(feature = "decoders")]
mod rxing_result;
#[cfg(feature = "decoders")]
pub use rxing_result::*;

#[cfg(feature = "decoders")]
mod result_point;
#[cfg(feature = "decoders")]
pub use result_point::*;

#[cfg(feature = "decoders")]
pub mod result_point_utils;

#[cfg(feature = "decoders")]
mod rxing_result_point;
#[cfg(feature = "decoders")]
pub use rxing_result_point::*;

#[cfg(feature = "decoders")]
pub type DecodingHintDictionary = HashMap<DecodeHintType, DecodeHintValue>;

// Reading Sources
#[cfg(feature = "decoders")]
mod binarizer;
#[cfg(feature = "decoders")]
pub use binarizer::*;

#[cfg(feature = "decoders")]
mod binary_bitmap;
#[cfg(feature = "decoders")]
pub use binary_bitmap::*;

#[cfg(feature = "decoders")]
mod luminance_source;
#[cfg(feature = "decoders")]
pub use luminance_source::*;

#[cfg(feature = "decoders")]
mod planar_yuv_luminance_source;
#[cfg(feature = "decoders")]
pub use planar_yuv_luminance_source::*;

#[cfg(feature = "decoders")]
mod rgb_luminance_source;
#[cfg(feature = "decoders")]
pub use rgb_luminance_source::*;

#[cfg(feature = "decoders")]
mod luma_luma_source;
#[cfg(feature = "decoders")]
pub use luma_luma_source::*;

// Writing
#[cfg(feature = "encoders")]
mod encode_hints;
#[cfg(feature = "encoders")]
pub use encode_hints::*;

#[cfg(feature = "encoders")]
pub type EncodingHintDictionary = HashMap<EncodeHintType, EncodeHintValue>;

#[cfg(feature = "encoders")]
mod filtered_image_reader;
#[cfg(feature = "encoders")]
pub use filtered_image_reader::*;

#[cfg(feature = "encoders")]
mod multi_format_writer;
#[cfg(feature = "encoders")]
pub use multi_format_writer::*;

#[cfg(feature = "encoders")]
mod writer;
#[cfg(feature = "encoders")]
pub use writer::*;