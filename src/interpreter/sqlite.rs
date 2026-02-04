// This library implements GSP (General Search Parser)
// Copyright (C) 2026  Hakukaze Shikano
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use chrono::{DateTime, ParseError, Utc};
use std::{collections::HashMap, num::ParseFloatError, num::ParseIntError, str::ParseBoolError};

use crate::{Expression, Node};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot parse to int: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("Cannot parse to float: {0}")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Cannot parse to bool: {0}")]
    ParseBool(#[from] ParseBoolError),
    #[error("Cannot parse to chrono: {0}")]
    ParseChrono(#[from] ParseError),
    #[error("Cannot find key {0} in types")]
    UnknownKey(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum SqliteType {
    BigInt(Option<i64>),
    Blob(Option<Vec<u8>>),
    Boolean(Option<bool>),
    DateTime(Option<DateTime<Utc>>),
    Integer(Option<i32>),
    Real(Option<f64>),
    Text(Option<String>),
}
impl SqliteType {
    pub fn replace_and_return(&self, s: &str) -> Result<Self> {
        match self {
            SqliteType::BigInt(_) => Ok(SqliteType::BigInt(Some(s.parse()?))),
            SqliteType::Blob(_) => Ok(SqliteType::Blob(Some(s.as_bytes().to_vec()))),
            SqliteType::Boolean(_) => Ok(SqliteType::Boolean(Some(s.parse()?))),
            SqliteType::DateTime(_) => Ok(SqliteType::DateTime(Some(s.parse()?))),
            SqliteType::Integer(_) => Ok(SqliteType::Integer(Some(s.parse()?))),
            SqliteType::Real(_) => Ok(SqliteType::Real(Some(s.parse()?))),
            SqliteType::Text(_) => Ok(SqliteType::Text(Some(s.to_string()))),
        }
    }
}

pub type SqliteRenames = HashMap<String, String>;
pub type SqliteTypes = HashMap<String, SqliteType>;

pub fn interpret_expression(
    expression: &Expression,
    renames: &SqliteRenames,
    types: &SqliteTypes,
) -> Result<(String, Vec<SqliteType>)> {
    Ok(match &expression.node {
        Node::And(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} AND {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Node::Or(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} OR {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Node::Not(expr) => {
            let (clause, types) = interpret_expression(expr, renames, types)?;
            (format!("(NOT {})", clause), types)
        }
        Node::Equal(key, target) => (
            format!("{} = ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(target)?,
            ],
        ),
        Node::EqualCI(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(target)?,
            ],
        ),
        Node::Greater(key, target) => (
            format!("{} > ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(target)?,
            ],
        ),
        Node::Less(key, target) => (
            format!("{} < ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(target)?,
            ],
        ),
        Node::Wildcard(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(&target.replace("*", "%").replace("?", "_"))?,
            ],
        ),
        Node::Regex(key, target) => (
            format!("{} = ?", renames.get(key).unwrap_or(key)),
            vec![
                types
                    .get(key)
                    .ok_or(Error::UnknownKey(key.to_string()))?
                    .replace_and_return(target)?,
            ],
        ),
        Node::Any(key, targets) => {
            let sql = if targets.is_empty() {
                "FALSE".to_string()
            } else {
                format!(
                    "{} IN ({})",
                    renames.get(key).unwrap_or(key),
                    targets.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
                )
            };
            let mut binds = Vec::with_capacity(targets.len());
            for target in targets.iter() {
                binds.push(
                    types
                        .get(key)
                        .ok_or(Error::UnknownKey(key.to_string()))?
                        .replace_and_return(target)?,
                );
            }
            (sql, binds)
        }
        Node::Null(key) => {
            if !types.contains_key(key) {
                return Err(Error::UnknownKey(key.to_string()));
            }
            (
                format!("{} IS NULL", renames.get(key).unwrap_or(key)),
                vec![],
            )
        }
    })
}

pub fn interpret(
    expression: &Expression,
    renames: &SqliteRenames,
    types: &SqliteTypes,
) -> Result<(String, Vec<SqliteType>)> {
    interpret_expression(expression, renames, types)
}
