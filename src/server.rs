use crate::process::{ self, Process };


pub trait GraphicsServer {
    type Process: Process;

    fn spawn_process(&mut self) -> Self::Process;
    fn halt_process(&mut self, proc: Self::Process) -> Result<<Self::Process as Process>::Info, process::Error>;
}
