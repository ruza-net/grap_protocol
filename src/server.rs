use crate::process::Process;


pub trait GraphicsServer {
    type Process: Process;

    fn spawn_process(&mut self) -> Self::Process;
}
