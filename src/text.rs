use std::path::Path;
use std::process::Command;

fn get_git() -> Vec<String> {
    fn get_branch() -> String {
        let buf = Command::new("sh")
            .arg("-c")
            .arg("git rev-parse --abbrev-ref HEAD")
            .output()
            .expect("Failed to execute command")
            .stdout;
        let mut res = String::from_utf8(buf).unwrap().trim().to_string();
        res.insert_str(0, "Branch: ");
        res
    }

    fn get_stat() -> String {
        let buf = Command::new("sh")
            .arg("-c")
            .arg("git status --porcelain")
            .output()
            .expect("Failed to execute command")
            .stdout;
        let mut stat = String::from_utf8(buf).unwrap().trim().to_string();
        if stat.trim().is_empty() {
            "".to_string()
        } else {
            stat.insert_str(0, "Status:\n");
            stat
        }
    }

    vec![get_branch(), get_stat()]
}

fn divider(text: &[Vec<String>], x: usize) -> String {
    let mut res: String = format!("╔{}", "═".repeat(x / text.len() - 1));
    for _ in 1..text.len() {
        res.push_str(&format!("╦{}", "═".repeat(x / text.len() - 1)));
    }
    res.push('\n');
    res
}

fn fmt_text(text_cols: &Vec<Vec<String>>, max_x: usize, max_y: usize) -> String {
    let mut cols: Vec<Vec<String>> = Vec::new();
    let col_width = (max_x / text_cols.len()) - 1;

    for text_col in text_cols {
        let mut col: Vec<String> = Vec::new();

        for section in text_col {
            for (i, line) in section.lines().enumerate() {
                let box_char = if i == 0 { "╠" } else { "║" };
                col.push(format!("{} {:<col_width$}", box_char, line))
            }
        }

        col.push("╚".to_string());
        cols.push(col);
    }

    let max_rows = cols.iter().map(|c| c.len()).max().unwrap_or(0);
    let col_ct = cols.len();

    let mut res_str = (0..max_rows)
        .map(|i| {
            (0..col_ct)
                .map(|j| cols[j].get(i).cloned().unwrap_or_default())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    res_str.insert_str(0, &divider(&cols, max_x));
    res_str
}

pub fn get_text(x: usize, y: usize) -> String {
    let mut cols: Vec<Vec<String>> = Vec::new();
    if Path::exists(Path::new("./.git")) {
        cols.push(get_git());
    }
    fmt_text(&cols, x, y)
}
