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

  **入力例:**
  ```
  kani key generate
  ```

- `from-mnemonic`: ニーモニックから鍵を導出します

  **入力例:**
  ```
  kani key from-mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
  ```

### `event`
イベント管理

**使用方法:**
```
kani event <COMMAND>
```

**サブコマンド:**
- `create-text-note`: テキスト投稿を作成します

  **入力例:**
  ```
  kani event create-text-note --relay wss://relay.damus.io --secret-key nsec1... "Hello, Nostr!"
  ```

- `get`: IDでイベントを取得します

  **入力例:**
  ```
  kani event get --relay wss://relay.damus.io note1...
  ```

- `delete`: IDでイベントを削除します

  **入力例:**
  ```
  kani event delete --relay wss://relay.damus.io --secret-key nsec1... note1...
  ```

### `contact`
コンタクトリスト管理

**使用方法:**
```
kani contact <COMMAND>
```

**サブコマンド:**
- `set`: コンタクトリストを設定します

  **入力例:**
  ```
  kani contact set --relay wss://relay.damus.io --secret-key nsec1... npub1... npub1...
  ```

- `get`: コンタクトリストを取得します

  **入力例:**
  ```
  kani contact get --relay wss://relay.damus.io npub1...
  ```

### `nip19`
NIP-19 bech32エンコーディング

**使用方法:**
```
kani nip19 <COMMAND>
```

**サブコマンド:**
- `encode`: エンティティをbech32形式にエンコードします
  - `npub`: 公開鍵をnpub形式にエンコードします

    **入力例:**
    ```
    kani nip19 encode npub <hex_public_key>
    ```
  - `nsec`: 秘密鍵をnsec形式にエンコードします

    **入力例:**
    ```
    kani nip19 encode nsec <hex_secret_key>
    ```
  - `note`: イベントIDをnote形式にエンコードします

    **入力例:**
    ```
    kani nip19 encode note <hex_event_id>
    ```
  - `nprofile`: プロファイルをnprofile形式にエンコードします

    **入力例:**
    ```
    kani nip19 encode nprofile <hex_public_key> wss://relay.one wss://relay.two
    ```
  - `nevent`: イベントをnevent形式にエンコードします

    **入力例:**
    ```
    kani nip19 encode nevent <hex_event_id> --author-pubkey <hex_public_key> --kind 1 wss://relay.one
    ```
- `decode`: bech32文字列をデコードします

  **入力例:**
  ```
  kani nip19 decode npub1...
  ```

### `uri`
NIP-21 nostr URIのパース

**使用方法:**
```
kani uri <URI>
```

**説明:**
`nostr:`で始まるURIをパースして、その内容を表示します。

**入力例:**
```
kani uri nostr:npub1...
```
