use crate::{
    process::Process,
    server::GraphicsServer,
};


pub struct Dummy;


impl GraphicsServer for Dummy {
    type Process = ();

    fn spawn_process(&mut self) -> Self::Process {
        ()
    }
}

impl Process for () {
}
