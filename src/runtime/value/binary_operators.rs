use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    frontend::token::operator::Operator,
    runtime::value::value::Value,
};

impl Value {
    pub fn apply_binary_operator(
        self,
        operator: &Operator,
        right: Value,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::Addition => self.add(right),
            Operator::Substraction => self.substract(right),
            Operator::Multiplication => self.multiplicate(right),
            Operator::Division => self.divide(right),
            Operator::Equality => self.equals_op(right),
            Operator::Difference => self.difference(right),
            Operator::And => self.and(right),
            Operator::Or => self.or(right),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator '{operator}'")).into(),
            ),
        }
    }

    fn add(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
            (left, right) => Ok(Value::String(format!(
                "{}{}",
                left.to_string(),
                right.to_string()
            ))),
        }
    }

    fn substract(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
            _ => Err(type_error()),
        }
    }

    fn multiplicate(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
            _ => Err(type_error()),
        }
    }

    fn divide(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Number(_), Value::Number(0)) => {
                Err(InterpreterError::DivisionByZero("Division by zero".into()).into())
            }
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left / right)),
            _ => Err(type_error()),
        }
    }

    fn equals_op(self, right: Value) -> Result<Value, LangError> {
        Ok(Value::Boolean(self.equals(&right)))
    }

    fn difference(self, right: Value) -> Result<Value, LangError> {
        Ok(Value::Boolean(!self.equals(&right)))
    }

    fn and(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left && right)),
            _ => Err(type_error()),
        }
    }

    fn or(self, right: Value) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left || right)),
            _ => Err(type_error()),
        }
    }

    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

fn type_error() -> LangError {
    InterpreterError::InvalidOperator("Invalid operands".to_string()).into()
}
