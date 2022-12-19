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

use std::fmt::Display;
use std::fmt::Formatter;

use crate::ast::write_comma_separated_list;
use crate::ast::Expr;
use crate::ast::Identifier;
use crate::ast::TableReference;

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStmt<'a> {
    pub table: TableReference<'a>,
    pub update_list: Vec<UpdateExpr<'a>>,
    pub selection: Option<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateExpr<'a> {
    pub name: Identifier<'a>,
    pub expr: Expr<'a>,
}

impl Display for UpdateStmt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UPDATE {} SET ", self.table)?;
        write_comma_separated_list(f, &self.update_list)?;
        if let Some(conditions) = &self.selection {
            write!(f, " WHERE {conditions}")?;
        }
        Ok(())
    }
}

impl Display for UpdateExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.expr)
    }
}
