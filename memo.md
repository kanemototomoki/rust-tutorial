- `!`付きは関数ではなくマクロを呼んでいる
  - e.g. `!println!()`
- `cargo new {pj_name}`で新しいプロジェクトの作成
- `cargo build`でビルド
- `cargo run`でコンパイルから実行まで
- `cargo check`バイナリは生成せずにビルドする watch的な
- `cargo doc --open`でドキュメント生成&open

- ライブラリクレートを追加するには`Cargo.toml`の`[dependencies]`セクションに追加する
- `match`式は一般的なエラー処理で使う
