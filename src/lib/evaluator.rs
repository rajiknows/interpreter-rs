// evaluator.rs
use super::ast::{Expr, Stmt};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct Evaluator {
    env: HashMap<String, i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: HashMap::new(),
        }
    }

    pub fn eval(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Result<()> {
        match stmt {
            Stmt::Let(name, expr) => {
                let value = self.eval_expr(expr)?;
                self.env.insert(name, value);
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr)?;
                println!("{}", value);
                Ok(())
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<i64> {
        match expr {
            Expr::Int(value) => Ok(value),
            Expr::Ident(name) => self
                .env
                .get(&name)
                .cloned()
                .ok_or_else(|| anyhow!("undefined variable: {}", name)),
            Expr::Infix(left, op, right) => {
                let left_val = self.eval_expr(*left)?;
                let right_val = self.eval_expr(*right)?;
                match op.as_str() {
                    "+" => Ok(left_val + right_val),
                    "-" => Ok(left_val - right_val),
                    "*" => Ok(left_val * right_val),
                    "/" => Ok(left_val / right_val),
                    _ => Err(anyhow!("unknown operator: {}", op)),
                }
            }
        }
    }
}
