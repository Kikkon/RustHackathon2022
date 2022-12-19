// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::prelude::*;
use common_exception::Result;
use serde_json::json;
use serde_json::Value as JsonValue;

use crate::scalars::scalar_function_test::test_scalar_functions;
use crate::scalars::scalar_function_test::ScalarFunctionTest;

#[test]
fn test_parse_json_function() -> Result<()> {
    let tests = vec![
        ScalarFunctionTest {
            name: "parse_json_bool",
            columns: vec![Series::from_data(vec![true, false])],
            expect: Series::from_data(vec![
                VariantValue::from(json!(true)),
                VariantValue::from(json!(false)),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_int",
            columns: vec![Series::from_data(vec![1_i16, -1, 100])],
            expect: Series::from_data(vec![
                VariantValue::from(json!(1_i16)),
                VariantValue::from(json!(-1_i16)),
                VariantValue::from(json!(100_i16)),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_float",
            columns: vec![Series::from_data(vec![12.34_f64, 56.79, 0.12345679])],
            expect: Series::from_data(vec![
                VariantValue::from(json!(12.34_f64)),
                VariantValue::from(json!(56.79_f64)),
                VariantValue::from(json!(0.12345679_f64)),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_string",
            columns: vec![Series::from_data(vec!["\"abcd\"", "true", "123"])],
            expect: Series::from_data(vec![
                VariantValue::from(json!("abcd")),
                VariantValue::from(json!(true)),
                VariantValue::from(json!(123)),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_array",
            columns: vec![Series::from_data(vec![
                "[\"an\", \"array\"]",
                "[\"str\", true]",
                "[1, 2, 3]",
            ])],
            expect: Series::from_data(vec![
                VariantValue::from(json!(["an", "array"])),
                VariantValue::from(json!(["str", true])),
                VariantValue::from(json!([1, 2, 3])),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_object",
            columns: vec![Series::from_data(vec![
                "{\"an\": \"object\"}",
                "{\"key\": true}",
                "{\"k\": 1}",
            ])],
            expect: Series::from_data(vec![
                VariantValue::from(json!({"an": "object"})),
                VariantValue::from(json!({"key": true})),
                VariantValue::from(json!({"k": 1})),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_invalid_string",
            columns: vec![Series::from_data(vec!["\"abcd\"", "[1,2", "{\"k"])],
            expect: Series::from_data(vec![
                VariantValue::from(JsonValue::Null),
                VariantValue::from(JsonValue::Null),
                VariantValue::from(JsonValue::Null),
            ]),
            error: "Error parsing JSON: EOF while parsing a list at line 1 column 4",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_bool",
            columns: vec![Series::from_data(vec![None, Some(true), Some(false)])],
            expect: Series::from_data(vec![
                None,
                Some(VariantValue::from(json!(true))),
                Some(VariantValue::from(json!(false))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_int",
            columns: vec![Series::from_data(vec![Some(1_i16), Some(-1), Some(100)])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(1_i16))),
                Some(VariantValue::from(json!(-1_i16))),
                Some(VariantValue::from(json!(100_i16))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_float",
            columns: vec![Series::from_data(vec![
                Some(12.34_f64),
                Some(56.79),
                Some(0.12345679),
            ])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(12.34_f64))),
                Some(VariantValue::from(json!(56.79_f64))),
                Some(VariantValue::from(json!(0.12345679_f64))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_string",
            columns: vec![Series::from_data(vec![
                None,
                Some("\"abcd\""),
                Some("true"),
            ])],
            expect: Series::from_data(vec![
                None,
                Some(VariantValue::from(json!("abcd"))),
                Some(VariantValue::from(json!(true))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_array",
            columns: vec![Series::from_data(vec![
                Some("[\"an\", \"array\"]"),
                Some("[\"str\", true]"),
                Some("[1, 2, 3]"),
            ])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(["an", "array"]))),
                Some(VariantValue::from(json!(["str", true]))),
                Some(VariantValue::from(json!([1, 2, 3]))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_object",
            columns: vec![Series::from_data(vec![
                Some("{\"an\": \"object\"}"),
                Some("{\"k\": true}"),
                Some("{\"k\": 1}"),
            ])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!({"an": "object"}))),
                Some(VariantValue::from(json!({"k": true}))),
                Some(VariantValue::from(json!({"k": 1}))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_nullable_invalid_string",
            columns: vec![Series::from_data(vec![
                Some("\"abcd\""),
                Some("[1,2"),
                Some("{\"k"),
            ])],
            expect: Series::from_data(vec![Some(VariantValue::from(json!("abcd"))), None, None]),
            error: "Error parsing JSON: EOF while parsing a list at line 1 column 4",
        },
    ];

    test_scalar_functions("parse_json", &tests)
}

#[test]
fn test_try_parse_json_function() -> Result<()> {
    let tests = vec![
        ScalarFunctionTest {
            name: "parse_json_bool",
            columns: vec![Series::from_data(vec![true, false])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(true))),
                Some(VariantValue::from(json!(false))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_int",
            columns: vec![Series::from_data(vec![1_i16, -1, 100])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(1_i16))),
                Some(VariantValue::from(json!(-1_i16))),
                Some(VariantValue::from(json!(100_i16))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_float",
            columns: vec![Series::from_data(vec![12.34_f64, 56.79, 0.12345679])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(12.34_f64))),
                Some(VariantValue::from(json!(56.79_f64))),
                Some(VariantValue::from(json!(0.12345679_f64))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_string",
            columns: vec![Series::from_data(vec!["\"abcd\"", "true", "123"])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!("abcd"))),
                Some(VariantValue::from(json!(true))),
                Some(VariantValue::from(json!(123i32))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_array",
            columns: vec![Series::from_data(vec![
                "[\"an\", \"array\"]",
                "[\"str\", true]",
                "[1, 2, 3]",
            ])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!(["an", "array"]))),
                Some(VariantValue::from(json!(["str", true]))),
                Some(VariantValue::from(json!([1i32, 2, 3]))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_object",
            columns: vec![Series::from_data(vec![
                "{\"an\": \"object\"}",
                "{\"key\": true}",
                "{\"k\": 1}",
            ])],
            expect: Series::from_data(vec![
                Some(VariantValue::from(json!({"an": "object"}))),
                Some(VariantValue::from(json!({"key": true}))),
                Some(VariantValue::from(json!({"k": 1i32}))),
            ]),
            error: "",
        },
        ScalarFunctionTest {
            name: "parse_json_invalid_string",
            columns: vec![Series::from_data(vec!["\"abcd\"", "[1,2", "{\"k"])],
            expect: Series::from_data(vec![Some(VariantValue::from(json!("abcd"))), None, None]),
            error: "",
        },
    ];

    test_scalar_functions("try_parse_json", &tests)
}
