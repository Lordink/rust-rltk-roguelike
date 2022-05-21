
pub struct GameLog {
    pub entries: Vec<String>
}

impl GameLog {
    pub fn log(&mut self, str: String) {
        println!("{}", str);
        self.entries.push(str);
    }
}