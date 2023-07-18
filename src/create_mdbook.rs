//mdbook
use crate::message_alert;
use mdbook::config::Config;
use mdbook::MDBook;
use std::{
    fs::{self, File, OpenOptions},
    io::Result,
    io::Write,
};

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
    if let Ok(_) = File::create("./mdBook_html_files/src/SUMMARY.md") {}
}

pub fn build_mdbook() {
    let md = MDBook::load("./mdBook_html_files/");

    match md {
        Ok(md) => match md.build() {
            Ok(_) => {
                // message_alert("static files init sucessfully");
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
