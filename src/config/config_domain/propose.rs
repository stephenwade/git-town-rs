#[derive(Debug)]
pub enum Propose {
    Yes,
    No,
}

impl From<bool> for Propose {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
