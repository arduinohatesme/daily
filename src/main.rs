use chrono::{Local, Timelike};
use cutify::{ColorPalette, Cutifier, GradientConfig, GradientDirection, cuteprintln};
use rand::seq::IndexedRandom;
use std::process::Command;
use text2art::{BasicFonts, Font, Printer};

fn get_theme() -> GradientConfig {
    let hour: u32 = Local::now().hour();

    if hour < 9 {
        // Sunrise

        GradientConfig::default()
            .palette(ColorPalette::Fire)
            .direction(GradientDirection::Diagonal)
            .scale(50.0)
    } else if hour > 20 {
        // Midday

        GradientConfig::default()
            .palette(ColorPalette::Fire)
            .direction(GradientDirection::Vertical)
            .step(50.0)
    } else {
        // Sunset

        GradientConfig::default()
            .palette(ColorPalette::Sunset)
            .direction(GradientDirection::Diagonal)
            .scale(50.0)
            .reverse()
    }
}

fn print_head(theme: &GradientConfig) {
    let hour: u32 = Local::now().hour();
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Failed to load font"),
    };
    let printer = Printer::with_font(font);
    let curr_time = Local::now().format("%H:%M").to_string();
    let mut rng = rand::rng();

    if hour < 9 {
        // Sunrise

        let mut disp_head = [
            "Welcome back.",
            "Let's start on the right foot.",
            "Good morning!",
            "Let's get started.",
            "Ready when you are.",
            "Up bright and early!",
        ]
        .choose(&mut rng)
        .copied()
        .unwrap_or("Good morning!")
        .to_string();

        disp_head.push_str(&format!("\nIt's {}.", curr_time));

        let rdrd_head = match printer.render_text(&disp_head) {
            Ok(str) => str,
            Err(_) => "Error rendering text".to_string(),
        };

        cuteprintln(&rdrd_head);
    } else if hour > 19 {
        // Sunset

        let mut disp_head = [
            "Good evening!",
            "Ready for sundown?",
            "Let's get this done.",
            "Finish off the day.",
        ]
        .choose(&mut rng)
        .copied()
        .unwrap_or("Ending off strong!")
        .to_string();

        disp_head.push_str(&format!("\nIt's {}.", curr_time));

        let rdrd_head = match printer.render_text(&disp_head) {
            Ok(str) => str,
            Err(_) => "Error rendering text".to_string(),
        };

        cuteprintln(&rdrd_head);
    } else {
        // Midday

        let mut disp_head = [
            "Keep powering through.",
            "I'm just getting warmed up!",
            "Still at it, boss.",
            "I don't need a lunch break.",
        ]
        .choose(&mut rng)
        .copied()
        .unwrap_or("Keep powering through.")
        .to_string();

        disp_head.push_str(&format!("\nIt's {}.", curr_time));

        let rdrd_head = match printer.render_text(&disp_head) {
            Ok(str) => str,
            Err(_) => "Error rendering text".to_string(),
        };
    };
}

fn get_repo_info() -> Vec<String> {
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
    let theme = get_theme();
    print_head(&theme);
    let text = get_repo_info();

    let offy: usize = 0;

    for _ in 0..offy {
        println!();
    }
    println!("{} {}", Cutifier::new("╠"), text.join("\n"))
}
