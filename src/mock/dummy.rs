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

    fn halt_process(&mut self, _proc: &mut Self::Process) -> Result<<Self::Process as Process>::Info, crate::process::Error> {
        self.active.pop();

        Ok(())
    }
}

impl Process for () {
    type Info = ();

    fn info(&self) -> Self::Info {
        ()
    }
}
