use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    frontend::token::operator::Operator,
    runtime::value::value::Value,
};

impl Value {
    pub fn apply_unary_operator(self, operator: &Operator) -> Result<Value, LangError> {
        match operator {
            Operator::Negation => self.not(),
            Operator::Substraction => self.negate(),
            Operator::Addition => self.positive(),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator '{operator}'")).into(),
            ),
        }
    }

    fn not(self) -> Result<Value, LangError> {
        match self {
            Value::Boolean(b) => Ok(Value::Boolean(!b)),
            _ => Err(type_error()),
        }
    }

    fn negate(self) -> Result<Value, LangError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(type_error()),
        }
    }

    fn positive(self) -> Result<Value, LangError> {
        match self {
            Value::Number(n) => Ok(Value::Number(n)),
            _ => Err(type_error()),
        }
    }
}

fn type_error() -> LangError {
    InterpreterError::InvalidOperator("Invalid operand for unary operator".to_string()).into()
}
