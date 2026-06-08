use cutify::Cutifier;
mod head;
mod text;

fn main() {
    let (head, text_col, palette) = head::get_head();
    let text = text::get_text(70, 20);
    let mut buf = Vec::new();
    Cutifier::new(text)
        .palette(palette)
        .base_hue(text_col)
        .hue_shift(0.0)
        .write_to(&mut buf)
        .unwrap();
    let cute_text = String::from_utf8(buf).unwrap().trim_end().to_string();

    print!("{}", head);
    println!("{}", cute_text);
}
