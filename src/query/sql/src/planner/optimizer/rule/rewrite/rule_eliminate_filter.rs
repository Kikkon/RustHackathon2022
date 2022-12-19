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

use common_exception::Result;
use itertools::Itertools;

use crate::optimizer::rule::Rule;
use crate::optimizer::rule::RuleID;
use crate::optimizer::rule::TransformResult;
use crate::optimizer::SExpr;
use crate::plans::Filter;
use crate::plans::PatternPlan;
use crate::plans::RelOp;
use crate::plans::Scalar;

pub struct RuleEliminateFilter {
    id: RuleID,
    pattern: SExpr,
}

impl RuleEliminateFilter {
    pub fn new() -> Self {
        Self {
            id: RuleID::EliminateFilter,
            // Filter
            //  \
            //   *
            pattern: SExpr::create_unary(
                PatternPlan {
                    plan_type: RelOp::Filter,
                }
                .into(),
                SExpr::create_leaf(
                    PatternPlan {
                        plan_type: RelOp::Pattern,
                    }
                    .into(),
                ),
            ),
        }
    }
}

impl Rule for RuleEliminateFilter {
    fn id(&self) -> RuleID {
        self.id
    }

    fn apply(&self, s_expr: &SExpr, state: &mut TransformResult) -> Result<()> {
        let eval_scalar: Filter = s_expr.plan().clone().try_into()?;
        // First, de-duplication predicates.
        let origin_predicates = eval_scalar.predicates.clone();
        let predicates = eval_scalar
            .predicates
            .into_iter()
            .unique()
            .collect::<Vec<Scalar>>();
        if predicates.is_empty() {
            state.add_result(s_expr.child(0)?.clone());
        } else if origin_predicates.len() != predicates.len() {
            let filter = Filter {
                predicates,
                is_having: eval_scalar.is_having,
            };
            state.add_result(SExpr::create_unary(filter.into(), s_expr.child(0)?.clone()));
        }
        Ok(())
    }

    fn pattern(&self) -> &SExpr {
        &self.pattern
    }
}