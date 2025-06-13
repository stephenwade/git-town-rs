#[derive(Debug)]
pub enum Commit {
    Yes,
    No,
}

impl From<bool> for Commit {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
