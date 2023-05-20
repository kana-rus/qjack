#![allow(non_camel_case_types)]

use std::{pin::Pin, future::Future};
use futures_util::{TryStreamExt, TryFutureExt, future, StreamExt};
use sqlx::{Executor, Either};
use crate::{Error, model, FetchAll, FetchOne, FetchOptional, q, pool};
use super::{param::Param, transaction::X, FetchQueryOutput};


impl<'q, M:model+'q> FnOnce<(FetchAll<'q, M>,)> for q {
    type Output = FetchQueryOutput<'q, Vec<M>>;
    extern "rust-call" fn call_once(self, (FetchAll { __as__, sql },): (FetchAll<'q, M>,)) -> Self::Output {
        Box::pin(
            Box::pin(
                pool().fetch_many(sql)
                    .try_filter_map(|step| async move {
                        Ok(match step {
                            Either::Left(_)    => None,
                            Either::Right(row) => M::from_row(&row).ok(),
                        })
                    })
            )
            .try_collect()
        )
    }
}

impl<'q, M:model+'q> FnOnce<(FetchAll<'q, M>,)> for X {
    type Output = FetchQueryOutput<'q, Vec<M>>;
    extern "rust-call" fn call_once(mut self, (FetchAll { __as__, sql },): (FetchAll<'q, M>,)) -> Self::Output {
        Box::pin(async move {
            Box::pin(
                self.0.fetch_many(sql)
                    .try_filter_map(|step| async move {
                        Ok(match step {
                            Either::Left(_)    => None,
                            Either::Right(row) => M::from_row(&row).ok(),
                        })
                })
            )
            .try_collect().await
        })
    }
}
impl<'q, M:model+'q> FnMut<(FetchAll<'q, M>,)> for X {
    extern "rust-call" fn call_mut(&mut self, (FetchAll { __as__, sql },): (FetchAll<'q, M>,)) -> Self::Output {
        let output = Box::pin(async move {
            Box::pin(
                self.0.fetch_many(sql)
                    .try_filter_map(|step| async move {
                        Ok(match step {
                            Either::Left(_)    => None,
                            Either::Right(row) => M::from_row(&row).ok(),
                        })
                })
            )
            .try_collect().await
        });
        unsafe {std::mem::transmute::<
            FetchQueryOutput<'_, Vec<M>>,
            FetchQueryOutput<'q, Vec<M>>    
        >(output)}
    }
}

macro_rules! fetch_all_query_with_params {
    ($( $param:ident )+) => {
        impl<'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchAll<'q, M>, $( $param ),+)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchAll{__as__, sql}, $( $param ),+): (FetchAll<'q, M>, $( $param ),+)
            ) -> Self::Output {
                Box::pin(
                    Box::pin(
                        pool().fetch_many(sqlx::query(sql) $( .bind($param) )+)
                            .try_filter_map(|step| async move {
                                Ok(match step {
                                    Either::Left(_)    => None,
                                    Either::Right(row) => M::from_row(&row).ok(),
                                })
                            })
                    )
                    .try_collect()
                )
            }
        }
        // impl<'q, 'x:'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchAll<'q, M>, $( $param ),+)> for &'x mut X {
        //     type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
        //     extern "rust-call" fn call_once(self,
        //         (FetchAll{__as__, sql}, $( $param ),+): (FetchAll<'q, M>, $( $param ),+)
        //     ) -> Self::Output {
        //         Box::pin(
        //             Box::pin(
        //                 self.0.fetch_many(sqlx::query(sql) $( .bind($param) )+)
        //                     .try_filter_map(|step| async move {
        //                         Ok(match step {
        //                             Either::Left(_)    => None,
        //                             Either::Right(row) => M::from_row(&row).ok(),
        //                         })
        //                     })
        //             )
        //             .try_collect()
        //         )
        //     }
        // }
    };
} const _: () = {
    fetch_all_query_with_params!(p1);
    fetch_all_query_with_params!(p1 p2);
    fetch_all_query_with_params!(p1 p2 p3);
    fetch_all_query_with_params!(p1 p2 p3 p4);
    fetch_all_query_with_params!(p1 p2 p3 p4 p5);
    fetch_all_query_with_params!(p1 p2 p3 p4 p5 p6);
    fetch_all_query_with_params!(p1 p2 p3 p4 p5 p6 p7);
};


impl<'q, M:model+'q> FnOnce<(FetchOne<'q, M>,)> for q {
    type Output = Pin<Box<dyn Future<Output = Result<M, Error>> + 'q>>;
    extern "rust-call" fn call_once(self, (FetchOne { __as__, sql },): (FetchOne<'q, M>,)) -> Self::Output {
        Box::pin(
            pool().fetch_optional(sqlx::query(sql))
                .and_then(|row| match row {
                    Some(row) => match M::from_row(&row) {
                        Ok(m)  => future::ok(m),
                        Err(e) => future::err(e),
                    },
                    None => future::err(Error::RowNotFound),
                })
        )
    }
}
// impl<'q, 'x:'q, M:model+'q> FnOnce<(FetchOne<'q, M>,)> for &'x mut X {
//     type Output = Pin<Box<dyn Future<Output = Result<M, Error>> + 'q>>;
//     extern "rust-call" fn call_once(self, (FetchOne { __as__, sql },): (FetchOne<'q, M>,)) -> Self::Output {
//         Box::pin(
//             self.0.fetch_optional(sqlx::query(sql))
//                 .and_then(|row| match row {
//                     Some(row) => match M::from_row(&row) {
//                         Ok(m)  => future::ok(m),
//                         Err(e) => future::err(e),
//                     },
//                     None => future::err(Error::RowNotFound),
//                 })
//         )
//     }
// }
macro_rules! fetch_one_query_with_params {
    ($( $param:ident )+) => {
        impl<'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchOne<'q, M>, $( $param ),+)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<M, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchOne{__as__, sql}, $( $param ),+): (FetchOne<'q, M>, $( $param ),+)
            ) -> Self::Output {
                Box::pin(
                    pool().fetch_optional(sqlx::query(sql) $(.bind($param))+)
                        .and_then(|row| match row {
                            Some(row) => match M::from_row(&row) {
                                Ok(m)  => future::ok(m),
                                Err(e) => future::err(e),
                            },
                            None => future::err(Error::RowNotFound),
                        })
                )
            }
        }
        // impl<'q, 'x:'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchOne<'q, M>, $( $param ),+)> for &'x mut X {
        //     type Output = Pin<Box<dyn Future<Output = Result<M, Error>> + 'q>>;
        //     extern "rust-call" fn call_once(self,
        //         (FetchOne{__as__, sql}, $( $param ),+): (FetchOne<'q, M>, $( $param ),+)
        //     ) -> Self::Output {
        //         Box::pin(
        //             self.0.fetch_optional(sqlx::query(sql) $(.bind($param))+)
        //                 .and_then(|row| match row {
        //                     Some(row) => match M::from_row(&row) {
        //                         Ok(m)  => future::ok(m),
        //                         Err(e) => future::err(e),
        //                     },
        //                     None => future::err(Error::RowNotFound),
        //                 })
        //         )
        //     }
        // }
    };
} const _: () = {
    fetch_one_query_with_params!(p1);
    fetch_one_query_with_params!(p1 p2);
    fetch_one_query_with_params!(p1 p2 p3);
    fetch_one_query_with_params!(p1 p2 p3 p4);
    fetch_one_query_with_params!(p1 p2 p3 p4 p5);
    fetch_one_query_with_params!(p1 p2 p3 p4 p5 p6);
    fetch_one_query_with_params!(p1 p2 p3 p4 p5 p6 p7);
};

impl<'q, M:model+'q> FnOnce<(FetchOptional<'q, M>,)> for q {
    type Output = Pin<Box<dyn Future<Output = Result<Option<M>, Error>> + 'q>>;
    extern "rust-call" fn call_once(self, (FetchOptional { __as__, sql },): (FetchOptional<'q, M>,)) -> Self::Output {
        Box::pin(
            pool().fetch_optional(sqlx::query(sql))
                .and_then(|row| match row {
                    Some(r) => match M::from_row(&r) {
                        Ok(m)  => future::ok(Some(m)),
                        Err(e) => future::err(e)
                    }
                    None => future::ok(None)
                })
        )
    }
}
// impl<'q, 'x:'q, M:model+'q> FnOnce<(FetchOptional<'q, M>,)> for &'x mut X {
//     type Output = Pin<Box<dyn Future<Output = Result<Option<M>, Error>> + 'q>>;
//     extern "rust-call" fn call_once(self, (FetchOptional { __as__, sql },): (FetchOptional<'q, M>,)) -> Self::Output {
//         Box::pin(
//             self.0.fetch_optional(sqlx::query(sql))
//                 .and_then(|row| match row {
//                     Some(r) => match M::from_row(&r) {
//                         Ok(m)  => future::ok(Some(m)),
//                         Err(e) => future::err(e)
//                     }
//                     None => future::ok(None)
//                 })
//         )
//     }
// }
macro_rules! fetch_optional_query_with_params {
    ($( $param:ident )+) => {
        impl<'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchOptional<'q, M>, $( $param ),+)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<Option<M>, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchOptional{__as__, sql}, $( $param ),+): (FetchOptional<'q, M>, $( $param ),+)
            ) -> Self::Output {
                Box::pin(
                    pool().fetch_optional(sqlx::query(sql) $( .bind($param) )+)
                        .and_then(|row| match row {
                            Some(r) => match M::from_row(&r) {
                                Ok(m)  => future::ok(Some(m)),
                                Err(e) => future::err(e)
                            }
                            None => future::ok(None)
                        })
                )
            }
        }
        // impl<'q, 'x:'q, M:model+'q, $( $param:Param<'q> ),+> FnOnce<(FetchOptional<'q, M>, $( $param ),+)> for &'x mut X {
        //     type Output = Pin<Box<dyn Future<Output = Result<Option<M>, Error>> + 'q>>;
        //     extern "rust-call" fn call_once(self,
        //         (FetchOptional{__as__, sql}, $( $param ),+): (FetchOptional<'q, M>, $( $param ),+)
        //     ) -> Self::Output {
        //         Box::pin(
        //             self.0.fetch_optional(sqlx::query(sql) $( .bind($param) )+)
        //                 .and_then(|row| match row {
        //                     Some(r) => match M::from_row(&r) {
        //                         Ok(m)  => future::ok(Some(m)),
        //                         Err(e) => future::err(e)
        //                     }
        //                     None => future::ok(None)
        //                 })
        //         )
        //     }
        // }
    };
} const _: () = {
    fetch_optional_query_with_params!(p1);
    fetch_optional_query_with_params!(p1 p2);
    fetch_optional_query_with_params!(p1 p2 p3);
    fetch_optional_query_with_params!(p1 p2 p3 p4);
    fetch_optional_query_with_params!(p1 p2 p3 p4 p5);
    fetch_optional_query_with_params!(p1 p2 p3 p4 p5 p6);
    fetch_optional_query_with_params!(p1 p2 p3 p4 p5 p6 p7);
};
