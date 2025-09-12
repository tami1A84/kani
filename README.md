# kani-nostr-cli

[![Crates.io](https://img.shields.io/crates/v/kani-nostr-cli.svg)](https://crates.io/crates/kani-nostr-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`kani-nostr-cli` は、Nostrプロトコルのための多機能コマンドラインインターフェース（CLI）ツールです。鍵の生成からイベントの送受信、Nostr Connectまで、さまざまなNIPをサポートし、Nostrエコシステムでの開発と対話を容易にします。

## ✨ 特徴 (Features)

- **鍵管理**: 鍵の生成、ニーモニックからの導出、暗号化/復号化 (NIP-06, NIP-49)
- **イベント操作**: テキスト投稿、イベント取得、削除、ギフトラップ (NIP-59)、長文コンテンツ (NIP-23)
- **暗号化通信**: 暗号化/復号化ペイロード (NIP-44)
- **コンタクトとリレーリスト**: フォローリスト (NIP-02) とリレーリスト (NIP-65) の管理
- **ID検証**: DNSベースの識別子検証 (NIP-05)
- **Bech32エンコーディング**: `npub`, `nsec`, `note` などのエンコード/デコード (NIP-19)
- **リモート署名**: Nostr Connectによるリモート署名 (NIP-46)
- **ウォレット連携**: Nostr Wallet Connectによる残高確認や支払い (NIP-47)
- **URIパース**: `nostr:` URIの解析 (NIP-21)

## 📦 インストール (Installation)

`kani-nostr-cli` は crates.io に公開されています。`cargo` を使ってインストールできます。

```bash
cargo install kani-nostr-cli
```

## 🚀 使い方 (Usage)

`kani-nostr-cli` はサブコマンドベースで動作します。

```
kani-nostr-cli <COMMAND>
```

### 主要コマンド

<details>
<summary>🔑 <strong>key</strong> - 鍵管理</summary>

**使用方法:** `kani-nostr-cli key <SUBCOMMAND>`

| サブコマンド      | 説明                                        |
| ----------------- | ------------------------------------------- |
| `generate`        | 新しい鍵を生成します                        |
| `from-mnemonic`   | ニーモニックから鍵を導出します (NIP-06)     |
| `encrypt`         | 秘密鍵をパスワードで暗号化します (NIP-49)   |
| `decrypt`         | 暗号化された秘密鍵を復号します (NIP-49)     |

**入力例 (`generate`):**
```bash
kani-nostr-cli key generate
```
</details>

<details>
<summary>⚡️ <strong>event</strong> - イベント管理</summary>

**使用方法:** `kani-nostr-cli event <SUBCOMMAND>`

| サブコマンド             | 説明                                                   |
| ------------------------ | ------------------------------------------------------ |
| `create-text-note`       | テキスト投稿を作成します (NIP-59ギフトラップ対応)      |
| `get`                    | IDでイベントを取得します                               |
| `delete`                 | IDでイベントを削除します                               |
| `encrypt-payload`        | ペイロードを暗号化します (NIP-44)                      |
| `decrypt-payload`        | ペイロードを復号します (NIP-44)                      |
| `create-long-form-post`  | 長文コンテンツ投稿を作成します (NIP-23)              |

**入力例 (`create-text-note`):**
```bash
kani-nostr-cli event create-text-note --relay wss://relay.damus.io --secret-key <nsec_secret_key> "Hello, Nostr!"
```
</details>

<details>
<summary>👥 <strong>contact</strong> - コンタクトリスト管理</summary>

**使用方法:** `kani-nostr-cli contact <SUBCOMMAND>`

| サブコマンド     | 説明                                        |
| ---------------- | ------------------------------------------- |
| `set`            | コンタクトリストを設定します (NIP-02)       |
| `get`            | コンタクトリストを取得します (NIP-02)       |
| `set-relays`     | リレーリストを設定します (NIP-65)           |
| `get-relays`     | リレーリストを取得します (NIP-65)           |

**入力例 (`set`):**
```bash
kani-nostr-cli contact set --relay wss://relay.damus.io --secret-key <nsec_secret_key> <npub_key_1> <npub_key_2>
```
</details>

<details>
<summary>🌐 <strong>nip05</strong> - DNSベースの識別子</summary>

**使用方法:** `kani-nostr-cli nip05 <SUBCOMMAND>`

| サブコマンド | 説明                             |
| ------------ | -------------------------------- |
| `verify`     | NIP-05識別子を検証します         |

**入力例 (`verify`):**
```bash
kani-nostr-cli nip05 verify --nip05 user@example.com --pubkey <npub_key>
```
</details>

<details>
<summary>🔗 <strong>nip19</strong> - bech32エンコーディング</summary>

**使用方法:** `kani-nostr-cli nip19 <SUBCOMMAND>`

| サブコマンド | 説明                                    |
| ------------ | --------------------------------------- |
| `encode`     | エンティティをbech32形式にエンコードします |
| `decode`     | bech32文字列をデコードします            |

**入力例 (`encode npub`):**
```bash
kani-nostr-cli nip19 encode npub <hex_public_key>
```
</details>

<details>
<summary>🔌 <strong>nip46</strong> - Nostr Connect</summary>

**使用方法:** `kani-nostr-cli nip46 <SUBCOMMAND>`

| サブコマンド       | 説明                                        |
| ------------------ | ------------------------------------------- |
| `get-public-key`   | リモート署名者から公開鍵を取得します        |
| `sign-event`       | リモート署名者でイベントに署名します        |

**入力例 (`get-public-key`):**
```bash
kani-nostr-cli nip46 get-public-key "nostrconnect://<bunker_hex_pubkey>?relay=<relay_url>" --secret-key <local_nsec_key>
```
</details>

<details>
<summary>💰 <strong>nip47</strong> - Nostr Wallet Connect</summary>

**使用方法:** `kani-nostr-cli nip47 <SUBCOMMAND>`

| サブコマンド    | 説明                             |
| --------------- | -------------------------------- |
| `get-info`      | ウォレットから情報を取得します   |
| `get-balance`   | ウォレットから残高を取得します   |
| `pay-invoice`   | ウォレットで請求書を支払います   |

**入力例 (`get-info`):**
```bash
kani-nostr-cli nip47 get-info "nostr+walletconnect://<wallet_hex_pubkey>?relay=<relay_url>&secret=<hex_secret>"
```
</details>

<details>
<summary>🏷️ <strong>uri</strong> - NIP-21 URIのパース</summary>

**使用方法:** `kani-nostr-cli uri <URI>`

**入力例:**
```bash
kani-nostr-cli uri nostr:npub1...
```
</details>

## 📄 ライセンス (License)

このプロジェクトは[MITライセンス](LICENSE)の下で公開されています。
