use std::collections::HashSet;

use uuid::Uuid;

use crate::{process::{Error, Process, ProcessInfo}, server::GraphicsServer};


#[derive(Default)]
pub struct Dummy {
    processes: HashSet<DummyInfo>,
}


impl GraphicsServer for Dummy {
    type Process = DummyProcess;

    fn spawn_process(&mut self) -> Self::Process {
        let info =
            DummyInfo {
                id: Uuid::new_v4(),
            };

        self.processes.insert(info);

        DummyProcess {
            info,
        }
    }

    fn halt_process(&mut self, proc: Self::Process) -> Result<<Self::Process as Process>::Info, crate::process::Error> {
        if self.processes.remove(&proc.info()) {
            Ok(proc.info())

        } else {
            Err(Error::UnknownProcess)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DummyProcess {
    info: DummyInfo,
}

impl Process for DummyProcess {
    type Info = DummyInfo;

    fn info(&self) -> Self::Info {
        self.info
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DummyInfo {
    id: Uuid,
}

impl ProcessInfo for DummyInfo {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }
}
