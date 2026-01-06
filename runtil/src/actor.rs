use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::runloop::MAIN_THREAD_ID;

#[derive(Debug, Clone)]
pub enum MainActorError {
    NotInMainThread,
    LockError,
}

pub type MainActorResult<T> = Result<T, MainActorError>;

fn lock_result<T>(val: LockResult<T>) -> MainActorResult<T> {
    match val {
        LockResult::Ok(v) => Ok(v),
        LockResult::Err(..) => Err(MainActorError::LockError),
    }
}

/// Check thread id at runtime.
pub struct MainActor<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> MainActor<T> {
    pub fn new(value: T) -> MainActorResult<Self> {
        Self::check_main()?;
        Ok(MainActor {
            inner: Arc::new(value.into()),
        })
    }

    fn check_main() -> MainActorResult<()> {
        let thread_id_current = std::thread::current().id();
        if thread_id_current != *MAIN_THREAD_ID.get().unwrap() {
            Err(MainActorError::NotInMainThread)
        } else {
            Ok(())
        }
    }

    pub fn read(&self) -> MainActorResult<RwLockReadGuard<'_, T>> {
        Self::check_main()?;
        lock_result(self.inner.read())
    }

    pub fn write(&self) -> MainActorResult<RwLockWriteGuard<'_, T>> {
        Self::check_main()?;
        lock_result(self.inner.write())
    }
}

//TODO: MainMarker and MainActor with comptime check

#[cfg(test)]
mod test {
    use super::*;
    use crate::{RunLoop, RunLoopHandler, UserMessage};

    struct Message();
    impl UserMessage for Message {}
    struct Handler();
    impl RunLoopHandler<Message> for Handler {}
    struct Data();

    #[test]
    fn main_actor() {
        let _runloop = RunLoop::new(Handler());
        let a = MainActor::new(Data()).expect("it is not same as a thread that runloop born");
        let thread = std::thread::spawn(move || {
            assert_eq!(a.read().is_err(), true);
        });
        thread.join().unwrap();
    }
}
