pub struct OutputRow {
    pub group_data: Vec<String>,
    pub stats_data: Vec<String>,
}

impl OutputRow {
    pub fn new(group_data: Vec<String>, stats_data: Vec<String>) -> Self {
        Self {
            group_data,
            stats_data,
        }
    }
}
