mod mock;

pub mod server;
pub mod process;


#[cfg(test)]
mod tests {
    macro_rules! setup {
        ( $dummy:ident ) => {
            use super::mock::Dummy;
            use super::server::GraphicsServer;

            let $dummy = Box::new(Dummy) as Box<dyn GraphicsServer<Process = ()>>;
        };

        ( mut $dummy:ident ) => {
            use super::mock::Dummy;
            use super::server::GraphicsServer;

            let mut $dummy = Box::new(Dummy) as Box<dyn GraphicsServer<Process = ()>>;
        };
    }

    #[test]
    fn dummy() {
        setup! { _dummy }
    }

    #[test]
    fn spawn_process() {
        use crate::process::Process;

        setup! { mut dummy }

        let process = dummy.spawn_process();

        assert_eq![(), process.info()];
    }
}
