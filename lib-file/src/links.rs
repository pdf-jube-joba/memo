use std::{io , fs};
use std::error;
use std::path::PathBuf;
use std::io::Write;
use fs::OpenOptions;
use ownlinkmemo_data::links as links;
use ownlinkmemo_data::setting as setting;

const TARGET_DIR: &str = ".";
const DATA_PATH: &str = ".memoData";
const LINKS_PATH: &str = "Links";

fn path_cur() -> PathBuf {
  PathBuf::from(&TARGET_DIR)
}

fn path_data() -> PathBuf {
  path_cur().join(&DATA_PATH)
}

fn path_link() -> PathBuf {
  path_data().join(&LINKS_PATH)
}

fn id_to_fullpath(id: &links::Id) -> PathBuf{
    let mut path = path_link().join(PathBuf::from(id.id()));
    path.set_extension("json");
    path
}

pub fn init() -> io::Result<()>{
    fs::create_dir(path_data())?;
    fs::create_dir(path_link())
}

pub fn listup() -> io::Result<Vec<links::LinkObject>> {
  let mut list: Vec<links::LinkObject> = Vec::new();
  let entries = std::fs::read_dir(path_link())?;
  for entry in entries {
      let entry = entry?;
      if let Ok(name) = entry.file_name().into_string() {
          if let Ok(id) = links::Id::temp(&name) {
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

pub fn search(id_name: &String) -> Result<links::LinkObject , Box<dyn error::Error>> {
    let id = links::Id::temp(id_name)?;
    let content = content_from_fullpath(id_to_fullpath(&id))?;
    Ok(links::LinkObject {id , content})
}

pub fn create(body: links::Body , user: setting::User) -> io::Result<()> {
    let info = links::Info::create(user);
    let content = links::Content::new(info , body);
    let obj = links::LinkObject::new(content);
    let file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(id_to_fullpath(&(obj.id)))?;
    let text = serde_json::to_string(&(obj.content))?;
    write!(&file , "{}" , text)?;
    Ok(())
}