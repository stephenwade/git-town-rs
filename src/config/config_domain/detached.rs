#[derive(Debug)]
pub enum Detached {
    Yes,
    No,
}

impl From<bool> for Detached {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
