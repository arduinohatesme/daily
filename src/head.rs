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

fn write_sunrise(rdrd_time: &str) -> (String, f32, ColorPalette) {
    let mut buf: Vec<u8> = Vec::new();
    let head: String = r#"


⠀⠀⠀⠀⠀   ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
     ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"#
        .to_string();

    let shift = 8.0;
    let mut cur_base = 40.0;
    let palette = ColorPalette::Fire;
    for l in zip_art_time(&head, rdrd_time).lines() {
        cur_base += shift;

        Cutifier::new(l.replace("\n", ""))
            .palette(palette)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .write_to(&mut buf)
            .unwrap();
    }
    (
        String::from_utf8(buf)
            .unwrap()
            .replace("\n\n", "")
            .trim_end()
            .to_string(),
        cur_base + shift,
        palette,
    )
}

fn write_midday(rdrd_time: &str) -> (String, f32, ColorPalette) {
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
    let mut cur_base = 40.0;
    let palette = ColorPalette::Fire;
    for l in zip_art_time(&head, rdrd_time).lines() {
        cur_base += shift;

        Cutifier::new(l.replace("\n", ""))
            .palette(palette)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .write_to(&mut buf)
            .unwrap();
    }
    (
        String::from_utf8(buf).unwrap().replace("\n\n", ""),
        cur_base + shift,
        palette,
    )
}

fn write_sunset(rdrd_time: &str) -> (String, f32, ColorPalette) {
    let mut buf: Vec<u8> = Vec::new();
    let head: String = r#"


⠀⠀⠀⠀⠀⠀  ⠀⠀⠀⣀⣤⣴⣶⣶⣦⣤⣀
⠀⠀⠀⠀ ⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄
⠀⠀⠀   ⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿"#
        .to_string();

    let shift = 15.0;
    let mut cur_base = 40.0;
    let palette = ColorPalette::Sunset;
    for l in zip_art_time(&head, rdrd_time).lines() {
        cur_base += shift;

        Cutifier::new(l.replace("\n", ""))
            .palette(palette)
            .direction(GradientDirection::Vertical)
            .base_hue(cur_base)
            .reverse()
            .write_to(&mut buf)
            .unwrap();
    }
    (
        String::from_utf8(buf)
            .unwrap()
            .replace("\n\n", "")
            .trim_end()
            .to_string(),
        cur_base + shift,
        palette,
    )
}

pub fn get_head() -> (String, f32, ColorPalette) {
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
