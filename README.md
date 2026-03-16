# pomodoro_timer

CH32V003F4P6マイコン向けのポモドーロタイマー実装です。
RustとEmbassy非同期ランタイムを使用しています。

`src/main.rs`以外は、`embedded-hal`を用い、`ch32-hal`に依存しないように作成したため、他のチップでも動くと思います。

https://github.com/user-attachments/assets/0f5a5919-d88c-4be7-9235-b88e820856ae

## 機能

- **ポモドーロサイクル**
  - 作業: 25分
  - 短い休憩: 5分
  - 長い休憩: 15分（4ポモドーロごと）
- 各フェーズをLEDで表示
- フェーズ終了時にパッシブブザーと振動モーターで通知
- ボタン操作でタイマー開始・次フェーズへの移行

各種時間や長い休憩までのポモドーロの回数は`src/pomodoro.rs`の対応した定数を変更することで、変えることができます。

## ハードウェア構成

| 機能                 | ピン |
| -------------------- | ---- |
| 作業LED              | PC6  |
| 短い休憩LED          | PC2  |
| 長い休憩LED          | PC5  |
| パッシブブザー       | PC3  |
| 振動モーター         | PA2  |
| ボタン1 (開始/次へ)  | PD0  |
| ボタン2 (現在未使用) | PC7  |

## 使い方

1. デバイスに書き込む
2. **ボタン1**を押すとタイマー開始
3. 各フェーズ終了後、ブザーと振動で通知される
4. **ボタン1**を押して次のフェーズへ進む

## セットアップ

### 必要なツール

- Rust nightly（`rust-toolchain.toml` で自動設定）
- [minichlink](https://github.com/cnlohr/ch32fun/tree/master/minichlink)（書き込みツール）
  - PATHに追加してください
- `rust-objcopy`

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## ビルド

```bash
cargo build --release
```

## 書き込み

```bash
cargo run --release
```

`cargo run`を実行すると、`rust-objcopy`でバイナリに変換後、`minichlink`でデバイスに書き込まれます。

## ライセンス

MIT OR Apache-2.0
