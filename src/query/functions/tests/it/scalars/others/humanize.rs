// Copyright 2022 Datafuse Labs
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

use crate::scalars::scalar_function_test::test_scalar_functions;
use crate::scalars::scalar_function_test::ScalarFunctionTest;

#[test]
fn test_humanize_size_function() -> Result<()> {
    let tests = vec![
        ScalarFunctionTest {
            name: "humanize_size(1024)",
            columns: vec![Series::from_data(vec![1024_u32])],
            expect: Series::from_data(vec!["1.00 KiB"]),
            error: "",
        },
        ScalarFunctionTest {
            name: "humanize_size(-1024)",
            columns: vec![Series::from_data(vec![-1024_i32])],
            expect: Series::from_data(vec!["-1.00 KiB"]),
            error: "",
        },
        ScalarFunctionTest {
            name: "humanize_size('abc')",
            columns: vec![Series::from_data(vec!["abc"])],
            expect: Series::from_data(vec!["-1 KiB"]),
            error: "Expected a numeric type, but got String",
        },
        ScalarFunctionTest {
            name: "humanize_size(true)",
            columns: vec![Series::from_data(vec![true])],
            expect: Series::from_data(vec!["-1 KiB"]),
            error: "Expected a numeric type, but got Boolean",
        },
    ];

    test_scalar_functions("humanize_size", &tests)
}

#[test]
fn test_humanize_size_nullable() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "humanize_size(null)",
        columns: vec![Series::from_data(vec![Some(1_048_576_i32), None])],
        expect: Series::from_data(vec![Some("1.00 MiB"), None]),
        error: "",
    }];

    test_scalar_functions("humanize_size", &tests)
}

#[test]
fn test_humanize_number_function() -> Result<()> {
    let tests = vec![
        ScalarFunctionTest {
            name: "humanize_number(1000)",
            columns: vec![Series::from_data(vec![1000_u32])],
            expect: Series::from_data(vec!["1 thousand"]),
            error: "",
        },
        ScalarFunctionTest {
            name: "humanize_number(-1000)",
            columns: vec![Series::from_data(vec![-1000_i32])],
            expect: Series::from_data(vec!["-1 thousand"]),
            error: "",
        },
        ScalarFunctionTest {
            name: "humanize_number('abc')",
            columns: vec![Series::from_data(vec!["abc"])],
            expect: Series::from_data(vec!["-1 thousand"]),
            error: "Expected a numeric type, but got String",
        },
        ScalarFunctionTest {
            name: "humanize_number(true)",
            columns: vec![Series::from_data(vec![true])],
            expect: Series::from_data(vec!["-1 thousand"]),
            error: "Expected a numeric type, but got Boolean",
        },
    ];

    test_scalar_functions("humanize_number", &tests)
}

#[test]
fn test_humanize_number_nullable() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "humanize_number(null)",
        columns: vec![Series::from_data(vec![Some(1_000_000_i32), None])],
        expect: Series::from_data(vec![Some("1 million"), None]),
        error: "",
    }];

    test_scalar_functions("humanize_number", &tests)
}
