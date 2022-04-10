use super::{Expr, Ident, Type};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stmt {
    Local(Local),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Local {
    pub ident: Ident,
    pub ty: Option<Type>,
    pub init: Option<Box<Expr>>,
}
