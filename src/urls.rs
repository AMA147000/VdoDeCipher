use crate::paths;

pub struct URLs {
    pub current: String,
    pub saved: Vec<String>,
}

impl Default for URLs {
    fn default() -> Self {
        Self {
            current: "https://duckduckgo.com".to_string(),
            saved: Vec::new(),
        }
    }
}

impl URLs {
    pub fn read_file() -> Result<Self, String> {
        let url_file = std::fs::read_to_string(paths::urls_file()).or_else(|_| {
            Self::default().write_file()?; // create default file if missing
            std::fs::read_to_string(paths::urls_file())
                .map_err(|e| format!("could not read the URLs file after creating it: {}", e))
        })?;

        let mut lines = url_file.lines();
        let current = lines
            .next()
            .ok_or_else(|| "no URL found in the URLs file")?
            .to_string();
        let saved = lines.map(|l| l.to_string()).collect();

        Ok(Self { current, saved })
    }

    pub fn write_file(self) -> Result<(), String> {
        let content = format!("{}\n{}", self.current, self.saved.join("\n"));
        std::fs::write(paths::urls_file(), content)
            .map_err(|e| format!("could not create/edit the URLs file: {}", e))
    }
}
