use super::codes_dtos::{ConditionCodesRequestDTO, ConditionCodesResponseDTO, ExchangeCodesRequestDTO, ExchangeCodesResponseDTO};

pub async fn condition_codes_request(client: reqwest::Client, request: ConditionCodesRequestDTO) -> Result<ConditionCodesResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/condition-codes".to_string()).query(&request).send().await?.json().await
}

pub async fn exchange_codes_request(client: reqwest::Client, request: ExchangeCodesRequestDTO) -> Result<ExchangeCodesResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/exchange-codes".to_string()).query(&request).send().await?.json().await
}
