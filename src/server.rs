pub trait GraphicsServer {
    type Process;

    fn spawn_process(&mut self) -> Self::Process;
}
