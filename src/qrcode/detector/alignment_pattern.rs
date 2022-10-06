/*
 * Copyright 2007 ZXing authors
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

//RXingResultPoint

use crate::RXingResultPoint;

/**
 * <p>Encapsulates an alignment pattern, which are the smaller square patterns found in
 * all but the simplest QR Codes.</p>
 *
 * @author Sean Owen
 */
pub struct AlignmentPattern {
    estimatedModuleSize: f32,
    internal_result_point: RXingResultPoint,
}

impl AlignmentPattern {
    pub fn new(posX: f32, posY: f32, estimatedModuleSize: f32) -> Self {
        Self {
            estimatedModuleSize,
            internal_result_point: RXingResultPoint { x: posX, y: posY },
        }
    }

    /**
     * <p>Determines if this alignment pattern "about equals" an alignment pattern at the stated
     * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
     */
    pub fn aboutEquals(&self, moduleSize: f32, i: f32, j: f32) -> bool {
        if (i - self.internal_result_point.getY()).abs() <= moduleSize
            && (j - self.internal_result_point.getX()).abs() <= moduleSize
        {
            let moduleSizeDiff = (moduleSize - self.estimatedModuleSize).abs();
            return moduleSizeDiff <= 1.0 || moduleSizeDiff <= self.estimatedModuleSize;
        }
        return false;
    }

    /**
     * Combines this object's current estimate of a finder pattern position and module size
     * with a new estimate. It returns a new {@code FinderPattern} containing an average of the two.
     */
    pub fn combineEstimate(&self, i: f32, j: f32, newModuleSize: f32) -> AlignmentPattern {
        let combinedX = (self.internal_result_point.getX() + j) / 2.0;
        let combinedY = (self.internal_result_point.getY() + i) / 2.0;
        let combinedModuleSize = (self.estimatedModuleSize + newModuleSize) / 2.0;
        AlignmentPattern::new(combinedX, combinedY, combinedModuleSize)
    }
}
