use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::language::callable::LoxFunction;
use crate::language::class::LoxClass;
use crate::language::environment::Environment;
use crate::language::expr::Expr;
use crate::language::stmt::Stmt;
use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::RuntimeError;
use crate::language::std::clock::Clock;

#[derive(Debug)]
pub enum ExecSignal {
    Runtime(RuntimeError),
    Return { keyword: Token, value: Literal },
}

impl From<RuntimeError> for ExecSignal {
    fn from(error: RuntimeError) -> Self {
        Self::Runtime(error)
    }
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<usize, usize>
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define(
            String::from("clock"),
            Literal::Callable(Rc::new(Clock))
        );

        Self {
            environment: Rc::clone(&globals),
            globals,
            locals: HashMap::new()
        }
    }

    pub fn resolve(&mut self, id: usize, depth: usize) {
        self.locals.insert(id, depth);
    }

    fn look_up_variable(
        &self,
        name: &Token,
        id: usize
    ) -> Result<Literal, RuntimeError> {
        match self.locals.get(&id) {
            Some(distance) => Ok(
                Environment::get_at(&self.environment, *distance, &name.lexeme)
            ),
            None => self.globals.borrow().get(name)
        }
    }

    fn is_truthy(literal: Literal) -> bool {
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

    fn evaluate(&mut self, expr: Expr) -> Result<Literal, RuntimeError> {
        Ok(match expr {

            Expr::Grouping {expression} => self.evaluate(*expression)?,

            Expr::Literal {value} => value.clone(),

            Expr::Unary {right, operator} => {
                let right_eval = self.evaluate(*right)?;
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
                    TokenType::Bang => Literal::Boolean(!Interpreter::is_truthy(right_eval)),
                    _=> return Err(RuntimeError {
                        token: operator.clone(),
                        message: String::from("invalid unary operator")
                    })
                }
            }

            Expr::Binary { left, operator, right } => {
                let left_eval = self.evaluate(*left)?;
                let right_eval = self.evaluate(*right)?;

                match operator.kind {
                    TokenType::Minus => Literal::Number(
                        self.eval_number(&operator, &left_eval)? -
                            self.eval_number(&operator, &right_eval)?
                    ),
                    TokenType::Slash => Literal::Number(
                        self.eval_number(&operator, &left_eval)? /
                            self.eval_number(&operator, &right_eval)?
                    ),
                    TokenType::Star => Literal::Number(
                        self.eval_number(&operator, &left_eval)? *
                            self.eval_number(&operator, &right_eval)?
                    ),
                    TokenType::Plus => {
                        match (left_eval, right_eval) {
                            (Literal::Number(l),
                                Literal::Number(r)) => Literal::Number(l + r),
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
                            self.eval_number(&operator, &left_eval)? >
                                self.eval_number(&operator, &right_eval)?
                        )
                    }
                    TokenType::GreaterEqual => {
                        Literal::Boolean(
                            self.eval_number(&operator, &left_eval)? >=
                                self.eval_number(&operator, &right_eval)?
                        )
                    }
                    TokenType::Less => {
                        Literal::Boolean(
                            self.eval_number(&operator, &left_eval)? <
                                self.eval_number(&operator, &right_eval)?
                        )
                    }
                    TokenType::LessEqual => {
                        Literal::Boolean(
                            self.eval_number(&operator, &left_eval)? <=
                                self.eval_number(&operator, &right_eval)?
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

            Expr::Variable { id, name } => self.look_up_variable(&name, id)?,

            Expr::Assign { id, name, value } => {
                let value = self.evaluate(*value)?;

                match self.locals.get(&id) {
                    Some(distance) => Environment::assign_at(
                        &self.environment, *distance, &name, value.clone()
                    ),
                    None => { self.globals.borrow_mut().assign(name, &value)?; }
                }

                value
            }

            Expr::Logical { left, operator, right} => {
                let eval = self.evaluate(*left)?;

                if let TokenType::Or = operator {
                    if Interpreter::is_truthy(eval) {
                        return Ok(Literal::Boolean(true));
                    }
                } else {
                    if !Interpreter::is_truthy(eval) {
                        return Ok(Literal::Boolean(false));
                    }
                }

                self.evaluate(*right)?
            }

            Expr::Call { callee, paren, arguments } => {
                let callee_value = self.evaluate(*callee)?;

                let mut args: Vec<Literal> = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    args.push(self.evaluate(*argument)?);
                }

                let function = match callee_value {
                    Literal::Callable(f) => f,
                    _ => return Err(RuntimeError {
                        token: paren,
                        message: String::from("only able to call functions and classes")
                    })
                };

                if args.len() != function.arity() {
                    return Err(RuntimeError {
                        token: paren,
                        message: format!(
                            "expected {} arguments but got {}",
                            function.arity(),
                            args.len()
                        )
                    })
                }

                function.call(self, args)?
            }
        })
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Option<Rc<RefCell<Environment>>>,
    ) -> Result<(), ExecSignal> {
        let previous = Rc::clone(&self.environment);

        self.environment = environment.unwrap_or_else(|| {
            Rc::new(RefCell::new(
                Environment::from_parent(Rc::clone(&previous)),
            ))
        });

        let result = (|| {
            for statement in statements {
                self.execute(statement)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), ExecSignal> {
        match stmt {

            Stmt::Expression(expr) => { self.evaluate(expr)?; return Ok(()) },

            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{value}");
            }

            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Literal::Nil
                };

                self.environment.borrow_mut().define(name.lexeme, value);

            }

            Stmt::Block(statements) => {
                self.execute_block(statements, None)?;
            }

            Stmt::If { condition, then_branch, else_branch } => {
                let eval = self.evaluate(condition)?;
                if Interpreter::is_truthy(eval) {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = *else_branch {
                    self.execute(else_branch)?;
                }
            }

            Stmt::While { condition, body } => {
                while Interpreter::is_truthy(self.evaluate(
                    condition.clone().unwrap_or(
                        Expr::Literal { value: Literal::Boolean(true) }
                    )
                )?) {
                    self.execute(*body.clone())?;
                }
            }

            Stmt::Function { name, params, body } => {
                let function_name = name.lexeme.clone();

                let function = LoxFunction {
                    closure: Rc::clone(&self.environment),
                    declaration: Stmt::Function { name, params, body },
                };

                self.environment.borrow_mut().define(
                    function_name,
                    Literal::Callable(Rc::new(function)),
                );
            }

            Stmt::Return { keyword, value } => {
                let value = self.evaluate(value)?;
                return Err(ExecSignal::Return { keyword, value })
            }

            Stmt::Class { name, .. } => {
                self.environment.borrow_mut().define(name.lexeme.clone(), Literal::Nil);

                let class = LoxClass { name: name.lexeme.clone() };

                self.environment.borrow_mut().assign(
                    name,
                    &Literal::Class(Rc::new(class))
                )?;
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            match self.execute(stmt) {
                Ok(()) => {}
                Err(ExecSignal::Runtime(error)) => return Err(error),
                Err(ExecSignal::Return { keyword, .. }) => return Err(RuntimeError {
                    token: keyword,
                    message: String::from("can't return from top-level code")
                })
            }
        }
        Ok(())
    }
}
