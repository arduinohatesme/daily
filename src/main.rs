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
    let art_raw = r#"
     _       _ _
  __| | __ _(_) |_   _
 / _` |/ _` | | | | | |
| (_| | (_| | | | |_| |
 \__,_|\__,_|_|_|\__, |
                 |___/
"#;

    let mut text = get_text();

    let mut art: Vec<&str> = art_raw.trim_matches('\n').lines().collect();

    let dispy = art.len().max(text.len());
    let dispx = art.iter().map(|s| s.len()).max().unwrap();
    let offy: i32 = 0;
    let offx: i32 = 5;
    let totalx = dispx + offx as usize;

    text.resize(dispy, String::new());
    art.resize(dispy, "");

    if offy > 0 {
        let mut pad = vec![String::new(); offy as usize];
        pad.append(&mut text);
        text = pad;
    } else if offy < 0 {
        let mut pad = vec![""; offy.unsigned_abs() as usize];
        pad.append(&mut art);
        text = pad.into_iter().map(str::to_owned).collect();
    }

    for i in 0..dispy {
        println!("{:<totalx$}{}", art[i], text[i])
    }
}
