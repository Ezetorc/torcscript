use crate::runtime::program::statement::Statement;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}
