use pest::{self, Parser};

use crate::ast::{BinaryOperator, Datum, Node};

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;
// ANCHOR_END: parser

// ANCHOR: parse_source
pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::program, source)?;
    for pair in pairs {
        if let Rule::expression = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}
// ANCHOR_END: parse_source

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::expression => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::binary_expression => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let lhs = build_ast_from_operand(lhspair);
            let op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let rhs = build_ast_from_operand(rhspair);
            parse_binary_expr(op, lhs, rhs)
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_operand(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::data => build_ast_from_data(pair),
        Rule::expression => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn build_ast_from_data(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::bool => Node::Data {
            value: pair.as_str().parse().unwrap(),
        },
        Rule::int => Node::Data {
            value: Datum::Int(pair.as_str().parse().unwrap()),
        },
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => BinaryOperator::Plus,
            "-" => BinaryOperator::Minus,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        assert!(parse("b").is_err());
    }

    // #[test]
    // fn nested_expr() {
    //     fn test_expr(expected: &str, src: &str) {
    //         assert_eq!(
    //             expected,
    //             parse(src)
    //                 .unwrap()
    //                 .iter()
    //                 .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
    //         );
    //     }

    //     test_expr("1 + 2 + 3", "(1 + 2) + 3");
    //     test_expr("1 + 2 + 3", "1 + (2 + 3)");
    //     test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
    //     test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
    // }
}
