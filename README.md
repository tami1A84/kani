# kani
rust-nostr CLIツール

## コマンド

### `key`
鍵管理

**使用方法:**
```
kani key <COMMAND>
```

**サブコマンド:**
- `generate`: 新しい鍵を生成します
- `from-mnemonic`: ニーモニックから鍵を導出します

### `event`
イベント管理

**使用方法:**
```
kani event <COMMAND>
```

**サブコマンド:**
- `create-text-note`: テキスト投稿を作成します
- `get`: IDでイベントを取得します

### `contact`
コンタクトリスト管理

**使用方法:**
```
kani contact <COMMAND>
```

**サブコマンド:**
- `set`: コンタクトリストを設定します
- `get`: コンタクトリストを取得します

### `ots`
OpenTimestamps

**使用方法:**
```
kani ots <COMMAND>
```

**サブコマンド:**
- `attest`: イベントを証明します

### `dm`
ダイレクトメッセージ

**使用方法:**
```
kani dm <COMMAND>
```

**サブコマンド:**
- `send`: ダイレクトメッセージを送信します
- `receive`: ダイレクトメッセージを受信します

### `nip05`
NIP-05

**使用方法:**
```
kani nip05 <COMMAND>
```

**サブコマンド:**
- `verify`: NIP-05識別子を検証します
