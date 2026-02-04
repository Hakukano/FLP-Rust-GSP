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

#![forbid(unsafe_code)]

pub mod interpreter;
mod parser;

use std::str::FromStr;

use parser::comparison::Comparison;
use parser::relation::Relation;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error {0}")]
    Parser(String),
}

#[derive(Debug)]
pub enum Node {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Equal(String, String),
    EqualCI(String, String),
    Greater(String, String),
    Less(String, String),
    Wildcard(String, String),
    Regex(String, String),
    Any(String, Vec<String>),
    Null(String),
}

#[derive(Debug)]
pub struct Expression {
    pub node: Node,
}

impl From<Comparison> for Expression {
    fn from(c: Comparison) -> Self {
        match c {
            Comparison::IsEqual(c) => Self {
                node: Node::Equal(c.left.0, c.right.0),
            },
            Comparison::IsEqualCI(c) => Self {
                node: Node::EqualCI(c.left.0, c.right.0),
            },
            Comparison::IsGreater(c) => Self {
                node: Node::Greater(c.left.0, c.right.0),
            },
            Comparison::IsLess(c) => Self {
                node: Node::Less(c.left.0, c.right.0),
            },
            Comparison::IsWildcard(c) => Self {
                node: Node::Wildcard(c.left.0, c.right.0),
            },
            Comparison::IsRegex(c) => Self {
                node: Node::Regex(c.left.0, c.right.0),
            },
            Comparison::IsAny(c) => Self {
                node: Node::Any(c.left.0, c.right.0),
            },
            Comparison::IsNull(c) => Self {
                node: Node::Null(c.0.0),
            },
        }
    }
}

impl From<Box<Relation>> for Expression {
    fn from(relation: Box<Relation>) -> Self {
        match *relation {
            Relation::C(c) => c.into(),
            Relation::Rar { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Rac { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Car { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Cac { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Ror { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Roc { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Cor { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::Coc { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::NR(r) => Self {
                node: Node::Not(Box::new(r.into())),
            },
            Relation::NC(c) => Self {
                node: Node::Not(Box::new(c.into())),
            },
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parser::relation::relation(s)
            .map_err(|err| Error::Parser(err.to_string()))?
            .1
            .into())
    }
}
