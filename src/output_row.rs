pub struct OutputRow {
    pub group_data: Vec<String>,
    pub number_data: Vec<String>,
}

impl OutputRow {
    pub fn new(group_data: Vec<String>, number_data: Vec<String>) -> Self {
        Self {
            group_data,
            number_data,
        }
    }
}
