use crate::{hashing::hash_file,manifest::Manifest};use std::path::Path;
pub struct ResultSet{pub unchanged:usize,pub modified:Vec<String>,pub missing:Vec<String>}
pub fn verify(m:&Manifest)->Result<ResultSet,String>{let root=Path::new(&m.root);let mut r=ResultSet{unchanged:0,modified:vec![],missing:vec![]};for f in &m.files{let p=root.join(&f.path);if !p.is_file(){r.missing.push(f.path.clone());continue}let hash=hash_file(&p,m.algorithm).map_err(|e|e.to_string())?;if f.hash.as_deref()==Some(&hash){r.unchanged+=1}else{r.modified.push(f.path.clone())}}Ok(r)}
