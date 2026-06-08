use std::path::Path;
use std::process::Command;
use std::time::Duration;
use systemstat::{Platform, System};

fn get_git() -> Vec<String> {
    let buf = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --abbrev-ref HEAD")
        .output()
        .expect("Failed to execute command")
        .stdout;
    let git_branch = format!("Branch: {}", String::from_utf8(buf).unwrap().trim());

    let buf = Command::new("sh")
        .arg("-c")
        .arg("git status --porcelain")
        .output()
        .expect("Failed to execute command")
        .stdout;
    let git_stat = format!("Status:\n{}", String::from_utf8(buf).unwrap().trim());
    let mut git = vec![git_branch];
    if git_stat != "Status:\n" {
        git.push(git_stat);
    }

    git
}

fn get_sysinfo() -> Vec<String> {
    let sys = System::new();

    fn fmt_uptime(uptime: Duration) -> String {
        let mins = uptime.as_secs() / 60;
        let hrs = mins / 60;
        let days = hrs / 24;

        if days != 0 && hrs != 0 {
            return format!("Uptime: {:02}:{:02}:{:02}", days, hrs, mins);
        } else if hrs != 0 {
            return format!("Uptime: {:02}:{:02}", hrs, mins);
        }
        format!("Uptime: {}m", mins)
    }

    let uptime = match sys.uptime() {
        Ok(uptime) => fmt_uptime(uptime),
        Err(_) => "Error getting uptime".to_string(),
    };

    let cpu_load = match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            let cpu = cpu.done().unwrap();
            format!("CPU Usage: {}%", cpu.system * 100.0)
        }
        Err(_) => "Error getting cpu usage".to_string(),
    };

    let cpu_temp = match sys.cpu_temp() {
        Ok(temp) => format!("CPU Temp (C): {}", temp),
        Err(_) => "Error getting cpu temp".to_string(),
    };

    vec![uptime, cpu_load, cpu_temp]
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
    let pad = col_width - 1;
    let bottom_pad = col_width + 1;

    for text_col in text_cols {
        let mut col: Vec<String> = Vec::new();

        for section in text_col {
            for (i, line) in section.lines().enumerate() {
                let box_char = if i == 0 { "╠" } else { "║" };
                col.push(format!("{} {:<pad$}", box_char, line))
            }
        }

        col.push(format!("{:<bottom_pad$}", "╚".to_string()));
        cols.push(col);
    }

    let max_rows = cols.iter().map(|c| c.len()).max().unwrap_or(0);
    let col_ct = cols.len();

    let res_vec = (0..max_rows)
        .map(|i| {
            (0..col_ct)
                .map(|j| cols[j].get(i).cloned().unwrap_or_default())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut res = String::new();
    for row in res_vec {
        for col in row {
            res.push_str(&format!("{:<bottom_pad$}", col));
        }
        res.push('\n');
    }

    res.insert_str(0, &divider(&cols, max_x));
    res
}

pub fn get_text(x: usize, y: usize) -> String {
    let mut cols: Vec<Vec<String>> = Vec::new();
    if Path::exists(Path::new("./.git")) {
        cols.push(get_git());
    }
    cols.push(get_sysinfo());
    fmt_text(&cols, x, y)
}
