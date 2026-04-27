# pomodoro_timer

A Pomodoro timer implementation for the CH32V003F4P6 microcontroller.
Built with Rust and the Embassy async runtime.

Files other than `src/main.rs` are written using `embedded-hal` without depending on `ch32-hal`, so they should work on other chips as well.

## Features

- **Pomodoro cycle**
  - Work: 25 minutes
  - Short break: 5 minutes
  - Long break: 15 minutes (every 4 pomodoros)
- Phase indication via LEDs
- Passive buzzer and vibration motor notification at the end of each phase
- Button to start the timer and advance to the next phase

Durations and the number of pomodoros before a long break can be changed by editing the corresponding constants in `src/pomodoro.rs`.

## Hardware Configuration

| Function              | Pin  |
| --------------------- | ---- |
| Work LED              | PC6  |
| Short break LED       | PC2  |
| Long break LED        | PC5  |
| Passive buzzer        | PC3  |
| Vibration motor       | PA2  |
| Button 1 (Start/Next) | PD0  |
| Button 2 (unused)     | PC7  |

## Usage

1. Flash the device
2. Press **Button 1** to start the timer
3. At the end of each phase, the buzzer and vibration motor will notify you
4. Press **Button 1** to advance to the next phase

## Setup

### Required Tools

- Rust nightly (configured automatically via `rust-toolchain.toml`)
- [minichlink](https://github.com/cnlohr/ch32fun/tree/master/minichlink) (flashing tool) — add to your PATH
- `rust-objcopy`

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Build

```bash
cargo build --release
```

## Flash

```bash
cargo run --release
```

Running `cargo run` converts the binary with `rust-objcopy` and flashes it to the device using `minichlink`.

## License

MIT OR Apache-2.0

---

# pomodoro_timer

CH32V003F4P6マイコン向けのポモドーロタイマー実装です。
RustとEmbassy非同期ランタイムを使用しています。

`src/main.rs`以外は、`embedded-hal`を用い、`ch32-hal`に依存しないように作成したため、他のチップでも動くと思います。

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
