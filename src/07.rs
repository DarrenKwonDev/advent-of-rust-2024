pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> LogQuery<'a> {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }
}
