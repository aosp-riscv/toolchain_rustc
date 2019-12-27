// Incorrect handling of uninhabited types could cause us to mark generator
// types as entirely uninhabited, when they were in fact constructible. This
// caused us to hit "unreachable" code (illegal instruction on x86).

// run-pass

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// compile-flags: --edition=2018

#![feature(async_await)]

pub enum Uninhabited { }

fn uninhabited_async() -> Uninhabited {
    unreachable!()
}

async fn noop() { }

#[allow(unused)]
async fn contains_never() {
    let error = uninhabited_async();
    noop().await;
    let error2 = error;
}

#[allow(unused)]
async fn overlap_never() {
    let error1 = uninhabited_async();
    noop().await;
    let error2 = uninhabited_async();
    drop(error1);
    noop().await;
    drop(error2);
}

#[allow(unused_must_use)]
fn main() {
    contains_never();
    overlap_never();
=======
// compile-flags: --edition=2018 -Aunused

pub enum Uninhabited { }

fn uninhabited_async() -> Uninhabited {
    unreachable!()
}

async fn noop() { }

async fn contains_never() {
    let error = uninhabited_async();
    noop().await;
    let error2 = error;
}

async fn overlap_never() {
    let error1 = uninhabited_async();
    noop().await;
    let error2 = uninhabited_async();
    drop(error1);
    noop().await;
    drop(error2);
}

#[allow(unused_must_use)]
fn main() {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
