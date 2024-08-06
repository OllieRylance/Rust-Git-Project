pub struct CommitDTO {
    pub message: String,
    pub timestamp: String,
    pub author: String,
    pub changes: Vec<String>,
}