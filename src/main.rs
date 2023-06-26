mod create_mdbook;
mod create_mdfiles;

use std::fs;
use std::process::Command;

//function that write_mdFile
use create_mdfiles::file_read;

//function that related with mdBook
use create_mdbook::*;

//dialog
use native_dialog::FileDialog;
use native_dialog::MessageDialog;
use native_dialog::MessageType;

fn message_alert(str: &str) {
    // MessageDialog::new()
    //     .set_type(MessageType::Info)
    //     .set_title(&str)
    //     .set_text(&str)
    //     .show_alert()
    //     .unwrap();
    println!("{}", str);
}

fn main() {
    message_alert("book create");
    create_mdbook();
    
    //window
    // let file_name = FileDialog::new()
    //     .set_location("~/")
    //     // .add_filter("md", &["md"])
    //     .show_open_single_file();
    // let path = file_name.expect("can not fine").expect("error");
    // file_read(path.to_str().unwrap().to_string());

    message_alert("book read start");

    //linux 
    file_read("../../spMessageSpecification/message_specification/message_specification.md".to_string());
    

    //linux 
    // match fs::copy("./mdbook-plantuml", "./mdBook_html_files/mdbook-plantuml") {
    // window
    // match fs::copy("./mdbook-plantuml.exe", "./mdBook_html_files/mdbook-plantuml.exe") {
    //     Err(err) => {
    //         println!("mdbook-plantuml can not find");
    //         message_alert(&err.to_string());
    //     }
    //    Ok(t) => {
    //         message_alert(&t.to_string());
    //     }
    // }

    message_alert("copy is done");

    build_mdbook();

    message_alert("done");
}
