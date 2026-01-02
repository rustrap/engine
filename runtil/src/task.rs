use std::sync::Mutex;

use futures::future::BoxFuture;

pub struct Task {
    fut: Mutex<Option<BoxFuture<'static, ()>>>,
}

pub trait Task {
    fn post(&self, task: Task);
    fn post_future(&self, fut: impl Future<Output = ()>);
}
