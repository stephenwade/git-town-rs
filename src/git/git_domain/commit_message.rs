#[derive(Debug)]
pub struct CommitMessage {
    message: String,
}

impl From<&String> for CommitMessage {
    fn from(item: &String) -> Self {
        CommitMessage {
            message: item.clone(),
        }
    }
}
