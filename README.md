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

### 注文 Id, 約定 Id, 建玉 Id のデータ型

これらのデータ型 は文字列とします。

### プロパティの命名規則

GMO コイン API ではプロパティはキャメルケース(例: `orderId`)で命名されますが、
このライブラリでは Rust の慣習に従いスネークケース(例: `order_id`)で命名を行います。

### レスポンスにプロパティが含まれない場合のデフォルト値

注文情報取得 API では通常次の形式のレスポンスが返ってきます。

```json
{
  "status": 0,
  "data": {
    "list": [
      {
        "orderId": 223456789,
        "rootOrderId": 223456789,
        "symbol": "BTC_JPY",
        "side": "BUY",
        "orderType": "NORMAL",
        "executionType": "LIMIT",
        "settleType": "OPEN",
        "size": "0.02",
        "executedSize": "0.02",
        "price": "1430001",
        "losscutPrice": "0",
        "status": "EXECUTED",
        "timeInForce": "FAS",
        "timestamp": "2020-10-14T20:18:59.343Z"
      }
    ]
  },
  "responsetime": "2019-03-19T02:15:06.059Z"
}
```

しかし、注文 ID が不適である場合には次の形式のレスポンスが返ってきます。

```json
{
  "status": 0,
  "data": {},
  "responsetime": "2019-03-19T02:15:06.059Z"
}
```

それぞれのレスポンスを見比べると、`list`プロパティが含まれている/含まれていないという違いがあることがわかります。

このように条件によってはレスポンスにプロパティが含まれない場合があります。
このライブラリでは、プロパティが含まれない場合はデフォルト値を持たせるようにしています。
各プロパティに対するデフォルト値を次に記載します。

#### 配列型のプロパティ

要素 0 の配列をデフォルト値とします。

#### ページ情報 (pagination)

取得ページ数 0、取得件数 0 をデフォルト値とします。

#### 取り消し区分

`NONE`をデフォルト値とします。

### エラーレスポンスが返ってきた場合

何らかの原因でエラーレスポンスが返ってくることがあります。
エラーレスポンスの形式は次の通りです。

```json
{
  "status": ステータスコード,
  "messages": {
    "message_code": エラーコード,
    "message_string": エラー内容
  }
}
```

エラーが返ってきた場合には各 API の返り値`Result<T, E>`の`E`側で`error::Error::APIError`という値を返します。
