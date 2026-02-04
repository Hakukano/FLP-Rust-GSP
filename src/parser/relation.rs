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

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::space0,
    combinator::map_res,
};

use super::{atom::*, comparison::*};

#[derive(Debug)]
pub enum Relation {
    C(Comparison),
    Rar {
        left: Box<Relation>,
        right: Box<Relation>,
    },
    Rac {
        left: Box<Relation>,
        right: Comparison,
    },
    Car {
        left: Comparison,
        right: Box<Relation>,
    },
    Cac {
        left: Comparison,
        right: Comparison,
    },
    Ror {
        left: Box<Relation>,
        right: Box<Relation>,
    },
    Roc {
        left: Box<Relation>,
        right: Comparison,
    },
    Cor {
        left: Comparison,
        right: Box<Relation>,
    },
    Coc {
        left: Comparison,
        right: Comparison,
    },
    NR(Box<Relation>),
    NC(Comparison),
}

fn group_start(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn group_end(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn c(input: &str) -> IResult<&str, Box<Relation>> {
    map_res(
        (group_start, space0, comparison, space0, group_end),
        |(_, _, c, _, _): (&str, &str, Comparison, &str, &str)| {
            Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(Relation::C(c)))
        },
    )
    .parse(input)
}

macro_rules! bi_relation {
    ($fname:ident, $left_func:ident, $oper_func:ident, $right_func:ident, $left_type:ty, $oper_type:ident, $right_type:ty, $relation:ident) => {
        fn $fname(input: &str) -> IResult<&str, Box<Relation>> {
            map_res(
                (
                    group_start,
                    space0,
                    $left_func,
                    space0,
                    $oper_func,
                    space0,
                    $right_func,
                    space0,
                    group_end,
                ),
                |(_, _, left, _, _, _, right, _, _): (
                    &str,
                    &str,
                    $left_type,
                    &str,
                    $oper_type,
                    &str,
                    $right_type,
                    &str,
                    &str,
                )| {
                    Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(
                        Relation::$relation { left, right },
                    ))
                },
            )
            .parse(input)
        }
    };
}

bi_relation!(
    rar,
    relation,
    and,
    relation,
    Box<Relation>,
    And,
    Box<Relation>,
    Rar
);
bi_relation!(
    rac,
    relation,
    and,
    comparison,
    Box<Relation>,
    And,
    Comparison,
    Rac
);
bi_relation!(
    car,
    comparison,
    and,
    relation,
    Comparison,
    And,
    Box<Relation>,
    Car
);
bi_relation!(
    cac, comparison, and, comparison, Comparison, And, Comparison, Cac
);
bi_relation!(
    ror,
    relation,
    or,
    relation,
    Box<Relation>,
    Or,
    Box<Relation>,
    Ror
);
bi_relation!(
    roc,
    relation,
    or,
    comparison,
    Box<Relation>,
    Or,
    Comparison,
    Roc
);
bi_relation!(
    cor,
    comparison,
    or,
    relation,
    Comparison,
    Or,
    Box<Relation>,
    Cor
);
bi_relation!(
    coc, comparison, or, comparison, Comparison, Or, Comparison, Coc
);

macro_rules! uni_relation {
    ($fname:ident, $oper_func:ident, $target_func:ident, $oper_type:ty, $target_type:ty, $relation:ident) => {
        fn $fname(input: &str) -> IResult<&str, Box<Relation>> {
            map_res(
                (
                    group_start,
                    space0,
                    $oper_func,
                    space0,
                    $target_func,
                    space0,
                    group_end,
                ),
                |(_, _, _, _, target, _, _): (
                    &str,
                    &str,
                    $oper_type,
                    &str,
                    $target_type,
                    &str,
                    &str,
                )| {
                    Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(
                        Relation::$relation(target),
                    ))
                },
            )
            .parse(input)
        }
    };
}

uni_relation!(nr, not, relation, Not, Box<Relation>, NR);
uni_relation!(nc, not, comparison, Not, Comparison, NC);

pub fn relation(input: &str) -> IResult<&str, Box<Relation>> {
    map_res(
        alt((c, rar, rac, car, cac, ror, roc, cor, coc, nr, nc)),
        |r: Box<Relation>| Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(r),
    )
    .parse(input)
}
