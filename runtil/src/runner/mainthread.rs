use crossbeam::queue::SegQueue;

use crate::task::Task;

pub struct MainThreadRunner {
    tasks: SegQueue<Task>,
}

impl MainThreadRunner {
    pub fn new() -> Self {
        MainThreadRunner {
            tasks: SegQueue::new(),
        }
    }
}
