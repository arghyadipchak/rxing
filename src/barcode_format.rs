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

//package com.google.zxing;

use std::fmt::Display;

/**
 * Enumerates barcode formats known to this package. Please keep alphabetized.
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BarcodeFormat {
    /** Aztec 2D barcode format. */
    AZTEC,

    /** CODABAR 1D format. */
    CODABAR,

    /** Code 39 1D format. */
    CODE_39,

    /** Code 93 1D format. */
    CODE_93,

    /** Code 128 1D format. */
    CODE_128,

    /** Data Matrix 2D barcode format. */
    DATA_MATRIX,

    /** EAN-8 1D format. */
    EAN_8,

    /** EAN-13 1D format. */
    EAN_13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MAXICODE,

    /** PDF417 format. */
    PDF_417,

    /** QR Code 2D barcode format. */
    QR_CODE,

    /** RSS 14 */
    RSS_14,

    /** RSS EXPANDED */
    RSS_EXPANDED,

    /** UPC-A 1D format. */
    UPC_A,

    /** UPC-E 1D format. */
    UPC_E,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UPC_EAN_EXTENSION,
}

impl Display for BarcodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BarcodeFormat::AZTEC => "aztec",
                BarcodeFormat::CODABAR => "codabar",
                BarcodeFormat::CODE_39 => "code 39",
                BarcodeFormat::CODE_93 => "code 93",
                BarcodeFormat::CODE_128 => "code 128",
                BarcodeFormat::DATA_MATRIX => "datamatrix",
                BarcodeFormat::EAN_8 => "ean 8",
                BarcodeFormat::EAN_13 => "ean 13",
                BarcodeFormat::ITF => "itf",
                BarcodeFormat::MAXICODE => "maxicode",
                BarcodeFormat::PDF_417 => "pdf 417",
                BarcodeFormat::QR_CODE => "qrcode",
                BarcodeFormat::RSS_14 => "rss 14",
                BarcodeFormat::RSS_EXPANDED => "rss expanded",
                BarcodeFormat::UPC_A => "upc a",
                BarcodeFormat::UPC_E => "upc e",
                BarcodeFormat::UPC_EAN_EXTENSION => "upc/ean extension",
            }
        )
    }
}

/// Defaults to QRCode if no proper formatting available
impl From<&str> for BarcodeFormat {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "aztec" => BarcodeFormat::AZTEC,
            "codabar" => BarcodeFormat::CODABAR,
            "code 39" | "code_39" | "code39" => BarcodeFormat::CODE_39,
            "code 93" | "code_93" | "code93" => BarcodeFormat::CODE_93,
            "code 128" | "code_129" | "code128" => BarcodeFormat::CODE_128,
            "datamatrix" | "data matrix" | "data_matrix" => BarcodeFormat::DATA_MATRIX,
            "ean 8" | "ean_8" | "ean8" => BarcodeFormat::EAN_8,
            "ean 13" | "ean_13" | "ean13" => BarcodeFormat::EAN_13,
            "itf" | "itf_code" | "itf14" | "itf 14" | "itf_14" | "interleaved 2 of 5" => {
                BarcodeFormat::ITF
            }
            "maxicode" | "maxi_code" => BarcodeFormat::MAXICODE,
            "pdf 417" | "pdf_417" | "pdf417" => BarcodeFormat::PDF_417,
            "qrcode" | "qr_code" | "qr code" => BarcodeFormat::QR_CODE,
            "rss 14" | "rss_14" | "rss14" | "gs1 databar" => BarcodeFormat::RSS_14,
            "rss expanded" | "expanded rss" | "rss_expanded" => BarcodeFormat::RSS_EXPANDED,
            "upc a" | "upc_a" | "upca" => BarcodeFormat::UPC_A,
            "upc e" | "upc_e" | "upce" => BarcodeFormat::UPC_E,
            "upc ean extension" | "upc extension" | "ean extension" | "upc/ean extension"
            | "upc_ean_extension" => BarcodeFormat::UPC_EAN_EXTENSION,
            _ => BarcodeFormat::QR_CODE,
        }
    }
}
