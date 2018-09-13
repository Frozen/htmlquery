#htmlquery

Html parsing library with jquery like css selectors

### Installing
Add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
htmlquery = {git="https://github.com/Frozen/htmlquery.git", tag = "0.1"}
```

### Examples
```rust
extern crate htmlquery;
use htmlquery::parse_html;

fn main() {
    let html = r#"<div class="main"><a href="https://www.rust-lang.org"></div>"#;
    let dom = parse_html(html);
    assert_eq!(1, dom.all(".main a").len());
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)