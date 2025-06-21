#[derive(Debug)]
pub struct CommitMessage(String);

impl From<&String> for CommitMessage {
    fn from(item: &String) -> Self {
        CommitMessage(item.clone())
    }
}
