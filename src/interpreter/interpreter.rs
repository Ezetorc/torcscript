use crate::{
    abstract_syntax_tree::{
        expression::Expression, literal::Literal, operator::Operator, statement::Statement,
    },
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::{environment::Environment, value::Value},
};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn execute(statements: Vec<Statement>) -> Result<(), LangError> {
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.execute_block(statements)
    }

    pub fn execute_block(&mut self, statements: Vec<Statement>) -> Result<(), LangError> {
        for statement in statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), LangError> {
        match statement {
            Statement::Print { expression } => self.handle_print(&expression),

            Statement::Expression { expression } => {
                self.evaluate_expression(&expression)?;
                Ok(())
            }

            Statement::VariableDeclaration {
                identifier,
                expression,
            } => self.handle_variable_declaration(identifier, expression),

            Statement::VariableAssignation {
                identifier,
                expression,
            } => self.handle_variable_assignation(identifier, expression),

            Statement::Conditional {
                condition,
                statements,
            } => self.handle_conditional_statements(condition, statements),

            _ => Err(LangError::Interpreter(InterpreterError::NotImplemented(
                "Statement execution not yet implemented".to_string(),
            ))),
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, LangError> {
        match expression {
            Expression::Literal(literal) => match literal {
                Literal::Number(number) => Ok(Value::Number(*number)),
                Literal::String(string) => Ok(Value::String(string.clone())),
                Literal::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
                Literal::None => Ok(Value::None),
            },

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value: Value = self.evaluate_expression(left)?;
                let right_value: Value = self.evaluate_expression(right)?;

                self.apply_operator(left_value, operator, right_value)
            }

            Expression::Unary {
                operator,
                expression,
            } => {
                let value: Value = self.evaluate_expression(expression)?;

                match value {
                    Value::Number(number) => match operator {
                        Operator::Substraction => Ok(Value::Number(-number)),
                        Operator::Addition => Ok(Value::Number(number)),
                        _ => {
                            return Err(LangError::Interpreter(InterpreterError::TypeMismatch(
                                "Expected addition or substraction operator".to_string(),
                            )));
                        }
                    },
                    _ => {
                        return Err(LangError::Interpreter(InterpreterError::TypeMismatch(
                            "Expected number".into(),
                        )));
                    }
                }
            }

            Expression::Identifier(identifier) => {
                let value: Option<&Value> = self.environment.get_variable(identifier);

                if let Some(value) = value {
                    return Ok(value.clone());
                } else {
                    return Err(LangError::Interpreter(InterpreterError::NotFound(format!(
                        "Variable '{identifier}' not found"
                    ))));
                }
            }
        }
    }

    pub fn apply_operator(
        &self,
        left: Value,
        operator: &Operator,
        right: Value,
    ) -> Result<Value, LangError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => match operator {
                Operator::Addition => Ok(Value::Number(a + b)),
                Operator::Substraction => Ok(Value::Number(a - b)),
                Operator::Multiplication => Ok(Value::Number(a * b)),
                Operator::Division => {
                    if b == 0 {
                        return Err(LangError::Interpreter(InterpreterError::DivisionByZero(
                            "Attempted to divide by zero".to_string(),
                        )));
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
            },

            _ => Err(LangError::Interpreter(InterpreterError::TypeMismatch(
                "Invalid operands".into(),
            ))),
        }
    }
}
