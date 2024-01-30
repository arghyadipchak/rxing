/*
 * Copyright 2009 ZXing authors
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

use std::collections::HashMap;

use crate::{
    common::{cpp_essentials::QuadrilateralIsPlausibleSquare, Quadrilateral, Result},
    point_f, Binarizer, BinaryBitmap, DecodingHintDictionary, Exceptions, Point, RXingResult,
    Reader,
};

use super::MultipleBarcodeReader;

/**
 * <p>Attempts to locate multiple barcodes in an image by repeatedly decoding portion of the image.
 * After one barcode is found, the areas left, above, right and below the barcode's
 * {@link Point}s are scanned, recursively.</p>
 *
 * <p>A caller may want to also employ {@link ByQuadrantReader} when attempting to find multiple
 * 2D barcodes, like QR Codes, in an image, where the presence of multiple barcodes might prevent
 * detecting any one of them.</p>
 *
 * <p>That is, instead of passing a {@link Reader} a caller might pass
 * {@code new ByQuadrantReader(reader)}.</p>
 *
 * @author Sean Owen
 */
#[derive(Default)]
pub struct GenericMultipleBarcodeReader<T: Reader>(T);

impl<T: Reader> MultipleBarcodeReader for GenericMultipleBarcodeReader<T> {
    fn decode_multiple<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
    ) -> Result<Vec<RXingResult>> {
        self.decode_multiple_with_hints(image, &HashMap::new())
    }

    fn decode_multiple_with_hints<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodingHintDictionary,
    ) -> Result<Vec<RXingResult>> {
        let mut results = Vec::new();
        self.do_decode_multiple(image, hints, &mut results, 0, 0, 0);

        if results.is_empty() {
            return Err(Exceptions::NOT_FOUND);
        }
        Ok(results)
    }
}
impl<T: Reader> GenericMultipleBarcodeReader<T> {
    const MIN_DIMENSION_TO_RECUR: f32 = 2.0;
    const MAX_DEPTH: u32 = 8;

    pub fn new(delegate: T) -> Self {
        Self(delegate)
    }

    fn do_decode_multiple<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodingHintDictionary,
        results: &mut Vec<RXingResult>,
        xOffset: u32,
        yOffset: u32,
        currentDepth: u32,
    ) {
        if currentDepth > Self::MAX_DEPTH {
            return;
        }

        // let result;
        let Ok(result) = self.0.decode_with_hints(image, hints) else {
            return;
        };

        // let alreadyFound = results.iter().any(|r| r.getText() == result.getText() && r.getBarcodeFormat() == result.getBarcodeFormat());

        let resultPoints = result.getPoints().clone();

        let possible_new_result = Self::translatePoints(result, xOffset, yOffset);

        let already_found = if possible_new_result.getPoints().len() >= 4 {
            let q1 = Quadrilateral::new(
                possible_new_result.getPoints()[0],
                possible_new_result.getPoints()[1],
                possible_new_result.getPoints()[2],
                possible_new_result.getPoints()[3],
            );
            results.iter().any(|e| {
                if e.getPoints().len() >= 4 {
                    let q2 = Quadrilateral::new(
                        e.getPoints()[0],
                        e.getPoints()[1],
                        e.getPoints()[2],
                        e.getPoints()[3],
                    );
                    Quadrilateral::have_intersecting_bounding_boxes(&q1, &q2)
                } else {
                    e.getPoints().iter().any(|p| q1.is_inside(*p))
                }
            })
        } else {
            results.iter().any(|e| {
                if e.getPoints().len() >= 4 {
                    let q2 = Quadrilateral::new(
                        e.getPoints()[0],
                        e.getPoints()[1],
                        e.getPoints()[2],
                        e.getPoints()[3],
                    );
                    e.getPoints().iter().any(|p| q2.is_inside(*p))
                } else {
                    e.getText() == possible_new_result.getText()
                        && e.getBarcodeFormat() == possible_new_result.getBarcodeFormat()
                }
            })
        };

        if !already_found {
            results.push(possible_new_result);
        }

        if resultPoints.is_empty() {
            return;
        }

        let width = image.get_width();
        let height = image.get_height();
        let mut minX: f32 = width as f32;
        let mut minY: f32 = height as f32;
        let mut maxX: f32 = 0.0;
        let mut maxY: f32 = 0.0;
        for point in &resultPoints {
            let x = point.x;
            let y = point.y;

            minX = f32::min(x, minX);
            minY = f32::min(y, minY);
            maxX = f32::max(x, maxX);
            maxY = f32::max(y, maxY);
        }

        // Decode left of barcode
        if minX > Self::MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(
                &mut image.crop(0, 0, minX as usize, height),
                hints,
                results,
                xOffset,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode above barcode
        if minY > Self::MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(
                &mut image.crop(0, 0, width, minY as usize),
                hints,
                results,
                xOffset,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode right of barcode
        if maxX < (width as f32) - Self::MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(
                &mut image.crop(maxX as usize, 0, width - maxX as usize, height),
                hints,
                results,
                xOffset + maxX as u32,
                yOffset,
                currentDepth + 1,
            );
        }
        // Decode below barcode
        if maxY < (height as f32) - Self::MIN_DIMENSION_TO_RECUR {
            self.do_decode_multiple(
                &mut image.crop(0, maxY as usize, width, height - maxY as usize),
                hints,
                results,
                xOffset,
                yOffset + maxY as u32,
                currentDepth + 1,
            );
        }
    }

    fn translatePoints(result: RXingResult, xOffset: u32, yOffset: u32) -> RXingResult {
        let oldPoints = result.getPoints();
        if oldPoints.is_empty() {
            return result;
        }

        let newPoints: Vec<Point> = oldPoints
            .iter()
            .map(|oldPoint| point_f(oldPoint.x + xOffset as f32, oldPoint.y + yOffset as f32))
            .collect();

        // let mut newPoints = Vec::with_capacity(oldPoints.len());
        // for oldPoint in oldPoints {
        //     newPoints.push(point(
        //         oldPoint.getX() + xOffset as f32,
        //         oldPoint.getY() + yOffset as f32,
        //     ));
        // }
        let mut newRXingResult = RXingResult::new_complex(
            result.getText(),
            result.getRawBytes().clone(),
            result.getNumBits(),
            newPoints,
            *result.getBarcodeFormat(),
            result.getTimestamp(),
        );
        newRXingResult.putAllMetadata(result.getRXingResultMetadata().clone());

        newRXingResult
    }
}
