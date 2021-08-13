pub trait Process {
    type Info;

    fn info(&self) -> Self::Info;
}
