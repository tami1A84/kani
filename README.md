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

## 🚀 オンボーディング (Onboarding)

`kani-nostr-cli` を初めて使う方のために、ウィザード形式で簡単に鍵を生成する方法を紹介します。リポジトリをクローンした直後など、まだインストールしていない場合は以下のコマンドを実行してください。

```bash
cargo run -- key generate --wizard
```

すでに `cargo install` を実行済みの場合は、以下のコマンドで実行できます。

```bash
kani-nostr-cli key generate --wizard
```

このコマンドを実行すると、対話形式でニーモニック（秘密のバックアップフレーズ）の表示、パスワードによる秘密鍵の暗号化（NIP-49）までを簡単に行うことができます。

---

## 🚀 使い方 (Usage)

`kani-nostr-cli` はサブコマンドベースで動作します。

```
kani-nostr-cli <COMMAND>
```

### 主要コマンド

<details>
<summary>🔐 <strong>login</strong> - ログイン</summary>

**使用方法:** `kani-nostr-cli login`

暗号化された秘密鍵を復号し、シェル環境に読み込みます。

**入力例:**
```bash
eval $(kani-nostr-cli login)
```
</details>

<details>
<summary>🔓 <strong>logout</strong> - ログアウト</summary>

**使用方法:** `kani-nostr-cli logout`

シェル環境から秘密鍵をクリアします。

**入力例:**
```bash
eval $(kani-nostr-cli logout)
```
</details>

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
| `edit-profile`           | プロフィールを対話的に編集します (NIP-01)            |

**入力例 (`create-text-note`):**
```bash
kani-nostr-cli event create-text-note --relay wss://relay.damus.io --secret-key <nsec_secret_key> "Hello, Nostr!"
```
</details>

<details>
<summary>👥 <strong>contact</strong> - コンタクトリスト管理</summary>

**使用方法:** `kani-nostr-cli contact <SUBCOMMAND>`

| サブコマンド | 説明                                     |
| ------------ | ---------------------------------------- |
| `add`        | コンタクトリストに公開鍵を追加します     |
| `list`       | 公開鍵のコンタクトリストを表示します     |

**入力例 (`add`):**
```bash
kani-nostr-cli contact add <npub_key_1> <npub_key_2> --secret-key <nsec_secret_key>
```
</details>

<details>
<summary>📡 <strong>relay</strong> - リレーリスト管理</summary>

**使用方法:** `kani-nostr-cli relay <SUBCOMMAND>`

| サブコマンド | 説明                                                   |
| ------------ | ------------------------------------------------------ |
| `set`        | リレーリストを設定します (NIP-65)                      |
| `get`        | リレーリストを取得します (NIP-65)                      |
| `edit`       | エディタでリレーリストを対話的に編集します (NIP-65)    |

**入力例 (`get`):**
```bash
kani-nostr-cli relay get --pubkey <npub_key> --relay wss://relay.damus.io
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

<details>
<summary>⚙️ <strong>config</strong> - 設定管理</summary>

**使用方法:** `kani-nostr-cli config <SUBCOMMAND>`

| サブコマンド | 説明                             |
| ------------ | -------------------------------- |
| `path`       | 設定ファイルのパスを表示します   |

**入力例 (`path`):**
```bash
kani-nostr-cli config path
```
</details>

## 📄 ライセンス (License)

このプロジェクトは[MITライセンス](LICENSE)の下で公開されています。
