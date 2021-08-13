mod mock;

pub mod server;


#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
        use super::mock::Dummy;
        use super::server::GraphicsServer;

        let _dummy = Box::new(Dummy) as Box<dyn GraphicsServer>;
    }
}
