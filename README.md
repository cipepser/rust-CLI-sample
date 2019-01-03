# rust-CLI-sample

[Command Line Toolを作ってみる in Rust \- Qiita](https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c)の写経。

## 環境

```sh
❯ rustc --version
rustc 1.31.1 (b6c32da9b 2018-12-18)
```

## urlクレートによるパーセントエンコーディング

```rust
extern crate url;

use url::percent_encoding::percent_decode;

fn main() {
    let input = "foo%20bar";
    let decoded = percent_decode(input.as_bytes()).decode_utf8();
    println!("{}", decoded.unwrap());
}
```

結果

```sh
❯ cargo run
foo bar
```

## References
* [Command Line Toolを作ってみる in Rust \- Qiita](https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c)