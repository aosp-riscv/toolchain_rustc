// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use std::sync::Arc;

trait SomeTrait: Send + Sync + 'static {
    fn do_something(&self);
}

async fn my_task(obj: Arc<dyn SomeTrait>) {
    unimplemented!()
}

fn main() {}
