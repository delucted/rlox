use crate::language::expr::Expr;
use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::RuntimeError;

pub struct Interpreter {

}

impl Interpreter {
    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Boolean(b) => b.clone(),
            Literal::Nil => false,
            _=>true
        }
    }

    fn eval_number(&self, operator: &Token, literal: &Literal) -> Result<f64, RuntimeError> {
        Ok(match literal {
            Literal::Number(n) => n.clone(),
            _=>return Err(RuntimeError {
                token: operator.clone(),
                message: String::from("operand must be number")
            })
        })
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        Ok(match expr {

            Expr::Grouping {expression} => self.evaluate(expression)?,

            Expr::Literal {value} => value.clone(),

            Expr::Unary {right, operator} => {
                let right_eval = self.evaluate(right)?;
                match operator.kind {
                    TokenType::Minus => Literal::Number(
                        match right_eval {
                            Literal::Number(n) => -n,
                            _=> return Err(RuntimeError {
                                token: operator.clone(),
                                message: String::from("operand must be number")
                            })
                        }
                    ),
                    TokenType::Bang => Literal::Boolean(!self.is_truthy(&right_eval)),
                    _=> return Err(RuntimeError {
                        token: operator.clone(),
                        message: String::from("invalid unary operator")
                    })
                }
            }

            Expr::Binary { left, operator, right } => {
                let left_eval = self.evaluate(left)?;
                let right_eval = self.evaluate(right)?;

                match operator.kind {
                    TokenType::Minus => Literal::Number(
                        self.eval_number(operator, &left_eval)? - self.eval_number(operator, &right_eval)?
                    ),
                    TokenType::Slash => Literal::Number(
                        self.eval_number(operator, &left_eval)? / self.eval_number(operator, &right_eval)?
                    ),
                    TokenType::Star => Literal::Number(
                        self.eval_number(operator, &left_eval)? * self.eval_number(operator, &right_eval)?
                    ),
                    TokenType::Plus => {
                        match (left_eval, right_eval) {
                            // Number addition
                            (Literal::Number(l),
                                Literal::Number(r)) => Literal::Number(l + r),
                            // String concatenation
                            (Literal::String(mut l),
                                Literal::String(r)) => {
                                l.push_str(&r);
                                Literal::String(l)
                            }
                            _ => return Err(RuntimeError {
                                token: operator.clone(),
                                message: String::from("operand must be two numbers or two strings")
                            })
                        }
                    }
                    TokenType::Greater => {
                        Literal::Boolean(
                            self.eval_number(operator, &left_eval)? > self.eval_number(operator, &right_eval)?
                        )
                    }
                    TokenType::GreaterEqual => {
                        Literal::Boolean(
                            self.eval_number(operator, &left_eval)? >= self.eval_number(operator, &right_eval)?
                        )
                    }
                    TokenType::Less => {
                        Literal::Boolean(
                            self.eval_number(operator, &left_eval)? < self.eval_number(operator, &right_eval)?
                        )
                    }
                    TokenType::LessEqual => {
                        Literal::Boolean(
                            self.eval_number(operator, &left_eval)? <= self.eval_number(operator, &right_eval)?
                        )
                    }
                    TokenType::BangEqual => Literal::Boolean(!(left_eval == right_eval)),
                    TokenType::EqualEqual => Literal::Boolean(left_eval == right_eval),
                    _ => return Err(RuntimeError {
                        token: operator.clone(),
                        message: String::from("invalid binary operator")
                    })
                }
            }
        })
    }
    
    pub fn interpret(&self, expr: Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(&expr)
    }
}