<h1 align="center">Rusty Style</h1>

<p align='center'>
    <b>💄 Style Text In Terminal 💄</b>
</p>

----

<p align="center">
    <img src="https://e1.pxfuel.com/desktop-wallpaper/299/908/desktop-wallpaper-lip-gloss-aesthetic-skincare-aesthetic.jpg" alt="gloss" width="500px"/>
</p>

<br/>

> # Introduction

* ***Rusty Style is a Terminal Utility to style your TUI project.***
* ***It is mainly inspired by lipgloss, a golang TUI library.***

* ***It is built using the build design pattern.***

<br/>

> # Example

```rs
use rusty_style::color::Color;

let my_style = rusty_style::style::Style::new()
  .bold()
  .italic()
  .underline()
  .foreground(Color::new(255, 0, 0))
  .background(Color::new(0, 0, 255))
  .set_string("Hello");

let rendered_string = my_style.render("World"); // render will happen the text to Hello
```

<br/>

> # Warning.

* ***If you have any suggestions, problems, open a problem (if it is an error, you must be sure to look if you can solve it with [Google](https://giybf.com)!)***  
  
<br/>

> # Support me.

* Thanks for looking at this repository, if you like to press the ⭐ button!
* Made by [Edward Elton](https://github.com/edwardelton).

<p align="center">
    <b>Informations</b><br>
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/edwardelton/rusty_style?color=0aa2fa">
    <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/edwardelton/rusty_style?color=0aa2fa">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/edwardelton/rusty_style?color=0aa2fa">
    <img alt="GitHub" src="https://img.shields.io/github/license/edwardelton/rusty_style?color=0aa2fa">
    <img alt="GitHub watchers" src="https://img.shields.io/github/watchers/edwardelton/rusty_style?color=0aa2fa">
</p>
