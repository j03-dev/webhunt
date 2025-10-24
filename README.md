# WebHunt

**WebHunt** is a **Rust** library for web scraping, built upon the **scraper** library. Its goal is to make web scraping in Rust **easier**.

-----

## Setup

This crate is not yet **published** on **crates.io**, so if you want to install it, add the following line to your project's `Cargo.toml`:

```toml
webhunt = { git = "https://github.com/j03-dev/webhunt", branch="main" }
```

-----

## How to use it

I primarily use this package for my **personal** web scraping needs and wanted to share it. I've tried to make it as **accessible** as possible and optimize the user experience.

Since this project uses the `scraper` crate, the syntax is **similar**. If you encounter any problems or need more information, you can check the `scraper` [documentation](https://docs.rs/scraper/latest/scraper/).

```rust
use webhunt::{Html, Hunt};

const HTML: &str =  r#"
  <html>
    <body>
      <h1 class="title">Lorem ipsum</h1>
      <div class="content">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut pulvinar blandit lacinia.
      </div>
      <a class="link" href="https://example.com">
        <button>More</button>
      </a>
    <body>
  </html>
"#;


#[derive(Hunt)]
struct LoremIpsum {
  #[select(tag="h1.title")]
  title: String,

  #[select(tag="div.content")]
  content: String,

  #[select(tag="a.link", attr="href")]
  link: String
}

let html = Html::parse_document(HTML);

let data = LoremIpsum::from_html(&html).unwrap();

println!("title: {}", data.title);
println!("content: {}", data.content);
println!("link: {}", data.link);
```
