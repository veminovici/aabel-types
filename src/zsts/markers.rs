pub struct ReadOnly;
pub struct ReadWrite;

pub struct Database<Access> {
    _connection_string: String,
    _marker: std::marker::PhantomData<Access>,
}

impl<Access> Database<Access> {
    pub fn query(&self, query: &str) -> Vec<String> {
        // Common query logic
        vec![format!("Result of query: {}", query)]
    }
}

impl Database<ReadWrite> {
    pub fn execute(&self, _command: &str) -> Result<(), String> {
        // Only available in read-write mode
        Ok(())
    }
}
