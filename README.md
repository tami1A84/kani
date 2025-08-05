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
- `from-mnemonic`: ニーモニックから鍵を導出します (NIP-06)
- `encrypt`: パスワードで秘密鍵を暗号化します (NIP-49)
- `decrypt`: 暗号化された秘密鍵をパスワードで復号します (NIP-49)

### `event`
イベント管理

**使用方法:**
```
kani event <COMMAND>
```

**サブコマンド:**
- `create-text-note`: テキスト投稿を作成します
  - `--gift-wrap-recipient <npub>`: 指定した公開鍵の受信者でイベントをギフトラップします (NIP-59)
- `create-dm`: ダイレクトメッセージを作成します (NIP-04)
- `get`: IDでイベントを取得します
- `delete`: IDでイベントを削除します
- `encrypt-payload`: ペイロードを暗号化します (NIP-44)
- `decrypt-payload`: ペイロードを復号します (NIP-44)

### `contact`
コンタクトリスト管理

**使用方法:**
```
kani contact <COMMAND>
```

**サブコマンド:**
- `set`: コンタクトリストを設定します (NIP-02)
- `get`: コンタクトリストを取得します (NIP-02)
- `set-relays`: リレーリストを設定します (NIP-65)
- `get-relays`: リレーリストを取得します (NIP-65)

### `nip05`
DNSベースの識別子 (NIP-05)

**使用方法:**
```
kani nip05 <COMMAND>
```
**サブコマンド:**
- `verify`: NIP-05識別子を検証します

### `nip19`
bech32エンコーディング (NIP-19)

**使用方法:**
```
kani nip19 <COMMAND>
```

**サブコマンド:**
- `encode`: エンティティをbech32形式にエンコードします
- `decode`: bech32文字列をデコードします

### `nip46`
Nostr Connect (NIP-46)

**使用方法:**
```
kani nip46 <COMMAND>
```

**サブコマンド:**
- `get-public-key`: リモート署名者から公開鍵を取得します
- `sign-event`: リモート署名者でイベントに署名します

### `nip47`
Nostr Wallet Connect (NIP-47)

**使用方法:**
```
kani nip47 <COMMAND>
```

**サブコマンド:**
- `get-info`: ウォレットから情報を取得します
- `get-balance`: ウォレットから残高を取得します
- `pay-invoice`: ウォレットで請求書を支払います

### `uri`
NIP-21 nostr URIのパース

**使用方法:**
```
kani uri <URI>
```
