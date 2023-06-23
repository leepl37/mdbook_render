use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use crate::message_alert;

fn md_file_name(name: &str) -> String {
    let mut name = name.replace('#', "");
    name = name.replace(' ', "");
    name = name.replace(':', "_");
    name = name.replace('[', "");
    name.replace(']', "").replace('/', "_")
}

fn summary_name(name: &str) -> String {
    let md_file_name = md_file_name(&name);
    if name.contains('#') {
        let cnt = get_line_marker_cnt(&name);
        let mut name = name.replace('#', "");

        name = name.trim().split_once(' ').unwrap().1.to_string();
        let mut tab = "".to_string();
        for _ in 1..cnt {
            tab.push('\t');
        }

        return format!("{}- [{}](./{}.md)", tab, name, md_file_name);
    }

    format!("[ {} ](./{}.md", name, md_file_name)
}

fn summary_write_append(contents: &str) {
    // let _ = fs::create_dir_all("./mdBook_html_files/src");

    let mut summary = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./mdBook_html_files/src/SUMMARY.md");

    match summary {
        Ok(mut summary_file) => {
            writeln!(summary_file, "{}", contents).expect("Failed to write to the file");
            summary_file.flush().expect("Failed to flush the file");
        }
        Err(err) => {
            message_alert(&err.to_string());
        }
    }
}

fn file_write(content: &String) {
    let subject = content.lines().next().to_owned().unwrap();
    summary_write_append(&summary_name(subject));

    let subject = md_file_name(subject);

    let create_file_path_name = format!("./mdBook_html_files/src/{}.md", subject);
    let mut file = match File::create(create_file_path_name) {
        Ok(file) => file,
        Err(err) => {
            message_alert(&err.to_string());
            return;
        }
    };

    if let Err(err) = file.write_all(content.as_bytes()) {
        eprintln!("Failed to write to the file: {}", err);
        message_alert(&err.to_string());
    } else {
        // println!("Successfully wrote to the file.");
    }
}

fn get_line_marker_cnt(line: &str) -> u8 {
    let mut cnt = 0;
    for n in line.chars() {
        if n == '#' {
            cnt += 1;
        }
    }
    cnt
}

fn append_content(line: String, path: String) {
    let last_index = get_last_index(path.clone());
    // let file = File::open(path).unwrap();

    // let mut file = File::open(path);

    match File::open(path) {
        Ok(file) => {
            let buf = BufReader::new(file);

            let mut can_append: bool = false;
            let mut contents = "".to_string();

            let first_marker_cnt = get_line_marker_cnt(&line);

            let mut the_point_of_add_line = 0;
            let mut buf_line_index = 0;
            for (i, n) in buf.lines().enumerate() {
                let buf_line = n.unwrap();

                buf_line_index = i;
                if buf_line.contains(&line) {
                    can_append = true;
                    the_point_of_add_line = i;
                    contents.push_str(&buf_line);
                    contents.push('\n');
                }

                if can_append && i > the_point_of_add_line {
                    if buf_line.contains('#')
                        && buf_line.starts_with('#')
                        && get_line_marker_cnt(&buf_line) <= first_marker_cnt
                    {
                        file_write(&contents);
                        break;
                    }

                    // if buf_line.contains("plantuml" .. 뒷자리 다 자르기. uml 에러남
                    match buf_line.contains("@startuml") {
                        true => {
                            contents.push_str("@startuml");
                        }
                        false => {
                            contents.push_str(&buf_line);
                        }
                    }
                    contents.push('\n');
                }
            }

            println!(
                "buf_line_index : {}, last_index :{}",
                buf_line_index, last_index
            );

            if buf_line_index == last_index {
                file_write(&contents);
            }
        }
        Err(err) => {
            message_alert(&err.to_string());
        }
    };
}

pub fn file_read(path: String) {
    message_alert("file read");
    let mut does_first_line_marker_not_contain = false;
    let mut contents = "".to_string();

    if let Ok(file) = File::open(path.clone()) {
        let reader = BufReader::new(file);
        for (i, n) in reader.lines().enumerate() {
            let line = n.unwrap();
            if line.contains('#') && line.starts_with('#') {
                if does_first_line_marker_not_contain {
                    // file_write(&contents);
                    does_first_line_marker_not_contain = false;
                }

                append_content(line.clone(), path.clone());
            } else if i == 0 {
                does_first_line_marker_not_contain = true;
            }
            if does_first_line_marker_not_contain {
                contents.push_str(&line);
                contents.push('\n');
            }
        }
    } else {
    }
}

fn get_last_index(path: String) -> usize {
    let file = File::open(path).unwrap();

    BufReader::new(file).lines().count() - 1
}
