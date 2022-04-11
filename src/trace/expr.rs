use crate::lang::{Block, Expr, ExprIf, Type};

pub trait ShadyValue {
    type TraceValue: TraceValue;
}

pub trait TraceValue {
    fn ty() -> Type;
}

pub struct TraceExpr<T> {
    pub expr: Expr,
    pub inner: T,
}

pub struct TraceBlock<T> {
    pub block: Block,
    pub inner: T,
}

fn if_expr<T>(
    cond: TraceExpr<TraceBool>,
    then_branch: TraceBlock<T>,
    else_branch: Option<TraceExpr<T>>,
) -> TraceExpr<T>
where
    T: TraceValue,
{
    TraceExpr {
        expr: Expr::If(ExprIf {
            cond: Box::new(cond.expr),
            then_branch: then_branch.block,
            else_branch: else_branch.map(|e| Box::new(e.expr)),
            ty: T::ty(),
        }),
        inner: then_branch.inner,
    }
}

/*pub trait TraceExpr {
    fn epxr() -> Expr;
    fn ty() -> Type;
}*/

pub struct TraceBool;
