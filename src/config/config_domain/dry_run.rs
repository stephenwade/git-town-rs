#[derive(Debug)]
pub enum DryRun {
    Yes,
    No,
}

impl From<bool> for DryRun {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
