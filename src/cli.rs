use clap::{Parser, Subcommand, ValueEnum};use std::path::PathBuf;
#[derive(Parser)]#[command(name="filesentry",version,about="Read-only file integrity and duplicate detection")]
pub struct Cli{#[command(subcommand)]pub command:Command}
#[derive(Subcommand)]pub enum Command{Hash{file:PathBuf,#[arg(long,value_enum,default_value_t=Algorithm::Sha256)]algorithm:Algorithm},Scan{path:PathBuf,#[arg(long)]hidden:bool,#[arg(long,value_enum,default_value_t=Algorithm::Sha256)]algorithm:Algorithm,#[arg(long)]json:bool,#[arg(long="ignore")]ignore:Vec<String>},Duplicates{path:PathBuf,#[arg(long)]hidden:bool,#[arg(long="ignore")]ignore:Vec<String>},Manifest{path:PathBuf,#[arg(long,default_value="manifest.json")]output:PathBuf,#[arg(long,value_enum,default_value_t=Algorithm::Sha256)]algorithm:Algorithm,#[arg(long="ignore")]ignore:Vec<String>},Verify{manifest:PathBuf}}
#[derive(Clone,Copy,ValueEnum,serde::Serialize,serde::Deserialize)]#[serde(rename_all="lowercase")]pub enum Algorithm{Sha256,Sha512}
impl Algorithm{pub fn name(self)->&'static str{match self{Self::Sha256=>"sha256",Self::Sha512=>"sha512"}}}
