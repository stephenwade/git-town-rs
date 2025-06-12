#[derive(Debug)]
pub enum Beam {
    Yes,
    No,
}

impl From<bool> for Beam {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
