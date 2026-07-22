use std::{io::stdin, process::Command};

use chrono::{DateTime, Local};
use tokio::{fs::File, io::AsyncWriteExt};

fn create_date() {
    let now: DateTime<Local> = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}

pub struct WebRoute {
    pub google: String,
    pub yt: String,
    pub gmail: String,
}

pub trait Routes {
    fn get_web_site(&self) -> String;
    fn get_gmail(&self) -> String;
    fn get_youtube(&self) -> String;
}

impl Routes for WebRoute {
    fn get_web_site(&self) -> String {
        format!("{}", self.google)
    }

    fn get_gmail(&self) -> String {
        format!("{}", self.gmail)
    }

    fn get_youtube(&self) -> String {
        format!("{}", self.yt)
    }
}

pub fn get_web_route(web_route: WebRoute, str: String) -> String {
    if str.contains(&web_route.google) {
        return format!("{}", "http://www.google.com");
    }

    return format!("{}", web_route.google);
}

pub fn get_gmail(web_route: WebRoute, str: String) -> String {
    if str.contains(&web_route.gmail) {
        return format!("{}", "http://www.google.com/gmail");
    }

    return format!("{}", web_route.gmail);
}

pub fn get_yt(web_route: WebRoute, str: String) -> String {
    if str.contains(&web_route.yt) {
        return format!("{}", "http://www.google.com/youtube");
    }

    return format!("{}", web_route.yt);
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

            let website: WebRoute = WebRoute {
                gmail: String::from(format!("{}", "")),
                google: String::from(format!("{}", "http://www.google.com")),
                yt: String::from(format!("{}", "")),
            };

            let website_yt: WebRoute = WebRoute {
                gmail: String::from(format!("{}", "")),
                google: String::from(format!("{}", "")),
                yt: String::from(format!("{}", "http://www.google.com/youtube")),
            };

            let website_gmail: WebRoute = WebRoute {
                gmail: String::from(format!("{}", "http://www.google.com/gmail")),
                google: String::from(format!("{}", "")),
                yt: String::from(format!("{}", "")),
            };

            if site.contains("Google") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("C:\\Program Files\\Mozilla Firefox\\Firefox.exe")
                    .arg(String::from(get_web_route(
                        website,
                        String::from(site.as_str()),
                    )))
                    .spawn()
                    .expect("No Site");

                let file: File = File::create("commands.txt").await.expect("NO File Created");
                write_to_file(file, String::from(site.as_str())).await;
            } else if site.contains("YouTube") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("C:\\Program Files\\Mozilla Firefox\\Firefox.exe")
                    .arg(String::from(get_yt(
                        website_yt,
                        String::from(site.as_str()),
                    )))
                    .spawn()
                    .expect("No Site");

                let file: File = File::create("commands.txt").await.expect("NO File Created");
                write_to_file(file, String::from(site.as_str())).await;
            } else if site.contains("Gmail") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("C:\\Program Files\\Mozilla Firefox\\Firefox.exe")
                    .arg(String::from(get_gmail(
                        website_gmail,
                        String::from(site.as_str()),
                    )))
                    .spawn()
                    .expect("No Site");

                let file: File = File::create("commands.txt").await.expect("NO File Created");
                write_to_file(file, String::from(site.as_str())).await;
            }
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
