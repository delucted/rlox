use crate::language::expr::Expr;
use crate::language::token::Literal;
use crate::language::token_type::TokenType;
use crate::util::errors::runtime_error;

pub struct Interpreter {

}

impl Interpreter {
    fn is_truthy(&self, literal: Literal) -> bool {
        match literal {
            Literal::Boolean(b) => b,
            Literal::Nil => false,
            _=>true
        }
    }

    fn eval_number(&self, literal: Literal) -> Result<f64, &'static str> {
        Ok(match literal {
            Literal::Number(n) => n,
            _=>return Err("operand must be number")
        })
    }

    fn is_equal(&self, a: Literal, b: Literal) -> bool {
        if a == Literal::Nil && b == Literal::Nil {
            return true
        }
        if a == Literal::Nil {
            return false
        }
        a == b
    }
    fn evaluate(&self, expr: &Expr) -> Result<Literal, &'static str> {
        Ok(match expr {

            Expr::Grouping {expression} => self.evaluate(expression)?,

            Expr::Literal {value} => value.clone(),

            Expr::Unary {right, operator} => {
                let right_eval = self.evaluate(right)?;
                match operator.kind {
                    TokenType::Minus => Literal::Number(
                        match right_eval {
                            Literal::Number(n) => -n,
                            _=> return Err("operand must be number")
                        }
                    ),
                    TokenType::Bang => Literal::Boolean(!self.is_truthy(right_eval)),
                    _=> return Err("invalid unary operator")
                }
            }

            Expr::Binary { left, operator, right } => {
                let left_eval = self.evaluate(left)?;
                let right_eval = self.evaluate(right)?;

                match operator.kind {
                    TokenType::Minus => Literal::Number(
                        self.eval_number(left_eval)? - self.eval_number(right_eval)?
                    ),
                    TokenType::Slash => Literal::Number(
                        self.eval_number(left_eval)? / self.eval_number(right_eval)?
                    ),
                    TokenType::Star => Literal::Number(
                        self.eval_number(left_eval)? * self.eval_number(right_eval)?
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
                            _ => return Err("operands must be two numbers or two strings")
                        }
                    }
                    TokenType::Greater => {
                        Literal::Boolean(
                            self.eval_number(left_eval)? > self.eval_number(right_eval)?
                        )
                    }
                    TokenType::GreaterEqual => {
                        Literal::Boolean(
                            self.eval_number(left_eval)? >= self.eval_number(right_eval)?
                        )
                    }
                    TokenType::Less => {
                        Literal::Boolean(
                            self.eval_number(left_eval)? < self.eval_number(right_eval)?
                        )
                    }
                    TokenType::LessEqual => {
                        Literal::Boolean(
                            self.eval_number(left_eval)? <= self.eval_number(right_eval)?
                        )
                    }
                    TokenType::BangEqual => Literal::Boolean(!self.is_equal(left_eval, right_eval)),
                    TokenType::EqualEqual => Literal::Boolean(self.is_equal(left_eval, right_eval)),
                    _ => return Err("invalid binary operator")
                }
            }
        })
    }
    
    pub fn interpret(&self, expr: Expr) -> Result<Literal, &'static str> {
        match self.evaluate(&expr) {
            Ok(literal) => Ok(literal),
            Err(message) => {
                runtime_error(message);
                Err(message)
            }
        }
    }
}