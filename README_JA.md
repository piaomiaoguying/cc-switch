# CC Switch（強化フォーク）

**[CC Switch](https://github.com/farion1231/cc-switch) の強化フォーク — AI CLI ツール向けに画像整流器とマルチプロバイダー視覚分析スキルを追加。**

[English](README.md) | [中文](README_ZH.md) | 日本語

---

## オリジナルプロジェクトについて

[CC Switch](https://github.com/farion1231/cc-switch) は、Claude Code、Codex、Gemini CLI、OpenCode、OpenClaw、Hermes などの AI CLI ツールを管理するデスクトップアプリです。プロバイダー管理、プロキシ/フェイルオーバー、MCP/Skills 管理、使用量統計などの機能を備えています。

本フォークでは、多くのサードパーティ API プロバイダーがマルチモーダル（視覚理解）に対応していない問題を解決するため、**画像入力**処理の機能を追加しています。

---

## 本フォークの独自機能

### 1. 画像整流器（Image Rectifier）

**解決する問題：** 多くのサードパーティ API プロバイダー（中継サービス、非公式エンドポイント）はマルチモーダル入力に対応していません。messages 配列に base64 画像データを含む `type: "image"` ブロックがあると、エラーや予期しない動作が発生します。

**解決策：** 画像整流器はプロキシレイヤーのインターセプターで、**リクエスト転送前に**実行されます。`messages[*].content` をスキャンし、`type: "image"` ブロックを検出して base64 データ（多くの場合数 MB）を削除し、指定されたスキルを呼び出して画像を処理するようモデルに指示するテキストプロンプトに置き換えます。

**主な機能：**
- messages 内の `type: "image"` ブロックの自動検出と置換
- 画像キャッシュ参照が利用可能な場合、ファイルパスをプロンプトに含める
- 呼び出すスキル名を設定可能（デフォルト: `image-analysis`）
- 以下の画像分析スキルとシームレスに連携

**設定方法：** プロキシ設定 → 高度な整流器 → 画像整流器。スイッチをオンにし、必要に応じてスキル名を変更します。

---

### 2. 画像分析スキル（Image Analysis Skill）

シェルコマンドを実行できる任意の AI CLI と互換性のある、スタンドアロンの Python コマンドライン画像視覚認識ツールです。

**主な機能：**
- ローカル画像（jpg/png/gif/webp/bmp）、ネットワーク画像 URL、システムクリップボード（macOS AppleScript）に対応
- 複数回の `--image` による画像比較
- 思考モード（`--thinking`）
- JSON 出力（`--json`）
- トークン使用量表示（`--show-usage`）

**30 以上のプリセット AI プロバイダー：**

| プラットフォーム | 代表的なモデル |
|------|------|
| Volcengine（火山引擎） | Doubao Seed 2.0 Pro/Lite/Mini、Vision 250815 |
| SiliconFlow（硅基流动） | Qwen3.6-35B-A3B、Qwen3.6-27B |
| Alibaba Bailian（阿里百煉） | Qwen3.6 Plus/Flash、Qwen3.5 Omni、Kimi K2.6、MiniMax M2.5 |
| Zhipu（智譜） | GLM-4.6V-Flash |
| SenseNova（商湯） | SenseNova-6.7-Flash-Lite |

**フォールバック機構：** `--fallback` フラグは設定された順序ですべてのプロバイダーを試行し、失敗時に自動的に切り替えます。最初の成功結果が `[プロバイダー名]` ラベル付きで返されます。

**クイックスタート：**

```bash
# 設定テンプレートをコピーして API キーを入力
cp skills/image-analysis/scripts/config.example.json skills/image-analysis/scripts/config.json

# フォールバックモードで単一画像分析
python skills/image-analysis/scripts/vision.py analyze \
  --image path/to/image.png \
  --prompt "この画像を詳しく説明してください" \
  --fallback

# クリップボード画像分析（macOS）
python skills/image-analysis/scripts/vision.py analyze \
  --prompt "この画像にどんな文字がありますか？" \
  --json --fallback
```

**画像整流器との連携：** モデルが画像に対応していない場合、整流器は画像ブロックを `"この画像を読み取って分析するには 'image-analysis' スキルを使用してください。画像パス: /path/to/file.png"` のようなプロンプトに置き換えます。モデルはこの CLI ツールを呼び出して実際の視覚分析を実行します。

---

### 3. dev.sh — 開発ヘルパースクリプト

プリセット設定で `pnpm tauri dev` / `pnpm tauri build` をラップする便利なスクリプト：

```bash
./dev.sh         # Debug モード（デフォルト）
./dev.sh debug   # Debug モード（リクエストボディダンプ付き）
./dev.sh release # Release モード
./dev.sh build   # Release ビルド
```

ファイアウォール内の開発者向けに `CARGO_HTTP_PROXY` がプリセットされています。

---

## インストール

[オリジナルの CC Switch のリリースページ](https://github.com/farion1231/cc-switch/releases)から最新バージョンをダウンロードするか、ソースからビルドします：

```bash
git clone https://github.com/piaomiaoguying/cc-switch.git
cd cc-switch
./dev.sh build
```

---

## ドキュメント

- [ユーザーマニュアル](docs/user-manual/README.md)
- [オリジナル CC Switch リポジトリ](https://github.com/farion1231/cc-switch)

---

## ライセンス

MIT — オリジナルプロジェクトと同じです。
