use crossbeam::queue::SegQueue;

use crate::{driver::EventPumpImpl, task::MainTask};

pub struct MainThreadRunner {
    tasks: SegQueue<MainTask>,
    pump: EventPumpImpl,
}

impl MainThreadRunner {
    pub fn new() -> Self {
        MainThreadRunner {
            pump: EventPumpImpl::new(),
            tasks: SegQueue::new(),
        }
    }

    pub fn schedule_task(&self) {}
}
