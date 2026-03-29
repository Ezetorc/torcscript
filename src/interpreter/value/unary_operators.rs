use crate::{
    abstract_syntax_tree::operator::Operator,
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::value::value::Value,
};

impl Value {
    pub fn apply_unary_operator(self, operator: &Operator) -> Result<Value, LangError> {
        match self {
            Value::Number(number) => Value::apply_unary_number_operators(operator, number),
            Value::Boolean(boolean) => Value::apply_unary_boolean_operators(operator, boolean),
            _ => Err(InterpreterError::TypeMismatch(format!("Invalid value type '{self}'")).into()),
        }
    }

    pub fn apply_unary_boolean_operators(
        operator: &Operator,
        boolean: bool,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::Negation => Ok(Value::Boolean(!boolean)),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator {operator}")).into(),
            ),
        }
    }

    pub fn apply_unary_number_operators(
        operator: &Operator,
        number: i64,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::Substraction => Ok(Value::Number(-number)),
            Operator::Addition => Ok(Value::Number(number)),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator {operator}")).into(),
            ),
        }
    }
}
