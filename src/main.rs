use chrono::{Local, Timelike};
use cutify::{Cutifier, GradientConfig, GradientDirection, HueRange, cuteprintln};
use rand::seq::IndexedRandom;
use std::process::Command;
use text2art::{BasicFonts, Font, Printer};

fn get_theme() -> GradientConfig {
    let hour: u32 = Local::now().hour();

    if hour < 9 {
        // Sunrise

        GradientConfig {
            base_hue: Some(30.0),
            hue_shift: 1.5,
            step: 1.0,
            hue_range: HueRange::Custom(30.0, 90.0),
            direction: GradientDirection::Diagonal,
            lightness: 85.0,
            saturation: 85.0,
            reverse: false,
            scale: 50.0,
        }
    } else if hour > 20 {
        // Midday

        GradientConfig {
            base_hue: Some(30.0),
            hue_shift: 1.5,
            step: 1.0,
            hue_range: HueRange::Oranges,
            direction: GradientDirection::Diagonal,
            lightness: 55.0,
            saturation: 85.0,
            reverse: false,
            scale: 50.0,
        }
    } else {
        // Sunset

        GradientConfig {
            base_hue: Some(60.0),
            hue_shift: 3.0,
            step: 1.0,
            hue_range: HueRange::Custom(330.0, 90.0),
            direction: GradientDirection::Vertical,
            lightness: 85.0,
            saturation: 85.0,
            reverse: true,
            scale: 50.0,
        }
    }
}

fn print_head(theme: GradientConfig) {
    let hour: u32 = Local::now().hour();
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Failed to load font"),
    };
    let printer = Printer::with_font(font);
    let curr_time = Local::now().format("%H:%M").to_string();
    let mut rng = rand::rng();
    cutify::set_palette(theme);

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
        cuteprintln(&rdrd_head);
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
    print_head(theme);
    let text = get_repo_info();

    let offy: usize = 0;

    for _ in 0..offy {
        println!();
    }
    println!("{} {}", Cutifier::new("╠"), text.join("\n"))
}
