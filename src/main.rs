use std::process::Command;
mod head;

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
    let head = head::get_head();
    let text = get_text();
    print!("{}", head);

    let offy: usize = 0;

    for _ in 0..offy {
        println!();
    }
    println!("{}", text.join("\n"))
}
