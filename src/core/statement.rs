use std::{any, borrow::Cow, fmt::Display};

use anyhow::{bail, Ok};
use itertools::{Either, Itertools};
use rust_decimal::Decimal;

use super::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Statement<'a>(pub Vec<Expression<'a>>);

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().map(|x| x.to_string()).join("").fmt(f)
    }
}

impl Statement<'_> {
    pub fn fully_expand(self) -> Result<Vec<String>, anyhow::Error> {
        let mut results = Vec::<String>::new();
        let mut statements_to_expand: Vec<Self> = vec![self];

        while let Some(next) = statements_to_expand.pop() {
            let expanded = next.expand()?;
            match expanded {
                Either::Left(mut new_statements) => {
                    new_statements.reverse();
                    statements_to_expand.append(&mut new_statements)
                }
                Either::Right(prompt) => results.push(prompt),
            }
        }

        Ok(results)
    }

    pub fn expand(&self) -> Result<Either<Vec<Self>, String>, anyhow::Error> {
        if let Some((perm_index, perm)) = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Expression::Literal(_) => None,
                Expression::Variable(_) => None,
                Expression::Permutation(p) => Some((i, p)),
            })
            .sorted_by_key(|(i, x)| x.ordering)
            .next()
        {
            let variable_indices = if let Some(var_name) = perm.var_name {
                self.0
                    .iter()
                    .enumerate()
                    .filter(|(i, x)| {
                        if let Expression::Variable(o_var_name) = x {
                            var_name == *o_var_name
                        } else {
                            false
                        }
                    })
                    .map(|x| x.0)
                    .collect_vec()
            } else {
                vec![]
            };

            let values_vec = perm.iterable.collect_vec();

            let result: Result<Vec<Statement>, _> = values_vec
                .into_iter()
                .map(|new_value| {
                    let mut new_expressions = self.0.clone();

                    //replace variables first
                    for variable_index in variable_indices.iter() {
                        new_expressions[*variable_index] = Expression::Literal(new_value.clone());
                    }

                    if perm.hidden {
                        if variable_indices.is_empty(){
                            bail!("Hidden variable is not used")
                        }

                        new_expressions.remove(perm_index);
                    } else {
                        new_expressions[perm_index] = Expression::Literal(new_value);
                    }
                    Ok(Statement(new_expressions))
                    
                })
                .collect();

            Ok(Either::Left(result?))
        } else {
            //No more permutations
            let (literals, failures): (Vec<Cow<str>>, Vec<anyhow::Error>) = self
                .0
                .iter()
                .map(|x| -> Result<Cow<str>, anyhow::Error> {
                    match x {
                        Expression::Literal(l) => Ok(l.clone()),
                        Expression::Variable(v) => bail!("Variable {} is never defined", v),
                        Expression::Permutation(_) => unreachable!(),
                    }
                })
                .partition_result();

            if failures.is_empty() {
                return Ok(Either::Right(literals.iter().join("")));
            } else {
                bail!(failures.into_iter().map(|x| x.to_string()).join("; "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use ntest::test_case;

    #[test_case("abc", "abc")]
    #[test_case("{cat|dog}", "cat\ndog")]
    #[test_case(
        "{cat|dog} and {red|blue}",
        "cat and red\ncat and blue\ndog and red\ndog and blue"
    )]
    #[test_case(
        "{cat|dog} and {1:red|blue}",
        "cat and red\ncat and blue\ndog and red\ndog and blue"
    )]
    #[test_case("{1;3;1}", "1\n2\n3")]
    #[test_case("a {i}{1:i:cat|dog}!", "a cat\na dog")]
    #[test_case(
        "a (red:{0.0;1.0;0.3}) cat",
        "a (red:0.0) cat\na (red:0.3) cat\na (red:0.6) cat\na (red:0.9) cat"
    )]
    fn should_expand_to(input_str: &str, expected: &str) -> Result<(), anyhow::Error> {
        let parsed = parse_prompt(input_str)?;

        let expanded = parsed.fully_expand()?;

        let actual = expanded.join("\n");

        assert_eq!(actual, expected);

        Ok(())
    }
}
