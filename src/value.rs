use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Num(f64),
}

impl Value {
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Self::Num(..))
    }

    pub fn as_bool(self) -> bool {
        match self {
            Self::Bool(b) => b,
            _ => panic!(),
        }
    }

    pub fn as_num(self) -> f64 {
        match self {
            Self::Num(n) => n,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "(unit)"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Num(n) => write!(f, "{}", n),
        }
    }
}
