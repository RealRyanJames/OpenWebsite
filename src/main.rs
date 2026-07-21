use std::{io::stdin, process::Command};

#[tokio::main]
async fn main() {
    let mut site = String::new();

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

        if empty(String::from(site.as_str())) {
            break;
        }

        break;
    }
    ()
}

fn empty(str: String) -> bool {
    str == ""
}
