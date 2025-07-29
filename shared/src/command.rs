pub struct ConnectCommand {
    pub source: String,
    pub target: String,
}

impl ConnectCommand {
    pub fn from_str(v: &[u8]) -> Option<Self> {
        let parts: Vec<&str> = str::from_utf8(v).ok()?.split('|').collect();
        if parts.len() != 2 {
            return None;
        }
        Some(Self {
            source: parts[0].to_string(),
            target: parts[1].to_string(),
        })
    }
    pub fn to_string(&self) -> String {
        format!("{}|{}", self.source, self.target)
    }
}
