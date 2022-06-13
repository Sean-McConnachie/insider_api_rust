use database_handler::models::stock_data::StockData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StockDataVec {
    pub defaults: Vec<StockData>,
}