use crossbeam::queue::SegQueue;

use crate::task::Task;

pub struct UIThreadRunner {
    tasks: SegQueue<Task>,
    imp: UIThreadRunnerImpl,
}

impl UIThreadRunner {
    pub fn new() -> Self {}
}
