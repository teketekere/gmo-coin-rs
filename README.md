# gmo-coin-rs

Rust 製の GMOCoin API のクライアントライブラリです。

## 実装状況

- Public API

  した

- Public Websocket API
- Private API
- Private Websocket API

  まだ

## 使い方

`examples`に大体書いてあります。

## 注意点

- 注文 Id

注文 Id は文字列で扱います。

- 特定の条件下でのみ返ってくるプロパティ

例えば、注文情報取得 API では取り消し区分(`cancelType`)プロパティは注文ステータス(`status`)が`CANCELLING`, `CANCELED`, `EXPIRED`の場合のみ
返されます。

このライブラリではプロパティがない場合は`NONE`という値を文字列で持つようにしています。
