#[derive(Debug)]
pub(crate) enum Expression {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Value(i64),
}

impl Expression {
    pub(crate) fn dependencies(&self) -> Option<[&str; 2]> {
        match self {
            Expression::Add(m1, m2)
            | Expression::Sub(m1, m2)
            | Expression::Mul(m1, m2)
            | Expression::Div(m1, m2) => Some([m1, m2]),
            Expression::Value(_) => None,
        }
    }
}
