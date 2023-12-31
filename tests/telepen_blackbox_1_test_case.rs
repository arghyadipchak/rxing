/*
 * Copyright 2008 ZXing authors
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

use rxing::{oned::TelepenReader, BarcodeFormat, oned::TelepenCommon};

mod common;

/**
 * @author Chris Wood
 */
#[test]
fn telepen_alpha_test_case() {
    const NUMTESTS: u32 = 5;

    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/telepen-1",
        TelepenReader::new(),
        BarcodeFormat::TELEPEN,
    );
    // super("src/test/resources/blackbox/Telepen-1", new MultiFormatReader(), BarcodeFormat.Telepen);
    tester.add_test(NUMTESTS, 2, 0.0);
    tester.add_test(NUMTESTS, 2, 180.0);

    tester.test_black_box();
}

#[test]
fn telepen_numeric_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/telepen-2",
        TelepenReader::new(),
        BarcodeFormat::TELEPEN,
    );

    tester.add_test_complex(1, 1, 0, 0, 0.0);
    tester.add_hint(rxing::DecodeHintType::TELEPEN_AS_NUMERIC, rxing::DecodeHintValue::TelepenAsNumeric(true));
    tester.ignore_pure = true;

    tester.test_black_box();
}

#[test]
fn telepen_checksum_test1() {
    let contents = "Hello world!";
    let checksum = TelepenCommon::calculate_checksum(contents);
    assert_eq!('\u{1a}', checksum);
}

#[test]
fn telepen_checksum_test2() {
    let contents = "ABC123456";
    let checksum = TelepenCommon::calculate_checksum(contents);
    assert_eq!('\u{1}', checksum);
}

#[test]
fn telepen_alpha_to_numeric_test() {
    let ascii = "'=Siu";
    let result = TelepenCommon::ascii_to_numeric(ascii);
    assert_eq!("1234567890", result);
}

#[test]
fn telepen_numeric_to_ascii_test() {
    let numeric = "1234567890";
    let result = TelepenCommon::numeric_to_ascii(numeric).unwrap();
    assert_eq!("'=Siu", result);
}