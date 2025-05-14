use super::codes_dtos::{ConditionCodesRequestDTO, ConditionCodesResponseDTO, ExchangeCodesRequestDTO, ExchangeCodesResponseDTO};

#[bon::builder]
pub async fn condition_codes_request_builder(client: reqwest::Client, ticktype: String, tape: String) -> Result<ConditionCodesResponseDTO, reqwest::Error> {
	condition_codes_request(client, ConditionCodesRequestDTO::builder().ticktype(ticktype).tape(tape).build()).await
}

pub async fn condition_codes_request(client: reqwest::Client, request: ConditionCodesRequestDTO) -> Result<ConditionCodesResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/condition-codes".to_string()).query(&request).send().await?.json().await
}

pub async fn exchange_codes_request(client: reqwest::Client, request: ExchangeCodesRequestDTO) -> Result<ExchangeCodesResponseDTO, reqwest::Error> {
	client.get("https://data.alpaca.markets/v2/stocks/exchange-codes".to_string()).query(&request).send().await?.json().await
}
