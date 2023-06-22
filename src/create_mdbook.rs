//mdbook
use mdbook::config::Config;
use mdbook::MDBook;
use std::{
    fs::{self, OpenOptions},
    io::Result,
    io::Write,
};

use crate::message_alert;

pub fn create_mdbook() {
    let _ = fs::create_dir("./mdBook_html_files/");
    let root_dir = "./mdBook_html_files/";

    let mut cfg = Config::default();

    cfg.book.title = Some("메시지 규격".to_string());

    MDBook::init(root_dir)
        .create_gitignore(true)
        .with_config(cfg)
        .build()
        .expect("Book generation failed");

    let _ = fs::remove_file("./mdBook_html_files/src/SUMMARY.md");
}

pub fn build_mdbook() {
    // write book.toml "preprocessor.plantuml"

    add_book_toml();

    let mut md = MDBook::load("./mdBook_html_files/");

    match md {
        Ok(md) => match md.build() {
            Ok(_) => {
                message_alert("html file created sucessfully");
            }
            Err(err) => {
                message_alert(&err.to_string());
            }
        },
        Err(err) => {
            message_alert(&err.to_string());
        }
    }
}

pub fn add_book_toml() -> Result<()> {
    let book_toml = OpenOptions::new()
        .append(true)
        .open("./mdBook_html_files/book.toml");

    match book_toml {
        Ok(mut toml) => {
            writeln!(toml, "[preprocessor.plantuml]")?;
            writeln!(toml, "command = \"mdbook-plantuml\"")?;
        }
        Err(e) => {
            message_alert(&e.to_string());
        }
    }

    Ok(())
}
