# rust-bot

### 最終目標：誕生日bot
### 小目標：オウム返しbot
### 現状：リクエストが返却できない

## 参考
- ZliTechbookV3 第4章 LINEのおうむ返しbotをRustで作ってみよう
- 実践Rust入門
など

# 基礎知識メモ

## 立ち上げ方
```cargo run```
```ngrok http 8080```
forwardingのurlをコピーし、``/webhook``を付け加え、LINE developersのwebhook urlに貼り付け
localhost:8080で確認可能

ログレベル別の立ち上げ方の例（参照：[env_loggerドキュメント](https://docs.rs/env_logger/latest/env_logger/)）
``RUST_LOG=info ./main``

## 命名規則
| | 例 |
| --- | --- |
| クラス名 | ``CamelCase`` |
| 普通の関数 | ``pageHeight()`` |
| 変数 | ``lower_separated`` |
| クラスのメンバ変数　| ``offset_`` |

## ツールチェインのアップデート
```
rustup update
```

## logの用途
| 用途 | 種類 |
| --- | --- |
| 障害時の調査 | アプリケーションログ |
| ユーザーの利用状況を分析する | アクセスログ |
| システムの稼働を監視する | エラーログ |
| 不正アクセスの監視、調査 | セキュリティログ |
| セキュリティ基準を満たす | 監査ログ |

## logの見方
分析ツール, cloudwatch, ログフォルダなど

## logのフォーマット
フォーマット、内容、ログレベル

## rust コンパイラなど
- Rustツールチェイン(rustup)
  - rustc(コンパイラ)
  - cargo(ビルドマネージャ兼パッケージマネージャ)
  - std(標準ライブラリ)
- ターゲット環境向けのリンカ
  - rustやrust以外のコンパイル後のオブジェクトファイルやライブラリを結合してバイナリ（実行可能ファイル）を生成

## エラーメッセージのドキュメント表示
ex. error[E0308]: `` rustc --explain 308 ``

## 実行可能ファイルの生成
`` cargo build ``

## 特定のファイルの実行
example/benchmark.rs に、引数の26を与えて実行
`` cargo run --release --example benchmark -- 26 ``

## 自動導出できる標準ライブラリのトレイト
Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

## crate
dotenvは更新が止まっている、dotenvy推奨
注意すべきcrateが含まれていないかチェックするクレート... cargo-audit


