use chrono::{Local, Timelike};
use cutify::{ColorPalette, Cutifier, GradientDirection};
use std::process::Command;
use text2art::{BasicFonts, Font, Printer};

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

fn zip_art_time(art: &str, rd_time: &str) -> String {
    let art = art.to_string();
    let art_lines: Vec<&str> = art.lines().collect();
    let time_lines: Vec<&str> = rd_time.lines().collect();
    let y = art_lines.len().max(time_lines.len());

    (0..y)
        .map(|i| {
            format!(
                "{:<30}{}\n",
                art_lines.get(i).unwrap_or(&""),
                time_lines.get(i).unwrap_or(&"")
            )
        })
        .collect()
}

fn main() {
    let hour: u32 = Local::now().hour();
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Failed to load font."),
    };
    let prtr = Printer::with_font(font);
    let curr_time = Local::now().format("%H:%M").to_string();
    let mut rdrd_time = match prtr.render_text(&curr_time) {
        Ok(str) => str,
        Err(_) => "Error rendering time".to_string(),
    };
    rdrd_time.insert(0, '\n');

    if true {
        // Sunrise
        let head: String = r#"


⠀⠀⠀⠀⠀   ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
     ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⠀⠘⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠃"#
            .to_string();

        let shift = 8.0;
        let mut buf = Vec::new();

        for (i, l) in zip_art_time(&head, &rdrd_time).lines().enumerate() {
            let cur_base = (i as f32 * shift) + 40.0;

            Cutifier::new(l.replace("\n", ""))
                .palette(ColorPalette::Fire)
                .direction(GradientDirection::Vertical)
                .base_hue(cur_base)
                .write_to(&mut buf)
                .unwrap();
        }
        let prt = String::from_utf8(buf).unwrap().replace("\n\n", "");
        print!("{}", prt);
    } else if hour < 20 {
        // Midday
        let head: String = r#"
      ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
      ⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
      ⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
      ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
      ⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇
       ⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋
      ⠀⠀⠀⠉⠛⠻⠿⠿⠟⠛⠉"#
            .to_string();

        Cutifier::new(zip_art_time(&head, &rdrd_time))
            .palette(ColorPalette::Fire)
            .direction(GradientDirection::Diagonal)
            .scale(10.0)
            .print();
    } else {
        // Sunset
        let head: String = r#"


⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
⠀⠀⠀⠀ ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⠀⠘⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠃"#
            .to_string();

        let shift = 15.0;
        let mut buf = Vec::new();

        for (i, l) in zip_art_time(&head, &rdrd_time).lines().enumerate() {
            let cur_base = (i as f32 * shift) + 40.0;

            Cutifier::new(l.replace("\n", ""))
                .palette(ColorPalette::Sunset)
                .direction(GradientDirection::Vertical)
                .base_hue(cur_base)
                .reverse()
                .write_to(&mut buf)
                .unwrap();
        }
        let prt = String::from_utf8(buf).unwrap().replace("\n\n", "");
        print!("{}", prt);
    };

    let text = get_text();

    let offy: usize = 0;

    for _ in 0..offy {
        println!();
    }
    println!("{}", text.join("\n"))
}
