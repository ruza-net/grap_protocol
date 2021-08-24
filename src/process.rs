pub trait Process {
    type Info;

    fn info(&self) -> Self::Info;
}

#[derive(Debug)]
pub enum Error {}
