use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

struct MdFile {
    path: String,
}

impl MdFile {
    fn new(path: String) -> Self {
        Self { path: path }
    }

    fn to_html(self, output_dir: &str) -> Result<()> {
        println!("{}", self.path);

        if !Path::new(output_dir).exists() {
            fs::create_dir_all(output_dir)?;
        }

        let file_stem = Path::new(&self.path)
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap();

        let html_file_path = Path::new(output_dir).join(format!("{}.html", file_stem));

        let md_content = fs::read_to_string(&self.path)?;
        let mut html_content = String::new();
        let parser = Parser::new_ext(&md_content, Options::empty());
        html::push_html(&mut html_content, parser);

        let mut html_file = File::create(html_file_path)?;
        write!(
            html_file,
            "<html><head><title>{}</title></head><body>{}</body></html>",
            file_stem, html_content
        )?;

        Ok(())
    }
}

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    let md = MdFile::new(file_name);
    let output_dir = args[2].clone();
    md.to_html(&output_dir)
}
