pub struct RunLoop {}

pub trait RunLoopHandler {
    fn on_init(&mut self) {}
    fn on_quit(&mut self) {}
}
