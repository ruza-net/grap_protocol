use std::hash::Hash;


pub trait Process: Eq {
    type Info: ProcessInfo;

    fn info(&self) -> Self::Info;
}

pub trait ProcessInfo {
    type Id: Hash + Eq;

    fn id(&self) -> Self::Id;
}

#[derive(Debug)]
pub enum Error {
    UnknownProcess,
}
