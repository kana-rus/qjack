#![allow(non_upper_case_globals)]
use std::marker::PhantomData;
use sqlx::FromRow;
use crate::__feature__;

pub struct FetchAll<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}
pub struct FetchOne<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}
pub struct FetchOptional<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}

#[allow(non_camel_case_types)]
pub trait Model: for<'r> FromRow<'r, __feature__::Row> {
    fn all<'q>(sql: &'q str) -> FetchAll<'q, Self> {
        FetchAll { sql, __as__:PhantomData }
    }
    fn one<'q>(sql: &'q str) -> FetchOne<'q, Self> {
        FetchOne { sql, __as__:PhantomData }
    }
    fn optional<'q>(sql: &'q str) -> FetchOptional<'q, Self> {
        FetchOptional { sql, __as__:PhantomData }
    }
}