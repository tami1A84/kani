# kani-nostr-cli
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
- `from-mnemonic`: ニーモニックから鍵を導出します (NIP-06)

  **入力例:**
  ```
  kani key from-mnemonic "mnemonic phrase here"
  ```
- `encrypt`: パスワードで秘密鍵を暗号化します (NIP-49)

  **入力例:**
  ```
  kani key encrypt --secret-key <nsec_secret_key> --password <password>
  ```
- `decrypt`: 暗号化された秘密鍵をパスワードで復号します (NIP-49)

  **入力例:**
  ```
  kani key decrypt --encrypted-key <ncryptsec_encrypted_key> --password <password>
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
  kani event create-text-note --relay wss://relay.damus.io --secret-key <nsec_secret_key> "Hello, Nostr!"
  ```
  - `--gift-wrap-recipient <npub>`: 指定した公開鍵の受信者でイベントをギフトラップします (NIP-59)

  **入力例 (ギフトラップ):**
  ```
  kani event create-text-note --relay wss://relay.damus.io --secret-key <nsec_secret_key> --gift-wrap-recipient <npub_recipient_key> "This is a wrapped note"
  ```
- `create-dm`: ダイレクトメッセージを作成します (NIP-04)

  **入力例:**
  ```
  kani event create-dm --relay wss://relay.damus.io --secret-key <nsec_secret_key> --recipient <npub_recipient_key> "Hello, secret world!"
  ```
- `get`: IDでイベントを取得します

  **入力例:**
  ```
  kani event get --relay wss://relay.damus.io <note_event_id>
  ```
- `delete`: IDでイベントを削除します

  **入力例:**
  ```
  kani event delete --relay wss://relay.damus.io --secret-key <nsec_secret_key> <note_event_id_to_delete>
  ```
- `encrypt-payload`: ペイロードを暗о化します (NIP-44)

  **入力例:**
  ```
  kani event encrypt-payload --secret-key <nsec_secret_key> --recipient <npub_recipient_key> "some sensitive data"
  ```
- `decrypt-payload`: ペイロードを復号します (NIP-44)

  **入力例:**
  ```
  kani event decrypt-payload --secret-key <nsec_secret_key> --sender <npub_sender_key> <encrypted_payload>
  ```
- `create-long-form-post`: 長文コンテンツ投稿を作成します (NIP-23)

  **入力例:**
  ```
  kani event create-long-form-post --relay wss://relay.damus.io --secret-key <nsec_secret_key> --file path/to/article.md --title "My Article" --summary "A summary of my article."
  ```

### `contact`
コンタクトリスト管理

**使用方法:**
```
kani contact <COMMAND>
```

**サブコマンド:**
- `set`: コンタクトリストを設定します (NIP-02)

  **入力例:**
  ```
  kani contact set --relay wss://relay.damus.io --secret-key <nsec_secret_key> <npub_key_1> <npub_key_2>
  ```
- `get`: コンタクトリストを取得します (NIP-02)

  **入力例:**
  ```
  kani contact get --relay wss://relay.damus.io <npub_key>
  ```
- `set-relays`: リレーリストを設定します (NIP-65)

  **入力例:**
  ```
  kani contact set-relays --relay wss://relay.damus.io --secret-key <nsec_secret_key> wss://relay.one#read wss://relay.two#write
  ```
- `get-relays`: リレーリストを取得します (NIP-65)

  **入力例:**
  ```
  kani contact get-relays --relay wss://relay.damus.io --pubkey <npub_key>
  ```

### `nip05`
DNSベースの識別子 (NIP-05)

**使用方法:**
```
kani nip05 <COMMAND>
```
**サブコマンド:**
- `verify`: NIP-05識別子を検証します

  **入力例:**
  ```
  kani nip05 verify --nip05 user@example.com --pubkey <npub_key>
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
  kani nip19 decode <bech32_string>
  ```

### `nip46`
Nostr Connect (NIP-46)

**使用方法:**
```
kani nip46 <COMMAND>
```

**サブコマンド:**
- `get-public-key`: リモート署名者から公開鍵を取得します

  **入力例:**
  ```
  kani nip46 get-public-key "nostrconnect://<bunker_hex_pubkey>?relay=<relay_url>" --secret-key <local_nsec_key>
  ```
- `sign-event`: リモート署名者でイベントに署名します

  **入力例:**
  ```
  kani nip46 sign-event "nostrconnect://<bunker_hex_pubkey>?relay=<relay_url>" --secret-key <local_nsec_key> '{"kind":1,"content":"...","tags":[],"created_at":...}'
  ```

### `nip47`
Nostr Wallet Connect (NIP-47)

**使用方法:**
```
kani nip47 <COMMAND>
```

**サブコマンド:**
- `get-info`: ウォレットから情報を取得します

  **入力例:**
  ```
  kani nip47 get-info "nostr+walletconnect://<wallet_hex_pubkey>?relay=<relay_url>&secret=<hex_secret>"
  ```
- `get-balance`: ウォレットから残高を取得します

  **入力例:**
  ```
  kani nip47 get-balance "nostr+walletconnect://<wallet_hex_pubkey>?relay=<relay_url>&secret=<hex_secret>"
  ```
- `pay-invoice`: ウォレットで請求書を支払います

  **入力例:**
  ```
  kani nip47 pay-invoice "nostr+walletconnect://<wallet_hex_pubkey>?relay=<relay_url>&secret=<hex_secret>" <bolt11_invoice>
  ```

### `uri`
NIP-21 nostr URIのパース

**使用方法:**
```
kani uri <URI>
```

**入力例:**
```
kani uri nostr:npub1...
```
