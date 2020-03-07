// use diesel::mysql::Mysql;
use diesel::sql_types::{BigInt, Integer};
use diesel::expression::{Expression, AsExpression};

// src/expression/operators.rs
// src/expression_methods/text_expression_methods.rs

// diesel_infix_operator!(Add, " @@ ", backend: Mysql);
// 当然最好使用 重载运算符实现参考
// src/sql_types/ops.rs
// src/expression/ops/mod.rs
// src/expression/ops/numeric.rs
// https://docs.diesel.rs/diesel/sql_types/ops/index.html
diesel_infix_operator!(MyBitAnd, " & ", ReturnBasedOnArgs);
diesel_infix_operator!(MyBitOr, " | ", ReturnBasedOnArgs);

pub trait BigIntBitOperatorExtensions: Expression + Sized {

    fn bit_and<T: AsExpression<Self::SqlType>>(self, other: T) -> MyBitAnd<Self, T::Expression> {
        MyBitAnd::new(self, other.as_expression())
    }

    fn bit_or<T: AsExpression<Self::SqlType>>(self, other: T) -> MyBitOr<Self, T::Expression> {
        MyBitOr::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType=BigInt>> BigIntBitOperatorExtensions for T {
}

pub trait IntegerBitOperatorExtensions: Expression + Sized {

    fn bit_and<T: AsExpression<Self::SqlType>>(self, other: T) -> MyBitAnd<Self, T::Expression> {
        MyBitAnd::new(self, other.as_expression())
    }

    fn bit_or<T: AsExpression<Self::SqlType>>(self, other: T) -> MyBitOr<Self, T::Expression> {
        MyBitOr::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType=Integer>> IntegerBitOperatorExtensions for T {
}
