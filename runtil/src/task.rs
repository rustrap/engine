use std::sync::Mutex;

use futures::future::BoxFuture;

use crate::actor::MainMarker;

#[derive(Clone)]
pub struct AsyncTask {
    fut: Mutex<Option<BoxFuture<'static, ()>>>,
}

impl AsyncTask {
    pub fn new(fut: BoxFuture<'static, ()>) -> Self {
        AsyncTask {
            fut: Mutex::new(Some(fut)),
        }
    }
}

#[derive(Clone)]
pub struct MainTask {
    pub(crate) f: Box<dyn Fn(MainMarker) -> ()>,
}
