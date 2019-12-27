// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
trait MyClosure {
    type Args;
}

impl<R> MyClosure for dyn FnMut() -> R
where R: 'static {
    type Args = ();
}

struct MyStream<C: ?Sized + MyClosure> {
    x: C::Args,
}

async fn get_future<C: ?Sized + MyClosure>(_stream: MyStream<C>) {}

async fn f() {
    let messages: MyStream<dyn FnMut()> = unimplemented!();
    get_future(messages).await;
}

fn main() {}
