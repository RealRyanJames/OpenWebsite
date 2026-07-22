use std::{io::stdin, process::Command};

use chrono::{DateTime, Local};
use tokio::{fs::File, io::AsyncWriteExt};

fn create_date() {
    let now: DateTime<Local> = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}

#[tokio::main]
async fn main() {
    let mut site = String::new();

    create_date();

    loop {
        let mut browser = String::new();
        stdin().read_line(&mut browser).unwrap();

        if browser.to_owned().starts_with("Firefox") {
            println!("{}", "Enter Site Name: ".to_uppercase());

            stdin().read_line(&mut site).unwrap();
            Command::new("cmd")
                .arg("/C")
                .arg("C:\\Program Files\\Mozilla Firefox\\Firefox.exe")
                .arg(site.to_string())
                .spawn()
                .expect("No Site");

            let file: File = File::create("commands.txt").await.expect("NO File Created");
            write_to_file(file, String::from(site.as_str())).await;
        } else {
            println!("{}", "Enter Site Name: ".to_uppercase());

            stdin().read_line(&mut site).unwrap();
            Command::new("cmd")
                .arg("/C")
                .arg(browser)
                .arg(site.to_string())
                .spawn()
                .expect("No Site");
        }

        let file: File = File::create("commands.txt").await.expect("NO File Created");
        write_to_file(file, String::from(site.as_str())).await;

        if empty(String::from(site.as_str())) {
            break;
        }

        break;
    }
    ()
}

async fn write_to_file(mut file: File, str: String) {
    file.write_all(str.as_bytes())
        .await
        .expect("No File Created");
    ()
}

fn empty(str: String) -> bool {
    str == ""
}
