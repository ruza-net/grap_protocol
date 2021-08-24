mod mock;

pub mod server;
pub mod process;

macro_rules! setup {
    ( $dummy:ident ) => {
        let $dummy = Box::new(Dummy::default()) as Box<dyn GraphicsServer<Process = DummyProcess>>;
    };

    ( mut $dummy:ident ) => {
        let mut $dummy = Box::new(Dummy::default()) as Box<dyn GraphicsServer<Process = DummyProcess>>;
    };
}

macro_rules! setup_process {
    ( $dummy:ident => $process:ident ) => {
        setup! { mut $dummy }

        let $process = $dummy.spawn_process();
    };

    ( $dummy:ident => mut $process:ident ) => {
        setup! { mut $dummy }

        let mut $process = $dummy.spawn_process();
    };
}

use crate::{
    mock::dummy::{
        Dummy,
        DummyInfo,
        DummyProcess,
    },

    server::GraphicsServer,

    process::{
        Process,
        ProcessInfo,
    },
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        setup! { _dummy }
    }

    #[test]
    fn spawn_process() {
        setup_process! { dummy => process }

        assert_eq![ process.info(), process.info() ];
    }

    #[test]
    fn process_id() {
        setup_process! { dummy => process }

        process.info().id();
    }

    #[test]
    fn halt_process() {
        setup_process! { dummy => process }

        dummy.halt_process(process).expect("dummy should halt");
    }

    #[test]
    fn two_processes_distinct() {
        setup! { mut dummy }

        let process_1 = dummy.spawn_process();
        let process_2 = dummy.spawn_process();

        assert![ process_1 != process_2 ];
    }

    #[test]
    fn same_info_same_process() {
        setup_process! { dummy => process_1 }

        let process_2 = process_1.clone();

        assert_eq![ process_1.info(), process_2.info() ];
        assert_eq![ process_1, process_2 ];
    }

    #[test]
    fn ids_dont_clash() {
        setup_process! { server_1 => process_1 }
        setup_process! { server_2 => process_2 }

        assert![ process_1 != process_2 ];
    }

    #[test]
    fn server_rejects_foreign_process() {
        setup! { mut server_1 }
        setup_process! { server_2 => process_2 }

        server_1.halt_process(process_2).expect_err("server should reject foreign process");

        // So that a simple counter isn't sufficient.
        //
        let _process_1 = server_1.spawn_process();
        let process_2 = server_2.spawn_process();

        server_1.halt_process(process_2).expect_err("server should reject foreign process");

    }
}
