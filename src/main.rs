#[macro_use] extern crate lalrpop_util;
use std::io::{stdin, BufReader};

mod lexer;
mod ast;
use ast::Node::{Statement, Expression};
use ast::Statement::*;
use ast::Expression::*;
use lexer::Lexer;
use lexer::Token;

lalrpop_mod!(pub python); // synthesized by LALRPOP

fn output(node: ast::Node, parent: Option<i32>, nodeNum: i32) -> i32{
    let mut retNum = nodeNum + 1;
    if let Some(p) = parent {
        println!("a{} -> a{};", p, nodeNum);
    }
    match node {
        Statement(s) => match s {
            Assignment(e1, e2) => {
                println!(r#"a{} [label = "Assignment"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            If(cond, block, elif, elblock) => {
                println!(r#"a{} [label = "If"];"#, nodeNum);
                retNum = output(Expression(*cond), Some(nodeNum), retNum);
                retNum = output(Statement(*block), Some(nodeNum), retNum);
                for s in elif {
                    retNum = output(Statement(*s), Some(nodeNum), retNum);
                }
                return if let Some(block) = elblock {
                    output(Statement(*block), Some(nodeNum), retNum)
                } else {
                    retNum
                }
            },
            ElIf(cond, block) => {
                println!(r#"a{} [label = "ElseIf"];"#, nodeNum);
                retNum = output(Expression(*cond), Some(nodeNum), retNum);
                return output(Statement(*block), Some(nodeNum), retNum);
            },
            While(cond, block) => {
                println!(r#"a{} [label = "While"];"#, nodeNum);
                retNum = output(Expression(*cond), Some(nodeNum), retNum);
                return output(Statement(*block), Some(nodeNum), retNum);
            },
            Block(statements) => {
                println!(r#"a{} [label = "Block"];"#, nodeNum);
                for s in statements {
                    retNum = output(Statement(*s), Some(nodeNum), retNum);
                }
                return retNum;
            },
            Break => {
                println!(r#"a{} [label = "Break"];"#, nodeNum);
                return retNum;
            },
        },
        Expression(e) => match e {
            Identifier(t) => {
                if let Token::Identifier(s) = t {
                    println!(r#"a{} [label = "Identifier: {}"];"#, nodeNum, s);
                }
                return retNum;
            },
            Integer(t) => {
                if let Token::Integer(i) = t {
                    println!(r#"a{} [label = "Integer: {}"];"#, nodeNum, i);
                }
                return retNum;
            },
            Float(t) => {
                if let Token::Float(f) = t {
                    println!(r#"a{} [label = "Float: {}"];"#, nodeNum, f);
                }
                return retNum;
            },
            Multiply(e1, e2) => {
                println!(r#"a{} [label = "Multiply"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            Divide(e1, e2) => {
                println!(r#"a{} [label = "Divide"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            Add(e1, e2) => {
                println!(r#"a{} [label = "Add"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);

            },
            Subtract(e1, e2) => {
                println!(r#"a{} [label = "Subtract"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },

            Boolean(t) => {
                if let Token::Boolean(b) = t {
                    println!(r#"a{} [label = "Boolean: {}"];"#, nodeNum, b);
                }
                return retNum;
            },
            And(e1, e2) => {
                println!(r#"a{} [label = "And"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            Or(e1, e2) => {
                println!(r#"a{} [label = "Or"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            Equal(e1, e2) => {
                println!(r#"a{} [label = "Equal"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            NotEqual(e1, e2) => {
                println!(r#"a{} [label = "NotEqual"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            Negate(e1) => {
                println!(r#"a{} [label = "Negate"];"#, nodeNum);
                return output(Expression(*e1), Some(nodeNum), retNum);
            },
            LT(e1, e2) => {
                println!(r#"a{} [label = "LT"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            GT(e1, e2) => {
                println!(r#"a{} [label = "GT"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            LTE(e1, e2) => {
                println!(r#"a{} [label = "LTE"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
            GTE(e1, e2) => {
                println!(r#"a{} [label = "GTE"];"#, nodeNum);
                retNum = output(Expression(*e1), Some(nodeNum), retNum);
                return output(Expression(*e2), Some(nodeNum), retNum);
            },
        },
    }
}

fn main() {
    let mut input = BufReader::new(stdin());
    let lexer = Lexer::new(&mut input);
    let parser = python::ProgramParser::new();
    let program = parser.parse(lexer).unwrap();
    // println!("{:?}", program);

    println!("Digraph G{{");
    output(Statement(*program), None, 0);
    println!("}}");

    // for t in lexer {
    //     println!("{:?}", t);
    //     if t.is_err() {
    //         break;
    //     }
    // }
}

#[test]
fn calculator1() {
    assert!(python::TermParser::new().parse("22").is_ok());
    assert!(python::TermParser::new().parse("(22)").is_ok());
    assert!(python::TermParser::new().parse("((((22))))").is_ok());
    assert!(python::TermParser::new().parse("((22)").is_err());
}