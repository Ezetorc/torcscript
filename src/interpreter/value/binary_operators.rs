use crate::{
    abstract_syntax_tree::operator::Operator,
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::value::value::Value,
};

impl Value {
    pub fn apply_binary_operator(
        self,
        operator: &Operator,
        right: Value,
    ) -> Result<Value, LangError> {
        match (self, right) {
            (Value::Number(left), Value::Number(right)) => {
                Value::apply_binary_number_operator(left, operator, right)
            }

            (Value::String(left), Value::String(right)) => {
                Value::apply_binary_string_operator(left, operator, right)
            }

            (Value::Boolean(left), Value::Boolean(right)) => {
                Value::apply_binary_boolean_operator(left, operator, right)
            }

            _ => Err(LangError::Interpreter(InterpreterError::TypeMismatch(
                "Invalid operands".into(),
            ))),
        }
    }

    pub fn apply_binary_boolean_operator(
        left: bool,
        operator: &Operator,
        right: bool,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::And => Ok(Value::Boolean(left && right)),
            Operator::Or => Ok(Value::Boolean(left || right)),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator {operator}")).into(),
            ),
        }
    }

    pub fn apply_binary_string_operator(
        left: String,
        operator: &Operator,
        right: String,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::Addition => Ok(Value::String(format!("{left}{right}"))),
            Operator::Equality => Ok(Value::Boolean(left == right)),
            Operator::Difference => Ok(Value::Boolean(left != right)),
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator {operator}")).into(),
            ),
        }
    }

    fn apply_binary_number_operator(
        left: i64,
        operator: &Operator,
        right: i64,
    ) -> Result<Value, LangError> {
        match operator {
            Operator::Addition => Ok(Value::Number(left + right)),
            Operator::Substraction => Ok(Value::Number(left - right)),
            Operator::Multiplication => Ok(Value::Number(left * right)),
            Operator::Equality => Ok(Value::Boolean(left == right)),
            Operator::Difference => Ok(Value::Boolean(left != right)),
            Operator::Greater => Ok(Value::Boolean(left > right)),
            Operator::GreaterOrEqual => Ok(Value::Boolean(left >= right)),
            Operator::Less => Ok(Value::Boolean(left < right)),
            Operator::LessOrEqual => Ok(Value::Boolean(left <= right)),
            Operator::Division => {
                if right == 0 {
                    return Err(InterpreterError::DivisionByZero(format!(
                        "Attempted to divide {left} by 0"
                    ))
                    .into());
                } else {
                    Ok(Value::Number(left / right))
                }
            }
            _ => Err(
                InterpreterError::InvalidOperator(format!("Invalid operator {operator}")).into(),
            ),
        }
    }
}
