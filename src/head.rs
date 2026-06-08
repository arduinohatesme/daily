use chrono::{Local, Timelike};
use cutify::{ColorPalette, Cutifier, GradientDirection};
use text2art::{BasicFonts, Font, Printer};

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

fn write_sunrise(rdrd_time: &str) -> String {
    let mut buf: Vec<u8> = Vec::new();
    let head: String = r#"


⠀⠀⠀⠀⠀   ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
     ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"#
        .to_string();

    let shift = 8.0;

    for (i, l) in zip_art_time(&head, rdrd_time).lines().enumerate() {
        let cur_base = (i as f32 * shift) + 40.0;

        Cutifier::new(l.replace("\n", ""))
            .palette(ColorPalette::Fire)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .write_to(&mut buf)
            .unwrap();
    }
    String::from_utf8(buf)
        .unwrap()
        .replace("\n\n", "")
        .trim_end()
        .to_string()
}

fn write_midday(rdrd_time: &str) -> String {
    let mut buf: Vec<u8> = Vec::new();
    let head: String = r#"
      ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
      ⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
      ⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
      ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
      ⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇
       ⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋
      ⠀⠀⠀⠉⠛⠻⠿⠿⠟⠛⠉
"#
    .to_string();

    let shift = 9.0;

    for (i, l) in zip_art_time(&head, rdrd_time).lines().enumerate() {
        let cur_base = (i as f32 * shift) + 40.0;

        Cutifier::new(l.replace("\n", ""))
            .palette(ColorPalette::Fire)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .write_to(&mut buf)
            .unwrap();
    }
    String::from_utf8(buf).unwrap().replace("\n\n", "")
}

fn write_sunset(rdrd_time: &str) -> String {
    let mut buf: Vec<u8> = Vec::new();
    let head: String = r#"


⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
⠀⠀⠀⠀ ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"#
        .to_string();

    let shift = 15.0;

    for (i, l) in zip_art_time(&head, rdrd_time).lines().enumerate() {
        let cur_base = (i as f32 * shift) + 40.0;

        Cutifier::new(l.replace("\n", ""))
            .palette(ColorPalette::Sunset)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .reverse()
            .write_to(&mut buf)
            .unwrap();
    }
    String::from_utf8(buf)
        .unwrap()
        .replace("\n\n", "")
        .trim_end()
        .to_string()
}

pub fn get_head() -> String {
    let hour: u32 = Local::now().hour();
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Failed to load font."),
    };
    let prtr = Printer::with_font(font);
    let curr_time = Local::now().format("%m/%d").to_string();
    let mut rdrd_time = match prtr.render_text(&curr_time) {
        Ok(str) => str,
        Err(_) => "Error rendering time".to_string(),
    };
    rdrd_time.insert(0, '\n');

    match hour {
        0..9 => write_sunrise(&rdrd_time),
        9..20 => write_midday(&rdrd_time),
        _ => write_sunset(&rdrd_time),
    }
}
