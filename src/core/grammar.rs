use itertools::Itertools;
use pest::pratt_parser::Op;
use pest_consume::Error;
use pest_consume::Parser;
use rust_decimal::Decimal;

use super::prelude::*;
use pest_consume::match_nodes;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;
// Construct the first half of the parser using pest as usual.
#[derive(Parser)]
#[grammar = "core/prompt-lang.pest"]
struct PromptParser;

#[pest_consume::parser]

impl PromptParser {
    fn expression(input: Node) -> Result<Expression> {
        Ok(match_nodes!(input.into_children();
            [literal(fds)] => fds,
            [var_use(fds)] => fds,
            [permutation(fds)] => Expression::Permutation(fds),
        ))
    }

    fn hidden_marker(input: Node) -> Result<()> {
        Ok(())
    }

    fn ordering(input: Node) -> Result<usize> {
        input
            .as_str()
            .trim_matches(':')
            .parse::<usize>()
            .map_err(|x| input.error(x))
    }

    fn permutation(input: Node) -> Result<Permutation> {
        let mut ordering: Option<usize> = None;
        let mut var_name: Option<VariableName> = None;
        let mut hidden = false;
        let mut iterable_option: Option<PermutationIterable> = None;

        for child in input.children() {
            match child.as_rule() {
                Rule::hidden_marker => hidden = true,

                Rule::permutation_inner => {
                    let pi = PromptParser::permutation_inner(child)?;
                    iterable_option = Some(pi);
                }
                Rule::var_assign => {
                    let va = PromptParser::var_assign(child)?;
                    var_name = Some(va);
                }
                Rule::ordering => {
                    let o = PromptParser::ordering(child)?;
                    ordering = Some(o);
                }
                _ => unreachable!(),
            }
        }

        if let Some(iterable) = iterable_option {
            Ok(Permutation {
                ordering,
                var_name,
                hidden,
                iterable,
            })
        } else {
            Err(input.error("No permutation"))
        }
    }

    fn permutation_inner(input: Node) -> Result<PermutationIterable> {
        Ok(match_nodes!(input.into_children();
            [disjunction_inner(fds)] => fds,
            [range_inner(fds)] => fds,
        ))
    }

    fn var_assign(input: Node) -> Result<VariableName> {
        Ok(match_nodes!(input.into_children();
            [var_name(fds)] => fds,
        ))
    }

    fn var_name(input: Node) -> Result<VariableName> {
        let str = input.as_str();
        let trimmed = &str[1..str.len() - 1]; //This works because the string is ascii

        Ok(VariableName(trimmed))
    }

    fn var_use(input: Node) -> Result<Expression> {
        Ok(match_nodes!(input.into_children();
            [var_name(fds)] => Expression::Variable(fds),
        ))
    }

    fn disjunction_inner(input: Node) -> Result<PermutationIterable> {
        let vec = input.as_str().split('|').collect_vec();
        Ok(PermutationIterable::Disjunction(vec))
    }

    fn range_inner(input: Node) -> Result<PermutationIterable> {
        if let Some((start, end, step)) = input
            .as_str()
            .split(';')
            .map(|x| x.parse::<Decimal>().unwrap())
            .collect_tuple()
        {
            if step.is_zero() {
                return Err(input.error("Step cannot be zero"));
            }

            if (end - start).is_sign_positive() != step.is_sign_positive() {
                return Err(input.error("Step has the wrong sign"));
            }

            Ok(PermutationIterable::Range { start, end, step })
        } else {
            Err(input.error("Expected three elements in a range"))
        }
    }

    fn literal(input: Node) -> Result<Expression> {
        Ok(Expression::Literal(input.as_str().into()))
    }

    fn statement(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [expression(fds)..] => Statement(fds.collect()),
        ))
    }

    fn file(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [statement(fds), EOI(())] => fds,
        ))
    }

    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
}

pub fn parse_prompt(input_str: &str) -> Result<Statement> {
    // Parse the input into `Nodes`
    let inputs = PromptParser::parse(Rule::file, input_str)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    // Consume the `Node` recursively into the final value
    PromptParser::file(input)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use ntest::test_case;

    #[test_case("{1:<i>:cat|dog|fish}!")]
    #[test_case("abc")]
    #[test_case("{cat|dog}")]
    #[test_case("{1;2;3}")]
    #[test_case("{<i>} and {1:<i>:cat|dog|fish}!")]
    #[test_case("a (red:{0.0;1.0;0.1}) cat")]
    fn should_parse_and_round_trip(input_str: &str) -> Result<()> {
        let parsed = parse_prompt(input_str)?;

        assert_eq!(input_str, parsed.to_string());

        Ok(())
    }
}
