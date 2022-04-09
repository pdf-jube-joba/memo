use std::{io , fs};
use std::error;
use std::path::PathBuf;
use std::io::Write;
use fs::OpenOptions;
use ownlinkmemo_domain as domain;
use domain::links as links;
use domain::setting as setting;
use crate::config;

fn link_id_to_fullpath(config: &config::Config , id: &setting::Id) -> PathBuf{
    let mut path = config.path_link().join(PathBuf::from(id.id()));
    path.set_extension("json");
    path
}

pub fn listup(config: &config::Config) -> io::Result<Vec<links::LinkObject>> {
  let mut list: Vec<links::LinkObject> = Vec::new();
  let entries = std::fs::read_dir(config.path_link())?;
  for entry in entries {
      let entry = entry?;
      if let Ok(name) = entry.file_name().into_string() {
          if let Ok(id) = setting::Id::id_or(&name) {
              let content = content_from_fullpath(entry.path())?;
              list.push(links::LinkObject{id , content});
          }
      }
  }
  Ok(list)
}

fn content_from_fullpath(path: PathBuf) -> io::Result<links::Content> {
  let s = fs::read_to_string(path)?;
  let content: links::Content = serde_json::from_str(&s)?;
  Ok(content)
}

pub fn search(config: &config::Config , id_name: &String) -> Result<links::LinkObject , Box<dyn error::Error>> {
    let id = setting::Id::id_or(id_name)?;
    let content = content_from_fullpath(link_id_to_fullpath(config , &id))?;
    Ok(links::LinkObject {id , content})
}

pub fn create(config: &config::Config , body: links::Body , user: setting::User) -> io::Result<()> {
    let content = links::Content::create(user , body);
    let obj = links::LinkObject::create(content);
    let file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(link_id_to_fullpath(config , &(obj.id)))?;
    let text = serde_json::to_string(&(obj.content))?;
    write!(&file , "{}" , text)?;
    Ok(())
}