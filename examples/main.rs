use rusty_style::{Color, Style};

fn main() {
    let underline = Style::new().underline();

    println!("{}", underline.render("I am underline!"));

    let bold = Style::new().bold();

    println!("{}", bold.render("I am bold!"));

    let my_style = Style::new()
        .bold()
        .italic()
        .foreground(Color::convert_hex_to_rgb("#FFFFFF").unwrap())
        .set_string("I have multiple");

    println!("{}", my_style.render(", Styles!"));

    let blinker = Style::new().blink().strike_through();

    println!("{}", blinker.render("I am blinking, maybe not."));

    let reverse = Style::new().reverse();

    println!("{}", reverse.render("I am not here!"));

    let color = Color::convert_hex_to_rgb("#FF0000").unwrap();
    println!("{}", color)
}
