## <h1>Rusty Style</h1>

---

<p>
    <img src="https://e1.pxfuel.com/desktop-wallpaper/299/908/desktop-wallpaper-lip-gloss-aesthetic-skincare-aesthetic.jpg" alt="gloss" width="250px"/><br/>
    <a href="https://img.shields.io/crates/v/rusty_style?color=pink"><img src="https://img.shields.io/crates/v/rusty_style?color=pink" alt="Download"></a>
    <a href="https://img.shields.io/crates/d/rusty_style?color=pink"><img src="https://img.shields.io/crates/d/rusty_style?color=pink" alt="Download"></a>
    <a href="https://github.com/edwardelton/rusty_style/actions/"><img src="https://github.com/edwardelton/rusty_style/actions/workflows/rust.yml/badge.svg" alt="Build Status"></a>
</p>

<p>

</p>

---

<h3>Introduction</h3>

- Rusty Style is a Terminal Utility to style your TUI project.
- It is mainly inspired by lipgloss, a golang TUI library.

Rusty style is built on the builder design pattern. Designing your TUI is easier than ever with Rusty Style.

---

<h3>Example</h3>

```rs
use rusty_style::{Color, Style};

fn main() {
    let underline = Style::new().underline();

    println!("{}", underline.render("I am underline!"));

    let bold = Style::new().bold();

    println!("{}", bold.render("I am bold!"));

    let my_style = Style::new()
        .bold()
        .italic()
        .foreground(Color::new(255, 192, 203))
        .set_string("I have multiple");

    println!("{}", my_style.render(", Styles!")); // render will append the text to I Have multiple
}
```

<p>
    <img src="https://cdn.discordapp.com/attachments/1065385280393203892/1108860596406984784/Screenshot_2023-05-18_at_4.56.01_PM.png" alt="Example" width="330px"/>
</p>

---

<h3>Color</h3>

Rusty Style supports True Color:

<h4>RGB</h4>
```rs
rusty_style::Color::new(255, 192, 203) // pink
rusty_style::Color::new(166, 200, 148) // green
rusty_style::Color::new(142, 29, 206) // purple
```

<h4>HEX</h4>
```rs
rusty_style::Color::convert_hex_to_rgb("#DE3163").unwrap() // cerse
rusty_style::Color::convert_hex_to_rgb("#9F2B68").unwrap() // amaranth
rusty_style::Color::convert_hex_to_rgb("#F2D2BD").unwrap() // bisque
```

---

<h3>Inline Formatting</h3>

Rusty Style supports the usual ANSI text formatting options:

```rs
let style = rusty_style::Style::new()
    bold().
    faint().
    italic().
    underline().
    blink().
    reverse().
    invisible().
    strikethrough();
```

---

<h3>Tips</h3>

When using render, you will lose ownership of your style because render is made to be used once you are done with your style. If you want to keep your style object we recommend you to clone your style.

```rs
let style = rusty_style::Style::new();

let my_copy = style.clone();
```

<h3>Warning</h3>

- If you have any suggestions, problems, open a problem (if it is an error, you must be sure to look if you can solve it with [Google](https://giybf.com)!)

<h3>Support me</h3>

- Thanks for looking at this repository, if you like to press the ⭐ button!
- Made by [Edward Elton](https://github.com/edwardelton).

<p align="center">
    <b>Informations</b><br>
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/edwardelton/rusty_style?color=pink">
    <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/edwardelton/rusty_style?color=pink">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/edwardelton/rusty_style?color=pink">
    <img alt="GitHub" src="https://img.shields.io/github/license/edwardelton/rusty_style?color=pink">
    <img alt="GitHub watchers" src="https://img.shields.io/github/watchers/edwardelton/rusty_style?color=pink">
</p>
```
