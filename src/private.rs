//! Private APIを実装する。

pub mod active_orders;
pub mod assets;
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

    /// 新規成行注文APIを呼び出す。執行数量条件はFAK。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    ///
    pub async fn market_order(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Market,
            &symbol,
            &side,
            size,
            &TimeInForce::Fak,
            None,
            None,
        )
        .await?;
        Ok(response)
    }

    /// 新規成行注文APIをオプション引数付きで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `time_in_force` - 執行数量条件。
    ///
    pub async fn market_order_with_options(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        time_in_force: &TimeInForce,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Market,
            &symbol,
            &side,
            size,
            &time_in_force,
            None,
            None,
        )
        .await?;
        Ok(response)
    }

    /// 新規指値注文APIを呼び出す。執行数量条件はFAS、ロスカットレートは指定なし。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。
    ///
    pub async fn limit_order(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: i64,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Limit,
            &symbol,
            &side,
            size,
            &TimeInForce::Fas,
            Some(price),
            None,
        )
        .await?;
        Ok(response)
    }

    /// 新規指値注文APIをオプション引数つきで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。
    /// * `time_in_force` - 執行数量条件。
    /// * `losscut_price` - ロスカットレート。
    ///
    pub async fn limit_order_with_options(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: i64,
        time_in_force: &TimeInForce,
        losscut_price: i64,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Limit,
            &symbol,
            &side,
            size,
            &time_in_force,
            Some(price),
            Some(losscut_price),
        )
        .await?;
        Ok(response)
    }

    /// 新規逆指値注文APIを呼び出す。執行数量条件はFAK、ロスカットレートは指定なし。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。
    ///
    pub async fn stop_order(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: i64,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Stop,
            &symbol,
            &side,
            size,
            &TimeInForce::Fak,
            Some(price),
            None,
        )
        .await?;
        Ok(response)
    }

    /// 新規逆指値注文APIをオプション引数つきで呼び出す。
    ///
    /// # Arguments
    ///
    /// * `symbol` - 銘柄。
    /// * `side` - 売買区分。
    /// * `size` - 注文数量。
    /// * `price` - 注文価格。
    /// * `time_in_force` - 執行数量条件。
    /// * `losscut_price` - ロスカットレート。
    ///
    pub async fn stop_order_with_options(
        &self,
        symbol: &Symbol,
        side: &Side,
        size: f64,
        price: i64,
        time_in_force: &TimeInForce,
        losscut_price: i64,
    ) -> Result<RestResponse<Order>, Error> {
        let response = request_order(
            &self.http_client,
            &ExecutionType::Stop,
            &symbol,
            &side,
            size,
            &time_in_force,
            Some(price),
            Some(losscut_price),
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
}
