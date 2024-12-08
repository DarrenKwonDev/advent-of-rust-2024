pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let matching_logs = self.search(keyword);
        
        let mut file = File::create(path)?;
        
        for log in matching_logs {
            writeln!(file, "{}", log)?;
        }
        
        Ok(())
    }
}


fn main() {
    
}
