mod head;
mod text;

fn main() {
    let head = head::get_head();
    let text = text::get_text(70, 20);

    print!("{}", head);
    println!("{}", text);
}
