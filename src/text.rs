use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use sysinfo::{CpuRefreshKind, Disks, MINIMUM_CPU_UPDATE_INTERVAL, RefreshKind, System};

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
    let mut sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
    sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let cpu_loads = sys.cpus().iter().map(|c| c.cpu_usage());
    let (load_sum, load_ct) = cpu_loads
        .clone()
        .fold((0.0, 0), |(s, c), val| (s + val, c + 1));
    let cpu_load_avg = load_sum / load_ct as f32;
    let cpu_load = format!(
        "CPU Usage:
Min: {:.2}%
Avg: {:.2}%
Max: {:.2}%
",
        cpu_loads.clone().reduce(f32::min).unwrap_or(0.0),
        cpu_load_avg,
        cpu_loads.reduce(f32::max).unwrap_or(0.0),
    );

    let memory = format!(
        "RAM: {:.2}GB/{:.2}GB",
        sys.free_memory() as f32 / (1024.0 * 1024.0 * 1024.0),
        sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0)
    );

    let disks = Disks::new_with_refreshed_list();
    let disk_usage = format!(
        "Disk Usage: {:.3}%",
        disks[0].usage().total_written_bytes as f32 / disks[0].total_space() as f32,
    );

    vec![memory, cpu_load, disk_usage]
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
        let mut cur_height = 0;

        for section in text_col {
            for (i, line) in section.lines().enumerate() {
                if cur_height >= max_y && max_y != 0 {
                    break;
                };
                cur_height += 1;
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
            res.push_str(&match col.char_indices().nth(bottom_pad) {
                Some((idx, _)) => format!("{}... ", &col[..idx - 4]),
                None => format!("{:<bottom_pad$}", col),
            });
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
