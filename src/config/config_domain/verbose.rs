#[derive(Debug)]
pub enum Verbose {
    Yes,
    No,
}

impl From<bool> for Verbose {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
