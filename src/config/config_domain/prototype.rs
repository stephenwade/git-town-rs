#[derive(Debug)]
pub enum Prototype {
    Yes,
    No,
}

impl From<bool> for Prototype {
    fn from(item: bool) -> Self {
        if item { Self::Yes } else { Self::No }
    }
}
