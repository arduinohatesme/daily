use chrono::{Local, Timelike};
use cutify::{ColorPalette, Cutifier, GradientDirection};
use std::process::Command;

fn get_text() -> Vec<String> {
    let mut outvec: Vec<u8> = {
        Command::new("sh")
            .arg("-c")
            .arg("git rev-parse --abbrev-ref HEAD")
            .output()
            .expect("Failed to execute command")
            .stdout
    };

    outvec.append(&mut {
        Command::new("sh")
            .arg("-c")
            .arg("git status --porcelain")
            .output()
            .expect("Failed to execute command")
            .stdout
    });

    let outvecstr: String = String::from_utf8(outvec).expect("");

    let text: Vec<String> = outvecstr
        .trim_matches('\n')
        .lines()
        .map(str::to_owned)
        .collect();

    text
}

fn main() {
    let hour: u32 = Local::now().hour();
    if hour < 9 {
        // Sunrise
        Cutifier::new(
            r#"
⠀⠀⠀⠀⠀   ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀⠀⠀⠀  ⠀⠀⠀⠀⠀⠀
     ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀   ⠀⠀⠀⠀
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀   ⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠘⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠃⠀
"#,
        )
        .palette(ColorPalette::Fire)
        .direction(GradientDirection::Vertical)
        .scale(7.0)
        .print();
    } else if hour > 19 {
        // Sunset
        Cutifier::new(
            r#"
⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀⠀⠀⠀  ⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀ ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀ ⠀⠀⠀⠀
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀   ⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠘⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠃⠀
"#,
        )
        .palette(ColorPalette::Sunset)
        .direction(GradientDirection::Vertical)
        .scale(7.0)
        .reverse()
        .print();
    } else {
        // Midday
        Cutifier::new(
            r#"
⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
⠀⠀⠀⠀ ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
        ⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀   ⠀⠀⠀
         ⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋
⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⠉⠛⠻⠿⠿⠟⠛⠉
"#,
        )
        .palette(ColorPalette::Fire)
        .direction(GradientDirection::Diagonal)
        .scale(10.0)
        .print();
    };

    let text = get_text();

    let offy: usize = 0;

    for _ in 0..offy {
        println!();
    }
    println!("{}", text.join("\n"))
}
