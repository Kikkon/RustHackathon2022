// Copyright 2021 Datafuse Labs.
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

use std::fmt;
use std::sync::Arc;

use chrono_tz::Tz;
use common_datavalues::ColumnRef;
use common_datavalues::ColumnsWithField;
use common_datavalues::DataTypeImpl;
use common_datavalues::NullColumn;
use common_datavalues::NullType;
use common_exception::Result;
use dyn_clone::DynClone;

use super::Monotonicity;

/// for now, this is only store Timezone
#[derive(Clone)]
pub struct FunctionContext {
    pub tz: Tz,
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self {
            tz: "UTC".parse::<Tz>().unwrap(),
        }
    }
}

pub trait Function: fmt::Display + Sync + Send + DynClone {
    /// Returns the name of the function, should be unique.
    fn name(&self) -> &str;

    /// Calculate the monotonicity from arguments' monotonicity information.
    /// The input should be argument's monotonicity. For binary function it should be an
    /// array of left expression's monotonicity and right expression's monotonicity.
    /// For unary function, the input should be an array of the only argument's monotonicity.
    /// The returned monotonicity should have 'left' and 'right' fields None -- the boundary
    /// calculation relies on the function.eval method.
    fn get_monotonicity(&self, _args: &[Monotonicity]) -> Result<Monotonicity> {
        Ok(Monotonicity::default())
    }

    /// The method returns the return_type of this function.
    fn return_type(&self) -> DataTypeImpl;

    /// Evaluate the function, e.g. run/execute the function.
    fn eval(
        &self,
        _func_ctx: FunctionContext,
        _columns: &ColumnsWithField,
        _input_rows: usize,
    ) -> Result<ColumnRef>;

    /// If all args are constant column, then we just return the constant result
    /// TODO, we should cache the constant result inside the context for better performance
    fn passthrough_constant(&self) -> bool {
        true
    }
}

dyn_clone::clone_trait_object!(Function);

#[derive(Clone)]
pub struct AlwaysNullFunction;

impl Function for AlwaysNullFunction {
    fn name(&self) -> &str {
        "null"
    }

    fn return_type(&self) -> DataTypeImpl {
        DataTypeImpl::Null(NullType {})
    }

    fn eval(
        &self,
        _func_ctx: FunctionContext,
        _columns: &ColumnsWithField,
        input_rows: usize,
    ) -> Result<ColumnRef> {
        Ok(Arc::new(NullColumn::new(input_rows)))
    }
}

impl fmt::Display for AlwaysNullFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null")
    }
}
