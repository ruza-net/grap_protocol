mod mock;

pub mod server;


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
        setup! { mut dummy }

        let _process = dummy.spawn_process();
    }
}
