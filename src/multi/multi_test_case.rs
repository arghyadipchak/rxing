/*
 * Copyright 2016 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{collections::HashSet, path::PathBuf};

use crate::{
    common::HybridBinarizer, BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource,
    MultiFormatReader,
};

use super::{GenericMultipleBarcodeReader, MultipleBarcodeReader};

/**
 * Tests {@link MultipleBarcodeReader}.
 */

#[cfg(feature = "image_formats")]
#[test]
fn testMulti() {
    // Very basic test for now
    let mut testBase = PathBuf::from("test_resources/blackbox/multi-1");

    testBase.push("1.png");
    let image = image::ImageReader::open(testBase)
        .expect("image must open")
        .decode()
        .expect("must decode");
    let source = BufferedImageLuminanceSource::new(image);
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));

    let mut reader = GenericMultipleBarcodeReader::new(MultiFormatReader::default());
    let results = reader
        .decode_multiple(&mut bitmap)
        .expect("must decode multi");
    // assertNotNull(results);
    assert_eq!(2, results.len());

    assert_eq!("031415926531", results[0].getText());
    assert_eq!(&BarcodeFormat::UPC_A, results[0].getBarcodeFormat());

    assert_eq!("www.airtable.com/jobs", results[1].getText());
    assert_eq!(&BarcodeFormat::QR_CODE, results[1].getBarcodeFormat());
}

#[cfg(feature = "image_formats")]
#[test]
fn testMultiQR() {
    // Very basic test for now
    let mut testBase = PathBuf::from("test_resources/blackbox/multi-qrcode-1");

    testBase.push("1.png");
    let image = image::ImageReader::open(testBase)
        .expect("image must open")
        .decode()
        .expect("must decode");
    let source = BufferedImageLuminanceSource::new(image);
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));

    let mut reader = GenericMultipleBarcodeReader::new(MultiFormatReader::default());
    let results = reader
        .decode_multiple(&mut bitmap)
        .expect("must decode multi");
    assert_eq!(4, results.len());

    let mut barcodeContents = HashSet::new();
    for result in results {
        barcodeContents.insert(result.getText().to_owned());
        assert_eq!(&BarcodeFormat::QR_CODE, result.getBarcodeFormat());
        assert!(!result.getRXingResultMetadata().is_empty());
    }
    let mut expectedContents = HashSet::new();
    expectedContents.insert(
        "You earned the class a 5 MINUTE DANCE PARTY!!  Awesome!  Way to go!  Let's boogie!"
            .to_owned(),
    );
    expectedContents.insert(
        "You earned the class 5 EXTRA MINUTES OF RECESS!!  Fabulous!!  Way to go!!".to_owned(),
    );
    expectedContents.insert(
        "You get to SIT AT MRS. SIGMON'S DESK FOR A DAY!!  Awesome!!  Way to go!! Guess I better clean up! :)".to_owned());
    expectedContents
        .insert("You get to CREATE OUR JOURNAL PROMPT FOR THE DAY!  Yay!  Way to go!  ".to_owned());
    assert_eq!(expectedContents, barcodeContents);
}
