use std::{io , fs};
use std::error;
use std::path::PathBuf;
use std::io::Write;
use fs::OpenOptions;
use ownlinkmemo_domain as domain;
use crate::config;

fn link_id_to_fullpath(config: &config::Config , id: &domain::setting::Id) -> PathBuf{
    let mut path = config.path_link().join(PathBuf::from(domain::setting::Id::to_string(id)));
    path.set_extension("json");
    path
}

pub fn listup(config: &config::Config) -> io::Result<Vec<domain::link::LinkObject>> {
    let mut list: Vec<domain::link::LinkObject> = Vec::new();
    let entries = std::fs::read_dir(config.path_link())?;
    for entry in entries {
        let entry = entry?;
        if let Ok(name) = entry.file_name().into_string() {
            if let Ok(id) = domain::setting::Id::id_or(&name) {
                let content = content_from_fullpath(entry.path())?;
                list.push(domain::link::LinkObject{id , content});
            }
        }
    }
    Ok(list)
}

fn content_from_fullpath(path: PathBuf) -> io::Result<domain::link::Content> {
    let s = fs::read_to_string(path)?;
    let content: domain::link::Content = serde_json::from_str(&s)?;
    Ok(content)
}

pub fn search(config: &config::Config , id_name: &String) -> Result<domain::link::LinkObject , Box<dyn error::Error>> {
    let id = domain::setting::Id::id_or(id_name)?;
    let content = content_from_fullpath(link_id_to_fullpath(config , &id))?;
    Ok(domain::link::LinkObject {id , content})
}

pub fn create(
    config: &config::Config,
    user: domain::setting::User,
    origin: domain::link::Origin,
    kind: domain::link::Kind,
    name: String,
    content_type: String,
    link: String
) -> io::Result<()> {
    let content = domain::link::Content::new(user, origin, kind, name, content_type, link);
    let obj = domain::link::LinkObject::new(content);
    let file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(link_id_to_fullpath(config , &(obj.id)))?;
    let text = serde_json::to_string(&(obj.content))?;
    write!(&file , "{}" , text)?;
    Ok(())
}

pub fn overwrite(config: &config::Config , id: domain::setting::Id , content: domain::link::Content) -> io::Result<()> {
    let file = OpenOptions::new()
      .write(true)
      .create_new(false)
      .open(link_id_to_fullpath(config , &id))?;
    let text = serde_json::to_string(&content)?;
    write!(&file , "{}" , text)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn name() {
        let config = crate::config::Config {
            target_dir: String::from("../")
        };
        println!("{}" , config.path_link().into_os_string().into_string().unwrap());
        std::fs::read_dir(config.path_link()).unwrap();
    }
}