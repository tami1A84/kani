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
- `delete`: IDでイベントを削除します

### `contact`
コンタクトリスト管理

**使用方法:**
```
kani contact <COMMAND>
```

**サブコマンド:**
- `set`: コンタクトリストを設定します
- `get`: コンタクトリストを取得します

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
