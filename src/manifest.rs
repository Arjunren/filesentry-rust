use crate::{cli::Algorithm,scanner::FileRecord};use serde::{Deserialize,Serialize};use std::{fs,path::Path};
#[derive(Serialize,Deserialize)]pub struct Manifest{pub version:u8,pub root:String,pub algorithm:Algorithm,pub created_at:String,pub files:Vec<FileRecord>}
pub fn write(path:&Path,m:&Manifest)->Result<(),String>{fs::write(path,serde_json::to_vec_pretty(m).map_err(|e|e.to_string())?).map_err(|e|e.to_string())}pub fn read(path:&Path)->Result<Manifest,String>{serde_json::from_slice(&fs::read(path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())}
