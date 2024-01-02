# rust-bot
オウム返しbot
<br>

## 参考
- 実践Rust入門
- ZliTechbookV3 第4章 LINEのおうむ返しbotをRustで作ってみよう
など

## TODO よりよく
- 命名規則の統一
- コメント見直す
- 変数名見直す
- ライブラリのエラーを知ってきちんと分類する、エラーハンドリングのよりよい方法を知る
- アーキテクチャの勉強、依存方向の確認



## 立ち上げ方
```
cargo run
```  
```
ngrok http 8080
``` 

forwardingのurlをコピーし、``/webhook``を付け加え、LINE developersのwebhook urlに貼り付け  \
localhost:8080で確認可能

## 雑メモ
[雑メモリンク](memo.md)