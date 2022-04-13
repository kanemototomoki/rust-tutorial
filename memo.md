- `!`付きは関数ではなくマクロを呼んでいる
  - e.g. `!println!()`
- `cargo new {pj_name}`で新しいプロジェクトの作成
- `cargo build`でビルド
- `cargo run`でコンパイルから実行まで
- `cargo check`バイナリは生成せずにビルドする watch的な
- `cargo doc --open`でドキュメント生成&open

- ライブラリクレートを追加するには`Cargo.toml`の`[dependencies]`セクションに追加する
- `match`式は一般的なエラー処理で使う
- **命名はスネークケース**
- parameter = 仮引数
- argument = 実引数
  - 絶対忘れるわこれ

## 型
### スカラー型(JSでいうプリミティブ？)
- 整数
  - 8bit~128bitまである
  - e.g. i8, u8
  - iが符号付き(-の可能性もある)
  - arch: isize, usizeはプログラムが依存しているOSの種類に依存する(64 or 32)
- 浮動小数点数
  - f32, f64
- 論理値
- 文字

### 複合型
- タプル
  - それぞれが別の型でもOK
  ```rust
  fn main() {
      let x: (i32, f64, u8) = (500, 6.4, 1);

      let five_hundred = x.0;

      let six_point_four = x.1;

      let one = x.2;
  }
  ```
- 配列
  - 全て同じ型じゃないとダメ
  - 固定長
  - 配列か**ベクタ型**で迷ったらベクタ型を使う
