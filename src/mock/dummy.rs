use crate::{
    process::Process,
    server::GraphicsServer,
};


#[derive(Default)]
pub struct Dummy {
    active: Vec<()>,
}


impl GraphicsServer for Dummy {
    type Process = ();

    fn spawn_process(&mut self) -> Self::Process {
        self.active.push(());

        ()
    }

    fn active_processes(&self) -> &[Self::Process] {
        &self.active
    }
}

impl Process for () {
    type Info = ();

    fn info(&self) -> Self::Info {
        ()
    }
}
