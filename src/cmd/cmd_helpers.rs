/// Automatically compiles the long description of Clap commands
/// out of the given short summary and description.
pub fn long(summary: &str, desc: &str) -> String {
    format!("{summary}.\n{desc}")
}
