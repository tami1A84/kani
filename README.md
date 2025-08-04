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

### `relay`
リレー管理

**使用方法:**
```
kani relay <COMMAND>
```

**サブコマンド:**
- `info`: リレー情報ドキュメント(NIP-11)を取得します
- `get`: ユーザーのリレーリスト(NIP-65)を取得します
- `set`: あなたのリレーリスト(NIP-65)を設定します

### `nip19`
NIP-19 bech32エンコーディング

**使用方法:**
```
kani nip19 <COMMAND>
```

**サブコマンド:**
- `encode`: エンティティをbech32形式にエンコードします
  - `npub`: 公開鍵をnpub形式にエンコードします
  - `nsec`: 秘密鍵をnsec形式にエンコードします
  - `note`: イベントIDをnote形式にエンコードします
  - `nprofile`: プロファイルをnprofile形式にエンコードします
  - `nevent`: イベントをnevent形式にエンコードします
- `decode`: bech32文字列をデコードします

### `uri`
NIP-21 nostr URIのパース

**使用方法:**
```
kani uri <URI>
```

**説明:**
`nostr:`で始まるURIをパースして、その内容を表示します。

### `connect`
NIP-46 Nostr Connect

**使用方法:**
```
kani connect --relay <RELAY_URL> --client-name <CLIENT_NAME>
```

**説明:**
NIP-46 Nostr Connectプロトコルを使用して、リレー経由でクライアントに接続します。
