use std::{borrow::Cow, fmt::Display, path::Iter};

use itertools::Itertools;
use num::{iter::Range, ToPrimitive};
use rust_decimal::Decimal;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expression<'a> {
    Literal(Cow<'a, str>),
    Variable(VariableName<'a>),
    Permutation(Permutation<'a>),
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(x) => x.fmt(f),
            Expression::Variable(v) => write!(f, "{{{v}}}",),
            Expression::Permutation(p) => p.fmt(f),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Permutation<'a> {
    pub iterable: PermutationIterable<'a>,
    pub ordering: Option<usize>,
    pub var_name: Option<VariableName<'a>>,
    pub hidden: bool,
}

impl Display for Permutation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sb = "{".to_string();
        if let Some(o) = self.ordering {
            sb.push_str(o.to_string().as_str());
            sb.push(':');
        }
        if let Some(vn) = self.var_name {
            sb.push_str(vn.0);
            sb.push_str(":");
        }
        sb.push_str(self.iterable.to_string().as_str());
        sb.push('}');
        if self.hidden {
            sb.push('!');
        }

        write!(f, "{}", sb)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub struct VariableName<'a>(pub &'a str);

impl Display for VariableName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PermutationIterable<'a> {
    Disjunction(Vec<&'a str>),
    Range {
        start: Decimal,
        end: Decimal,
        step: Decimal,
    },
}

impl<'a> PermutationIterable<'a> {
    pub fn collect_vec(&self) -> Vec<Cow<'a, str>> {
        //TODO - use an iterator, do not allocate
        match self {
            PermutationIterable::Disjunction(d) => d.iter().map(|&x| x.into()).collect_vec(),
            PermutationIterable::Range { start, end, step } => {
                debug_assert_eq!((end - start).is_sign_positive(), step.is_sign_positive());
                debug_assert!(!step.is_zero());

                let len = ((end - start) / step).floor().to_usize().unwrap();
                let mut vec = Vec::with_capacity(len);
                let mut c = *start;
                while c <= *end {
                    vec.push(c.to_string().into());
                    c += *step;
                }

                vec
            }
        }
    }
}

impl Display for PermutationIterable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermutationIterable::Disjunction(v) => v.join("|").fmt(f),
            PermutationIterable::Range { start, end, step } => write!(f, "{start};{end};{step}"),
        }
    }
}
