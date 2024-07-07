use crate::expressions::affine_expression::AffineExpression;
use crate::Float;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub expression: AffineExpression,
    pub lower_bound: Option<Float>,
    pub upper_bound: Option<Float>,
    pub is_equality: bool,
}

pub fn eq<A: Into<AffineExpression>, B: Into<AffineExpression>>(lhs: A, rhs: B) -> Constraint {
    Constraint {
        expression: lhs.into() - rhs,
        lower_bound: None,
        upper_bound: None,
        is_equality: true,
    }
}

pub fn leq<A: Into<AffineExpression>, B: Into<AffineExpression>>(lhs: A, rhs: B) -> Constraint {
    Constraint {
        expression: lhs.into() - rhs,
        lower_bound: None,
        upper_bound: Some(0.0),
        is_equality: false,
    }
}

pub fn leq_leq<A: Into<Float>, B: Into<AffineExpression>, C: Into<Float>>(
    lhs: A,
    middle: B,
    rhs: C,
) -> Constraint {
    Constraint {
        expression: middle.into(),
        lower_bound: Some(lhs.into()),
        upper_bound: Some(rhs.into()),
        is_equality: false,
    }
}

#[macro_export]
macro_rules! constraint {
    // first segment
    (@first [$($left:tt)*] <= $($tail:tt)*) => {
        constraint!(@second [$($left)*] <= [] $($tail)*)
    };

    (@first [$($left:tt)*] >= $($tail:tt)*) => {
         constraint!(@second [$($left)*] >= [] $($tail)*)
    };

    (@first [$($left:tt)*] == $($tail:tt)*) => {
        constraint!(@second [$($left)*] == [] $($tail)*)
    };

    // stop condition: all token have been processed
    (@first [$($left:tt)*]) => {
        panic!("expected <=, == or >=")
    };

    // the next token is not a special one
    (@first [$($left:tt)*] $next:tt $($tail:tt)*) => {
        constraint!(@first [$($left)* $next] $($tail)*)
    };

    // second segment
    (@second [$($left:tt)*] $c:tt [$($middle:tt)*] <= $($tail:tt)*) => {
        constraint!(@final [$($left)*] $c [$($middle)*] <= [$($tail)*])
    };

    (@second [$($left:tt)*] $c:tt [$($middle:tt)*] >= $($tail:tt)*) => {
        constraint!(@final [$($left)*] $c [$($middle)*] >= [$($tail)*])
    };

    (@second [$($left:tt)*] $c:tt [$($middle:tt)*] == $($tail:tt)*) => {
        constraint!(@final [$($left)*] $c [$($middle)*] == [$($tail)*])
    };

    // stop condition: all tokens have been processed
    (@second [$($left:tt)*] $c:tt [$($middle:tt)*]) => {
        constraint!(@final [$($left)*] $c [$($middle)*])
    };

    // the next token is not a special one
    (@second [$($left:tt)*] $c:tt [$($middle:tt)*] $next:tt $($tail:tt)*) => {
         constraint!(@second [$($left)*] $c [$($middle)* $next] $($tail)*)
    };

    (@final [$($left:tt)*] <= [$($right:tt)*]) => {
        $crate::constraint::leq($($left)*, $($right)*)
    };

    (@final [$($right:tt)*] >= [$($left:tt)*]) => {
        $crate::constraint::leq($($left)*, $($right)*)
    };

    (@final [$($left:tt)*] == [$($right:tt)*]) => {
        $crate::constraint::eq($($left)*, $($right)*)
    };

    (@final [$($lower:tt)*] <= [$($middle:tt)*] <= [$($upper:tt)*]) => {
        $crate::constraint::leq_leq($($lower)*, $($middle)*, $($upper)*)
    };

    (@final [$($upper:tt)*] >= [$($middle:tt)*] >= [$($lower:tt)*]) => {
        $crate::constraint::leq_leq($($lower)*, $($middle)*, $($upper)*)
    };

    (@final $($t:tt)*) => {
        todo!("invalid: {}", stringify!($($t)*))
    };

    // initial rule: start the recursive calls
    ($($all:tt)*) => {
        constraint!(@first [] $($all)*)
    };
}
