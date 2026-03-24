#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
}
