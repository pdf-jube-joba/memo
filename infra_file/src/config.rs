use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub target_dir: String,
}

impl Config {
    pub fn path_current(self: &Self) -> PathBuf {
        PathBuf::from(self.target_dir.clone())
    }
    pub fn path_data(self: &Self) -> PathBuf {
        self.path_current().join(PathBuf::from(".memoData"))
    }
    pub fn path_link(self: &Self) -> PathBuf {
        self.path_data().join(PathBuf::from("Links"))
    }
    pub fn path_word(self: &Self) -> PathBuf {
        self.path_data().join(PathBuf::from("Words"))
    }
}

pub fn init(config: &Config) -> std::io::Result<()>{
    std::fs::create_dir(config.path_data())?;
    std::fs::create_dir(config.path_link())?;
    std::fs::create_dir(config.path_word())
}