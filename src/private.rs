//! Private APIを実装する。

pub mod active_orders;
pub mod assets;
pub mod cancel_bulk_order;
pub mod cancel_order;
pub mod cancel_orders;
pub mod change_order;
pub mod executions;
pub mod latest_executions;
pub mod margin;
pub mod open_positions;
pub mod order;
pub mod orders;
pub mod position_summary;

use crate::dto::{DEFAULT_COUNT, DEFAULT_PAGE};
use crate::error::Error;
use crate::execution_type::ExecutionType;
use crate::http_client::HttpClient;
use crate::private::active_orders::{request_active_orders, ActiveOrders};
use crate::private::assets::{request_assets, Assets};
use crate::private::cancel_bulk_order::{request_cancel_bulk_order, CancelBulkOrder};
use crate::private::cancel_order::{request_cancel_order, CancelOrder};
use crate::private::cancel_orders::{request_cancel_orders, CancelOrders};
use crate::private::change_order::{request_change_order, ChangeOrder};
use crate::private::executions::{
    request_executions_with_execution_id, request_executions_with_order_id, Executions,
};
use crate::private::latest_executions::{request_latest_executions, LatestExecutions};
use crate::private::margin::{request_margin, Margin};
use crate::private::open_positions::{request_open_positions, OpenPositions};
use crate::private::order::{request_order, Order};
use crate::private::orders::{request_orders, Orders};
use crate::private::position_summary::{request_position_summary, PositionSummary};
use crate::response::RestResponse;
use crate::settle_type::SettleType;
use crate::side::Side;
use crate::symbol::Symbol;
use crate::time_in_force::TimeInForce;

/// Private API。
pub struct PrivateAPI<T: HttpClient + std::marker::Sync + std::marker::Send> {
    pub http_client: T,
}

impl<T: HttpClient + std::marker::Sync + std::marker::Send> PrivateAPI<T> {
    /// 余力情報APIを呼び出す。
    ///
    /// # Arguments
    ///
    ///
    pub async fn margin(&self) -> Result<RestResponse<Margin>, Error> {
        let response = request_margin(&self.http_client).await?;
        Ok(response)
    }

    /// 資産残高APIを呼び出す。
    ///
    /// # Arguments
    ///
    ///
    pub async fn assets(&self) -> Result<RestResponse<Assets>, Error> {
        let response = request_assets(&self.http_client).await?;
        Ok(response)
    }

    /// 注文情報取得APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `order_ids` - 取得する注文の注文ID。最大10件まで指定できる。
    ///
    pub async fn orders(&self, order_ids: &[&str]) -> Result<RestResponse<Orders>, Error> {
        let response = request_orders(&self.http_client, &order_ids).await?;
        Ok(response)
    }

    /// 有効注文一覧APIを呼び出す。取得ページは1、取得件数は100(最大値)を指定したとする。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    ///
    pub async fn active_orders(
        &self,
        symbol: &Symbol,
    ) -> Result<RestResponse<ActiveOrders>, Error> {
        let response =
            request_active_orders(&self.http_client, &symbol, DEFAULT_PAGE, DEFAULT_COUNT).await?;
        Ok(response)
    }

    /// 有効注文一覧APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `page` - 取得対象ページ。
    /// * `count` - 1ページ当たりの取得件数。
    ///
    pub async fn active_orders_with_options(
        &self,
        symbol: &Symbol,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<ActiveOrders>, Error> {
        let response = request_active_orders(&self.http_client, &symbol, page, count).await?;
        Ok(response)
    }

    /// 約定情報取得APIを呼び出す。指定した注文IDの約定情報が取得できる。
    ///
    /// # Arguments
    ///
    /// * `order_id` - 注文ID。
    ///
    pub async fn executions_with_order_id(
        &self,
        order_id: &str,
    ) -> Result<RestResponse<Executions>, Error> {
        let response = request_executions_with_order_id(&self.http_client, &order_id).await?;
        Ok(response)
    }

    /// 約定情報取得APIを呼び出す。指定した約定IDの約定情報が取得できる。
    ///
    /// # Arguments
    ///
    /// * `execution_id` - 約定ID。
    ///
    pub async fn executions_with_execution_id(
        &self,
        execution_id: &str,
    ) -> Result<RestResponse<Executions>, Error> {
        let response =
            request_executions_with_execution_id(&self.http_client, &execution_id).await?;
        Ok(response)
    }

    /// 最新の約定一覧APIを呼び出す。取得ページは1、取得件数は100(最大値)を指定したとする。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    ///
    pub async fn latest_executions(
        &self,
        symbol: &Symbol,
    ) -> Result<RestResponse<LatestExecutions>, Error> {
        let response =
            request_latest_executions(&self.http_client, &symbol, DEFAULT_PAGE, DEFAULT_COUNT)
                .await?;
        Ok(response)
    }

    /// 最新の約定一覧APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `page` - 取得対象ページ。
    /// * `count` - 1ページ当たりの取得件数。
    ///
    pub async fn latest_executions_with_options(
        &self,
        symbol: &Symbol,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<LatestExecutions>, Error> {
        let response = request_latest_executions(&self.http_client, &symbol, page, count).await?;
        Ok(response)
    }

    /// 建玉一覧APIを呼び出す。取得ページは1、取得件数は100(最大値)を指定したとする。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `page` - 取得対象ページ。
    /// * `count` - 1ページ当たりの取得件数。
    ///
    pub async fn open_positions(
        &self,
        symbol: &Symbol,
    ) -> Result<RestResponse<OpenPositions>, Error> {
        let response =
            request_open_positions(&self.http_client, &symbol, DEFAULT_PAGE, DEFAULT_COUNT).await?;
        Ok(response)
    }

    /// 建玉一覧APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `page` - 取得対象ページ。
    /// * `count` - 1ページ当たりの取得件数。
    ///
    pub async fn open_positions_with_options(
        &self,
        symbol: &Symbol,
        page: i32,
        count: i32,
    ) -> Result<RestResponse<OpenPositions>, Error> {
        let response = request_open_positions(&self.http_client, &symbol, page, count).await?;
        Ok(response)
    }

    /// 建玉サマリーAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    ///
    pub async fn position_summary(
        &self,
        symbol: &Symbol,
    ) -> Result<RestResponse<PositionSummary>, Error> {
        let response = request_position_summary(&self.http_client, &symbol).await?;
        Ok(response)
    }

    /// 新規注文APIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `execution_type` - 注文方法。
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。Marketの場合は不要。
    ///
    pub async fn order(
        &self,
        execution_type: &ExecutionType,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: Option<i64>,
    ) -> Result<RestResponse<Order>, Error> {
        let time_in_force = match execution_type {
            ExecutionType::Limit => TimeInForce::Fas,
            _ => TimeInForce::Fak,
        };
        let response = request_order(
            &self.http_client,
            &execution_type,
            &symbol,
            &side,
            size,
            &time_in_force,
            price,
            None,
        )
        .await?;
        Ok(response)
    }

    /// 新規注文APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `execution_type` - 注文方法。
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。Marketの場合は不要。
    /// * `time_in_force` - 執行数量条件。
    /// * `losscut_price` - ロスカットレート。
    ///
    pub async fn order_with_options(
        &self,
        execution_type: &ExecutionType,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: Option<i64>,
        time_in_force: &TimeInForce,
        losscut_price: Option<i64>,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &execution_type,
            &symbol,
            &side,
            size,
            &time_in_force,
            price,
            losscut_price,
        )
        .await?;
        Ok(response)
    }

    /// 注文変更APIを呼び出す。ロスカットレートは指定しない。
    ///
    /// # Arguments
    ///
    /// * `order_id` - 変更する注文の注文ID。
    /// * `price` - 注文価格。
    ///
    pub async fn change_order(
        &self,
        order_id: &str,
        price: i64,
    ) -> Result<RestResponse<ChangeOrder>, Error> {
        let response = request_change_order(&self.http_client, &order_id, price, None).await?;
        Ok(response)
    }

    /// 注文変更APIをオプション引数つきで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `order_id` - 変更する注文の注文ID。
    /// * `price` - 注文価格。
    /// * `losscut_price` - ロスカットレート。
    ///
    pub async fn change_order_with_options(
        &self,
        order_id: &str,
        price: i64,
        losscut_price: i64,
    ) -> Result<RestResponse<ChangeOrder>, Error> {
        let response =
            request_change_order(&self.http_client, &order_id, price, Some(losscut_price)).await?;
        Ok(response)
    }

    /// 注文キャンセルAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `order_id` - 注文ID。
    ///
    pub async fn cancel_order(&self, order_id: &str) -> Result<RestResponse<CancelOrder>, Error> {
        let response = request_cancel_order(&self.http_client, &order_id).await?;
        Ok(response)
    }

    /// 複数の注文キャンセルAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `order_ids` - 注文IDの配列。
    ///
    pub async fn cancel_orders(
        &self,
        order_ids: &[&str],
    ) -> Result<RestResponse<CancelOrders>, Error> {
        let response = request_cancel_orders(&self.http_client, &order_ids).await?;
        Ok(response)
    }

    /// 注文の一括キャンセルAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbols` - 銘柄の配列。
    ///
    pub async fn cancel_bulk_order(
        &self,
        symbols: &[&Symbol],
    ) -> Result<RestResponse<CancelBulkOrder>, Error> {
        let response =
            request_cancel_bulk_order(&self.http_client, &symbols, None, None, false).await?;
        Ok(response)
    }

    /// 注文の一括キャンセルAPIを呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbols` - 銘柄の配列。
    /// * `side` - 指定時、指定された売買区分の注文を取り消し対象にする。
    /// * `settle_type` - 指定時、現物取引注文と指定された決済区分のレバレッジ取引注文を取消対象にする。
    /// * `desc` - trueの場合注文日時が新しい注文から取り消しする。falseの場合古い注文から取り消しする。指定されない場合falseとする。
    ///
    pub async fn cancel_bulk_order_with_options(
        &self,
        symbols: &[&Symbol],
        side: Option<&Side>,
        settle_type: Option<&SettleType>,
        desc: Option<bool>,
    ) -> Result<RestResponse<CancelBulkOrder>, Error> {
        let response = match desc {
            Some(d) => {
                request_cancel_bulk_order(&self.http_client, &symbols, side, settle_type, d).await?
            }
            None => {
                request_cancel_bulk_order(&self.http_client, &symbols, side, settle_type, false)
                    .await?
            }
        };
        Ok(response)
    }
}
