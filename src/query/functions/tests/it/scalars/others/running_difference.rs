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
use crate::scalars::scalar_function_test::test_scalar_functions_with_type;
use crate::scalars::scalar_function_test::ScalarFunctionTest;
use crate::scalars::scalar_function_test::ScalarFunctionWithFieldTest;

#[test]
fn test_running_difference_first_null() -> Result<()> {
    let tests = vec![
        ScalarFunctionTest {
            name: "i8_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i8),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i16)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u8_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u8),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i16)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i16_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i16),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i32)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u16_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u16),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i32)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i32_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i32),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u32_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u32),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i64_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i64),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u64_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u64),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i8_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i8),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i16), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u8_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u8),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i16), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i16_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i16),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i32), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u16_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u16),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i32), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i32_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i32),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u32_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u32),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "i64_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i64),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunctionTest {
            name: "u64_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u64),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
    ];

    test_scalar_functions("running_difference", &tests)
}

#[test]
fn test_running_difference_datetime32_first_null() -> Result<()> {
    let tests = vec![
        ScalarFunctionWithFieldTest {
            name: "datetime32_first_null",
            columns: vec![ColumnWithField::new(
                Series::from_data([None, Some(3_i64), None, Some(4), Some(10)]),
                DataField::new("dummy_1", NullableType::new_impl(TimestampType::new_impl())),
            )],
            expect: Series::from_data([None, None, None, None, Some(6_i64)]),
            error: "",
        },
        ScalarFunctionWithFieldTest {
            name: "datetime32_first_not_null",
            columns: vec![ColumnWithField::new(
                Series::from_data([Some(2_i64), Some(3), None, Some(4), Some(10)]),
                DataField::new("dummy_1", NullableType::new_impl(TimestampType::new_impl())),
            )],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
    ];

    test_scalar_functions_with_type("running_difference", &tests)
}
