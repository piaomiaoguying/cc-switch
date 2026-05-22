# CC Switch（強化フォーク）

**[CC Switch](https://github.com/farion1231/cc-switch) の強化フォーク — AI CLI ツール向けに画像整流器とマルチプロバイダー視覚分析スキルを追加。**

[English](README_ZH.md) | [中文](README.md) | 日本語

---

## なぜこのプロジェクトが必要なのか？

**AI CLI ツール + サードパーティモデル = 画像理解の空白地帯。** これは Claude Code や OpenCode などのツールを DeepSeek のようなマルチモーダル非対応モデルと組み合わせたときに直面する、最も厄介な問題です。

### 従来のアプローチの限界

よくある回避策は、グローバル `CLAUDE.md` にルールを追加して、画像処理に Skill や MCP の使用を強制することです。しかし、ここには避けられない落とし穴があります：

**Claude Code はシステムレベルで命令をハードコードしています — 画像を貼り付けると必ず `read` ツールを呼び出し、画像を直接モデルに送信します。** このシステム命令は `CLAUDE.md` より優先度が高いため、あなたのルールでは阻止できません。

結果は？Ctrl+V でスクリーンショットを貼り付けると、モデルは生の base64 画像データを受け取り、すぐにエラーを返すか、意味不明な応答をします。**会話全体が修復不可能なほど壊れてしまいます。**

![画像整流器オフ時、Claude Code + DeepSeek の会話が壊れる様子](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

さらに悪いことに、デバッグ作業中はスクリーンショットをわざわざファイルに保存しません — スクリーンショット → クリップボード → Ctrl+V が自然なワークフローです。従来のアプローチは「先にファイル保存 → パスを指定」というシナリオにしか対応できず、実際の使用習慣と完全にずれています。

### このプロジェクトの解決策

**画像整流器 + 画像分析スキル — 二層の防御線。**

| シナリオ | 従来のアプローチ | 本プロジェクト |
|------|---------|--------|
| Ctrl+V でクリップボード画像を Claude Code に貼り付け | ❌ システムが `read` をハードコード、モデルに直接送信、会話が壊れる | ✅ 整流器がプロキシレイヤーでインターセプト、テキストプロンプトに置換、モデルを Skill 呼び出しに誘導 |
| 画像ファイルパスを指定 | ⚠️ `CLAUDE.md` ルールがかろうじて機能するが、CLI ごとに動作が異なる | ✅ 整流器が統一的にインターセプト、MD ファイルルールに依存しない |
| OpenCode でクリップボード画像を貼り付け | ❌ OpenCode は一時ファイルを生成しないため、パスすら取得できない | ✅ Skill スクリプトが自動検出：パスがあればファイルを読み、なければクリップボードから直接読み取り |
| マルチプロバイダー切り替え | ❌ 単一モデルがダウンするとすべて停止 | ✅ フォールバック機構、30以上のプロバイダーを自動切り替え |

> **整流器オフ：** Claude Code + DeepSeek、Ctrl+V で画像貼り付け後すぐに会話が壊れる

![claudecode+DeepSeek关闭图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek关闭图片整流器运行示意图.png)

> **整流器オン：** Claude Code + DeepSeek、画像が正常にインターセプトされ Skill 呼び出しに誘導

![claudecode+DeepSeek启用图片整流器运行示意图](assets/screenshots/claudecode+DeepSeek启用图片整流器运行示意图.png)

> **OpenCode + DeepSeek：** Skill が一時ファイルなしを検出し、クリップボードから直接読み取り

![opencode+DeepSeek运行示意图](assets/screenshots/opencode+DeepSeek运行示意图.png)

### 二層の防御の仕組み

1. **画像整流器（プロキシレイヤー）** — リクエスト転送前にインターセプト。messages 内の base64 画像ブロックを検出し、生データ（通常数 MB）を削除して、指定された Skill を呼び出すようモデルに指示するテキストプロンプトに置き換えます。**この処理はモデルがリクエストを見る前に完了します。**

2. **画像分析スキル（CLI ツール）** — モデルに呼び出された後、画像のソースをインテリジェントに判断：パスが指定されていればローカルファイルを読み、パスがなければシステムクリップボードから直接読み取ります。**Claude Code（貼り付け時に一時ファイル生成）と OpenCode（貼り付け時に一時ファイル非生成）の両方の動作に互換性があります。**

> **注意：** 本プロジェクトは現在 **Claude Code** と **OpenCode** でのみ十分にテストされています。その他の AI CLI ツール（Codex、Gemini CLI、OpenClaw、Hermes など）も理論上は動作しますが、未検証です。自身でのテストとフィードバックをお待ちしています。

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

**設定方法：** ルーター設定 → 整流器セクション → 画像整流器。スイッチをオンにし、必要に応じてスキル名を変更します。

![画像整流器スイッチ](assets/screenshots/图片整流器开关示意图.png)

---

### 2. 画像分析スキル（Image Analysis Skill）

シェルコマンドを実行できる任意の AI CLI と互換性のある、スタンドアロンの Python コマンドライン画像視覚認識ツールです。

**主な機能：**
- ローカル画像（jpg/png/gif/webp/bmp）、ネットワーク画像 URL、システムクリップボード（macOS AppleScript）に対応
- 複数回の `--image` による画像比較
- フォールバック機構：設定順にすべてのプロバイダーを試行し、失敗時に自動切り替え。最初の成功結果が `[プロバイダー名]` ラベル付きで返されます

**30 以上のプリセット AI プロバイダー：**

| プラットフォーム | 代表的なモデル |
|------|------|
| Volcengine（火山引擎） | Doubao Seed 2.0 Pro/Lite/Mini、Vision 250815 |
| SiliconFlow（硅基流动） | Qwen3.6-35B-A3B、Qwen3.6-27B |
| Alibaba Bailian（阿里百煉） | Qwen3.6 Plus/Flash、Qwen3.5 Omni、Kimi K2.6、MiniMax M2.5 |
| Zhipu（智譜） | GLM-4.6V-Flash |
| SenseNova（商湯） | SenseNova-6.7-Flash-Lite |

**画像整流器との連携：** モデルが画像に対応していない場合、整流器が画像ブロックをテキストプロンプトに置き換え、モデルを Skill 呼び出しに誘導します。モデルは自動的に CLI ツールを呼び出して実際の視覚分析を実行します。プロセス全体がユーザーに対して透過的で、Python スクリプトを手動で実行する必要はありません。

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

> **プロキシアドレスについて：** `dev.sh` の 10-11 行目にプリセットされているプロキシアドレス `http://127.0.0.1:7890` はサンプル値です。中国国内のユーザーは通常、Rust 依存関係を取得するためにプロキシが必要です。ローカルのプロキシソフトウェアのポート番号に合わせて、`dev.sh` 内の以下の 2 行を変更してください：
>
> ```bash
> export CARGO_HTTP_PROXY=http://127.0.0.1:7890
> export CARGO_HTTPS_PROXY=http://127.0.0.1:7890
> ```
>
> `7890` をローカルプロキシの実際のポート番号に置き換えてください（例：Clash デフォルト 7890、V2Ray デフォルト 10809、手動構築の場合は各自設定）。

---

## インストール

ソースからビルド：

```bash
git clone https://github.com/piaomiaoguying/cc-switch.git
cd cc-switch
./dev.sh build
```

---

## セットアップガイド

### ステップ 1：AI プロバイダーに登録して API キーを取得

本 Skill には複数プラットフォームの視覚モデルがプリセットされており、いずれも無料枠があります。好みのプラットフォームを選んで登録してください：

| プラットフォーム | 代表的なモデル | 登録 URL |
|------|---------|---------|
| Alibaba Bailian（阿里百煉） | Qwen3.6 Plus/Flash、Qwen3.5 Omni など | [dashscope.aliyun.com](https://dashscope.aliyun.com) |
| Zhipu（智譜） | GLM-4.6V-Flash | [open.bigmodel.cn](https://open.bigmodel.cn) |
| SiliconFlow（硅基流动） | Qwen3.6-35B-A3B など | [siliconflow.cn](https://siliconflow.cn) |
| Volcengine（火山引擎） | Doubao Seed 2.0 Pro/Lite/Mini | [console.volcengine.com](https://console.volcengine.com) |
| SenseNova（商湯） | SenseNova-6.7-Flash-Lite | [platform.sensenova.cn](https://platform.sensenova.cn) |

登録後、API キーを取得し、Skill 設定ファイルに記入してください：

```bash
cp skills/image-analysis/scripts/config.example.json skills/image-analysis/scripts/config.json
# config.json を編集し、各プラットフォームの API キーを対応するフィールドに入力
```

### ステップ 2：グローバル CLAUDE.md の設定

グローバル `CLAUDE.md`（`~/.claude/CLAUDE.md`）に以下のルールを追加し、画像に対する `read` ツールの直接使用を禁止します：

```markdown
## 画像処理ルール
`read` ツールを使用して画像ファイル（.png, .jpg, .jpeg など）を読み取ってはいけません
画像分析には必ず image-analysis スキルを使用してください
モデルが直接画像入力に対応していない場合は、自動的に image-analysis スキルを呼び出して画像を分析してください
```

### ステップ 3：Skill を AI CLI にインポート

手動でフォルダをコピーするよりも、CC Switch を通じて Skill を管理することをお勧めします：

1. `skills/image-analysis` 設定フォルダを CC Switch ディレクトリ配下に配置
2. CC Switch → **Skills 管理** を開く
3. Claude Code と OpenCode の Skill スイッチで `image-analysis` を有効にする

これで CC Switch が自動的に Skill を各 CLI の Skill ディレクトリに同期します。

### ステップ 4：CC Switch のルーターと整流器の設定

これは最も重要なステップで、3 層のスイッチをオンにする必要があります：

**① ローカルルーティングを有効にする**

CC Switch → **ルーター設定** に移動：
- **ローカルルーティングマスタースイッチ** をオン
- **Claude Code** ルーティングスイッチをオン（よくある見落とし：マスタースイッチはオンでも Claude Code スイッチがオフ）

**② 画像整流器の設定**

ルーター設定画面を下にスクロールし、**整流器** セクションを探す：
- **整流器マスタースイッチ** をオン
- **画像整流器** スイッチをオン
- Skill 名フィールドに `image-analysis` と入力

![画像整流器スイッチ](assets/screenshots/图片整流器开关示意图.png)

### 完了

上記の設定が完了すると、通常は既存の Claude Code インスタンスにすぐに反映されます。もし反映されない場合は、新しい Claude Code インスタンスを再起動してください。モデルのネットワークリクエストは CC Switch プロキシを経由し、CC Switch が会話データをインターセプトして変更します：

> モデルが対応していない画像データ（base64）を強制的にテキストコンテンツに置き換えます。テキストが大規模モデルを誘導し、あなたの `image-analysis` Skill を呼び出して画像理解を完了させます。

これ以降の全体フロー：

```
Ctrl+V で画像貼り付け → CC Switch プロキシがインターセプト → base64 を削除、テキストプロンプトに置換
→ モデルがテキストを受信、image-analysis Skill を呼び出し → Skill が画像を読み取り分析結果を返す
```

---

## ドキュメント

- [オリジナル CC Switch リポジトリ](https://github.com/farion1231/cc-switch)

---

## ライセンス

MIT — オリジナルプロジェクトと同じです。
