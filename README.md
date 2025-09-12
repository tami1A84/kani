# kani

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`kani` は、Nostrプロトコルのための多機能コマンドラインインターフェース（CLI）ツールです。鍵の生成からイベントの送受信、Nostr Connectまで、さまざまなNIPをサポートし、Nostrエコシステムでの開発と対話を容易にします。

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

現在、`kani` は crates.io に公開されていません。ソースコードからビルド・インストールしてください。

```bash
# リポジトリをクローン (URLは適宜変更してください)
git clone https://github.com/tami1A84/kani-nostr-cli.git
cd kani-nostr-cli

# ビルド & インストール
cargo install --path .
```

## 🚀 使い方 (Usage)

`kani` はサブコマンドベースで動作します。

```
kani <COMMAND>
```

### 主要コマンド

<details>
<summary>🔑 <strong>key</strong> - 鍵管理</summary>

**使用方法:** `kani key <SUBCOMMAND>`

| サブコマンド      | 説明                                        |
| ----------------- | ------------------------------------------- |
| `generate`        | 新しい鍵を生成します                        |
| `from-mnemonic`   | ニーモニックから鍵を導出します (NIP-06)     |
| `encrypt`         | 秘密鍵をパスワードで暗号化します (NIP-49)   |
| `decrypt`         | 暗号化された秘密鍵を復号します (NIP-49)     |

**入力例 (`generate`):**
```bash
kani key generate
```
</details>

<details>
<summary>⚡️ <strong>event</strong> - イベント管理</summary>

**使用方法:** `kani event <SUBCOMMAND>`

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
kani event create-text-note --relay wss://relay.damus.io --secret-key <nsec_secret_key> "Hello, Nostr!"
```
</details>

<details>
<summary>👥 <strong>contact</strong> - コンタクトリスト管理</summary>

**使用方法:** `kani contact <SUBCOMMAND>`

| サブコマンド     | 説明                                        |
| ---------------- | ------------------------------------------- |
| `set`            | コンタクトリストを設定します (NIP-02)       |
| `get`            | コンタクトリストを取得します (NIP-02)       |
| `set-relays`     | リレーリストを設定します (NIP-65)           |
| `get-relays`     | リレーリストを取得します (NIP-65)           |

**入力例 (`set`):**
```bash
kani contact set --relay wss://relay.damus.io --secret-key <nsec_secret_key> <npub_key_1> <npub_key_2>
```
</details>

<details>
<summary>🌐 <strong>nip05</strong> - DNSベースの識別子</summary>

**使用方法:** `kani nip05 <SUBCOMMAND>`

| サブコマンド | 説明                             |
| ------------ | -------------------------------- |
| `verify`     | NIP-05識別子を検証します         |

**入力例 (`verify`):**
```bash
kani nip05 verify --nip05 user@example.com --pubkey <npub_key>
```
</details>

<details>
<summary>🔗 <strong>nip19</strong> - bech32エンコーディング</summary>

**使用方法:** `kani nip19 <SUBCOMMAND>`

| サブコマンド | 説明                                    |
| ------------ | --------------------------------------- |
| `encode`     | エンティティをbech32形式にエンコードします |
| `decode`     | bech32文字列をデコードします            |

**入力例 (`encode npub`):**
```bash
kani nip19 encode npub <hex_public_key>
```
</details>

<details>
<summary>🔌 <strong>nip46</strong> - Nostr Connect</summary>

**使用方法:** `kani nip46 <SUBCOMMAND>`

| サブコマンド       | 説明                                        |
| ------------------ | ------------------------------------------- |
| `get-public-key`   | リモート署名者から公開鍵を取得します        |
| `sign-event`       | リモート署名者でイベントに署名します        |

**入力例 (`get-public-key`):**
```bash
kani nip46 get-public-key "nostrconnect://<bunker_hex_pubkey>?relay=<relay_url>" --secret-key <local_nsec_key>
```
</details>

<details>
<summary>💰 <strong>nip47</strong> - Nostr Wallet Connect</summary>

**使用方法:** `kani nip47 <SUBCOMMAND>`

| サブコマンド    | 説明                             |
| --------------- | -------------------------------- |
| `get-info`      | ウォレットから情報を取得します   |
| `get-balance`   | ウォレットから残高を取得します   |
| `pay-invoice`   | ウォレットで請求書を支払います   |

**入力例 (`get-info`):**
```bash
kani nip47 get-info "nostr+walletconnect://<wallet_hex_pubkey>?relay=<relay_url>&secret=<hex_secret>"
```
</details>

<details>
<summary>🏷️ <strong>uri</strong> - NIP-21 URIのパース</summary>

**使用方法:** `kani uri <URI>`

**入力例:**
```bash
kani uri nostr:npub1...
```
</details>

## 📄 ライセンス (License)

このプロジェクトは[MITライセンス](LICENSE)の下で公開されています。
