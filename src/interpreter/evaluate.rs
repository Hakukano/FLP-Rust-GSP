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

use regex::Regex;
use std::collections::HashMap;
use wildmatch::WildMatch;

use crate::{Expression, Node};

pub struct EvaluateRule {
    pub is_equal: fn(value: &str, target: &str) -> bool,
    pub is_equal_ci: fn(value: &str, target: &str) -> bool,
    pub is_greater_than: fn(value: &str, target: &str) -> bool,
    pub is_less_than: fn(value: &str, target: &str) -> bool,
    pub is_match_wildcard: fn(value: &str, target: &str) -> bool,
    pub is_match_regex: fn(value: &str, target: &str) -> bool,
    pub is_in: fn(value: &str, target: &[String]) -> bool,
    pub is_none: fn(value: &str) -> bool,
}
impl Default for EvaluateRule {
    fn default() -> Self {
        Self {
            is_equal: |value, target| value == target,
            is_equal_ci: |value, target| value.to_lowercase() == target.to_lowercase(),
            is_greater_than: |value, target| value > target,
            is_less_than: |value, target| value < target,
            is_match_wildcard: |value, target| WildMatch::new(target).matches(value),
            is_match_regex: |value, target| {
                let reg = Regex::new(target);
                if reg.is_err() {
                    return false;
                }
                let reg = reg.unwrap();
                reg.is_match(value)
            },
            is_in: |value, target| target.contains(&value.to_string()),
            is_none: |value| {
                value.eq_ignore_ascii_case("none") || value.eq_ignore_ascii_case("null")
            },
        }
    }
}

pub type EvaluateRules = HashMap<String, EvaluateRule>;
pub type EvaluatePairs = HashMap<String, String>;

pub fn interpret_expression(
    expression: &Expression,
    rules: &EvaluateRules,
    pairs: &EvaluatePairs,
) -> bool {
    match &expression.node {
        Node::And(left, right) => {
            interpret_expression(left, rules, pairs) && interpret_expression(right, rules, pairs)
        }
        Node::Or(left, right) => {
            interpret_expression(left, rules, pairs) || interpret_expression(right, rules, pairs)
        }
        Node::Not(expr) => !interpret_expression(expr, rules, pairs),
        Node::Equal(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_equal)(value, target)
        }
        Node::EqualCI(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_equal_ci)(value, target)
        }
        Node::Greater(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_greater_than)(value, target)
        }
        Node::Less(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_less_than)(value, target)
        }
        Node::Wildcard(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_match_wildcard)(value, target)
        }
        Node::Regex(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_match_regex)(value, target)
        }
        Node::Any(key, targets) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_in)(value, targets)
        }
        Node::Null(key) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_none)(value)
        }
    }
}

pub fn interpret(expression: &Expression, rules: &EvaluateRules, pairs: &EvaluatePairs) -> bool {
    interpret_expression(expression, rules, pairs)
}
