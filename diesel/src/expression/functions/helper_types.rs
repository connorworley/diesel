#![allow(non_camel_case_types)]

use dsl::AsExprOf;
use expression::grouped::Grouped;
use expression::operators;
use sql_types::Bool;

/// The return type of [`not(expr)`](../dsl/fn.not.html)
pub type not<Expr> = operators::Not<Grouped<AsExprOf<Expr, Bool>>>;

/// The return type of `not(expr)`
#[deprecated(since = "1.1.0", note = "use `not` instead")]
#[cfg(feature = "with-deprecated")]
pub type Not<Expr> = not<Expr>;

/// The return type of [`max(expr)`](../dsl/fn.max.html)
pub type max<Expr> = super::aggregate_ordering::Max<Expr>;

/// The return type of [`min(expr)`](../dsl/fn.min.html)
pub type min<Expr> = super::aggregate_ordering::Min<Expr>;

/// The return type of [`sum(expr)`](../dsl/fn.sum.html)
pub type sum<Expr> = super::aggregate_folding::Sum<Expr>;

/// The return type of [`avg(expr)`](../dsl/fn.avg.html)
pub type avg<Expr> = super::aggregate_folding::Avg<Expr>;
