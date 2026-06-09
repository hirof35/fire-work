# 🎆 Rust + WebAssembly Firework

Rust で記述したロジックを WebAssembly（Wasm）へコンパイルし、ブラウザの Canvas API を使って高速かつ滑らかに描画する花火のアニメーション作品です。

Bundler（Vite や Webpack など）を一切挟まず、ブラウザ標準の ES Modules 形式（`--target web`）で直接動作する軽量な構成になっています。

<img width="1913" height="1032" alt="スクリーンショット 2026-06-10 070344" src="https://github.com/user-attachments/assets/bcff5f42-3a65-45e5-9243-a15ac3cfb577" />

---

## 🛠️ 技術スタック
- **Language:** Rust (edition 2021)
- **Technology:** WebAssembly / HTML5 Canvas API
- **Tooling:** `wasm-pack` / `cargo`
- **Libraries (Crates):**
  - `wasm-bindgen` (JS と Rust の相互運用のための架け橋)
  - `web-sys` (Web API、CanvasRenderingContext2d などの利用)
  - `js-sys` (JavaScript の標準組み込みオブジェクト・関数の利用)

---

## 📦 ディレクトリ構造
```text
fire-work/
  ├── Cargo.toml      # Rust のメタデータと依存関係
  ├── index.html      # Wasm モジュールを読み込んで表示するメインページ
  ├── README.md       # このファイル
  └── src/
       └── lib.rs     # 花火の物理演算・描画ロジック（Rust）
🚀 ローカルでの実行方法
1. 前提条件のインストール
Rust 環境に加え、wasm-pack が必要です。

Bash
# Rust のアップデート
rustup update

# wasm-pack のインストール
cargo install wasm-pack
2. WebAssembly のビルド
プロジェクトのルートディレクトリで以下のコマンドを実行し、ブラウザ向け（ES Modules 形式）のパッケージを生成します。

Bash
wasm-pack build --target web
ビルドが成功すると、自動的に ./pkg フォルダが作成され、Wasm バイナリと JavaScript のグルーコードが生成されます。

3. ローカルサーバーの起動
WebAssembly はセキュリティ（CORS）の制限があるため、index.html を直接ダブルクリックしても動作しません。ローカル開発サーバーを立ち上げる必要があります。

例1：VS Code の「Live Server」拡張機能を使う場合
index.html を開き、画面右下の「Go Live」をクリックします。

例2：Python を使う場合

Bash
python -m http.server 8080
起動後、ブラウザで http://localhost:8080 にアクセスします。

例3：Rust の miniserve を使う場合

Bash
cargo install miniserve
miniserve . --index index.html
📝 ライセンス
本プロジェクトは MIT ライセンス、またはオープンソースとして自由にご利用いただけます。
