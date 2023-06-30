use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    process::Command,
};

use rand::Rng;

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
    let _ = fs::create_dir_all("./mdBook_html_files/src");

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
            println!("summary file does not exist");
            message_alert(&err.to_string());
        }
    }
}

fn file_write(content: &String, img_file_names: Option<Vec<String>>) {
    let subject = content.lines().next().to_owned().unwrap();
    summary_write_append(&summary_name(subject));

    let subject = md_file_name(subject);

    let create_file_path_name = format!("./mdBook_html_files/src/{}.md", subject);

    let mut file = match File::create(create_file_path_name.clone()) {
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
        create_uml_image(create_file_path_name, img_file_names);
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

    let mut img_files: Vec<String> = vec![];
    // let mut file = File::open(path);
    match File::open(path) {
        Ok(file) => {
            let buf = BufReader::new(file);

            let mut can_append: bool = false;
            let mut contents = "".to_string();

            let first_marker_cnt = get_line_marker_cnt(&line);
            let mut uml_cnt = 0;
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
                        if img_files.is_empty() {
                            file_write(&contents, None);
                        } else {
                            file_write(&contents, Some(img_files));
                        }
                        break;
                    }

                    if buf_line.contains("```plantuml") {
                        println!("contains plantuml");
                        let image_name = format!("{}.svg", generate_random_hash());
                        let mut file_name =
                            md_file_name(contents.lines().next().to_owned().unwrap());

                        if uml_cnt > 0 {
                            file_name = format!("{}_00{}.svg", file_name, uml_cnt);
                        } else {
                            file_name = format!("{}.svg", file_name);
                        }
                        img_files.push(image_name.clone());
                        contents
                            .push_str(&format!("<img src=./{}>", &file_name));
                        contents.push('\n');
                        uml_cnt += 1;
                    } else {
                        match buf_line.contains("@startuml") {
                            true => {
                                contents.push_str("@startuml");

                                contents.push('\n');
                            }
                            false => {
                                contents.push_str(&buf_line);
                                contents.push('\n');
                            }
                        }
                    }
                }
            }

            println!(
                "buf_line_index : {}, last_index :{}",
                buf_line_index, last_index
            );

            if buf_line_index == last_index {
                file_write(&contents, None);
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

fn generate_random_hash() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..8).map(|_| rng.gen()).collect();
    let hash = md5::compute(random_bytes);

    format!("{:x}", hash)
}

fn create_uml_image(filename: String, img_file_names: Option<Vec<String>>) {
    println!("img_file_names : {:?}", &img_file_names);
    if let Some(names) = img_file_names {
        let mut command = Command::new("java");

        command.arg("-jar")
            .arg("./plantuml.jar")
            .arg("-tsvg")
            .arg("-nometadata")
            .arg("-charset")
            .arg("UTF-8");
        println!("file name log : {}", filename);

        // for name in names {
        //     command.arg("-o").arg(name);
        // }
        //
        command.arg(filename);

        message_alert(&format!("command : {:?}", command));

        match command.status() {
            Ok(done) => {
                let mut msg = done.to_string();
                msg.push_str(" : img created from jar file");
                message_alert(&msg);
            }
            Err(e) => {
                let msg = e.to_string();
                message_alert(&msg);
            }
        }
    }
}
