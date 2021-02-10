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

fn output(node: ast::Node, parent: Option<i32>, node_num: i32) -> i32{
    let mut ret_num = node_num + 1;
    if let Some(p) = parent {
        println!("a{} -> a{};", p, node_num);
    }
    match node {
        Statement(s) => match s {
            Assignment(e1, e2) => {
                println!(r#"a{} [label = "Assignment"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            If(cond, block, elif, elblock) => {
                println!(r#"a{} [label = "If"];"#, node_num);
                ret_num = output(Expression(*cond), Some(node_num), ret_num);
                ret_num = output(Statement(*block), Some(node_num), ret_num);
                for s in elif {
                    ret_num = output(Statement(*s), Some(node_num), ret_num);
                }
                return if let Some(block) = elblock {
                    output(Statement(*block), Some(node_num), ret_num)
                } else {
                    ret_num
                }
            },
            ElIf(cond, block) => {
                println!(r#"a{} [label = "ElseIf"];"#, node_num);
                ret_num = output(Expression(*cond), Some(node_num), ret_num);
                return output(Statement(*block), Some(node_num), ret_num);
            },
            While(cond, block) => {
                println!(r#"a{} [label = "While"];"#, node_num);
                ret_num = output(Expression(*cond), Some(node_num), ret_num);
                return output(Statement(*block), Some(node_num), ret_num);
            },
            Block(statements) => {
                println!(r#"a{} [label = "Block"];"#, node_num);
                for s in statements {
                    ret_num = output(Statement(*s), Some(node_num), ret_num);
                }
                return ret_num;
            },
            Break => {
                println!(r#"a{} [label = "Break"];"#, node_num);
                return ret_num;
            },
        },
        Expression(e) => match e {
            Identifier(t) => {
                if let Token::Identifier(s) = t {
                    println!(r#"a{} [label = "Identifier: {}"];"#, node_num, s);
                }
                return ret_num;
            },
            Integer(t) => {
                if let Token::Integer(i) = t {
                    println!(r#"a{} [label = "Integer: {}"];"#, node_num, i);
                }
                return ret_num;
            },
            Float(t) => {
                if let Token::Float(f) = t {
                    println!(r#"a{} [label = "Float: {}"];"#, node_num, f);
                }
                return ret_num;
            },
            Multiply(e1, e2) => {
                println!(r#"a{} [label = "Multiply"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            Divide(e1, e2) => {
                println!(r#"a{} [label = "Divide"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            Add(e1, e2) => {
                println!(r#"a{} [label = "Add"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);

            },
            Subtract(e1, e2) => {
                println!(r#"a{} [label = "Subtract"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },

            Boolean(t) => {
                if let Token::Boolean(b) = t {
                    println!(r#"a{} [label = "Boolean: {}"];"#, node_num, b);
                }
                return ret_num;
            },
            And(e1, e2) => {
                println!(r#"a{} [label = "And"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            Or(e1, e2) => {
                println!(r#"a{} [label = "Or"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            Equal(e1, e2) => {
                println!(r#"a{} [label = "Equal"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            NotEqual(e1, e2) => {
                println!(r#"a{} [label = "NotEqual"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            Negate(e1) => {
                println!(r#"a{} [label = "Negate"];"#, node_num);
                return output(Expression(*e1), Some(node_num), ret_num);
            },
            LT(e1, e2) => {
                println!(r#"a{} [label = "LT"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            GT(e1, e2) => {
                println!(r#"a{} [label = "GT"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            LTE(e1, e2) => {
                println!(r#"a{} [label = "LTE"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
            },
            GTE(e1, e2) => {
                println!(r#"a{} [label = "GTE"];"#, node_num);
                ret_num = output(Expression(*e1), Some(node_num), ret_num);
                return output(Expression(*e2), Some(node_num), ret_num);
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