use serde::{Deserialize, Serialize};
use super::Result;
use super::Error;
use serde_json::{Map, Result as JsonResult, Value as JsonValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AresOracleCheckResult {
    request_scheme: String,
    request_status: String,
    request_body: Option<String>,
    url_path: String,
    url_query: Option<String>,
}

impl AresOracleCheckResult {
    pub fn new(
        request_scheme: String,
        request_status: String,
        url_path: String,
        url_query: Option<String>,
        request_body: Option<String>,
    ) -> Self {
        Self {
            request_scheme,
            request_status,
            request_body,
            url_path,
            url_query
        }
    }

    pub fn get_request_scheme(&self) -> String {
        self.request_scheme.clone()
    }

    pub fn get_request_status(&self) -> String {
        self.request_status.clone()
    }

    pub fn get_url_path(&self) -> String {
        self.url_path.clone()
    }

    pub fn get_url_query(&self) -> Option<String> {
        self.url_query.clone()
    }

    pub fn get_request_body(&self) -> Option<String> {
        self.request_body.clone()
    }

    pub fn check_scheme(&self) -> Result<()> {
        if "http".to_string() == self.get_request_scheme() {
            return Ok(())
        }
        Err(Error::SchemeNotHttp)
    }

    pub fn check_status(&self) -> Result<()> {
        if "200 OK".to_string() == self.get_request_status() {
            return Ok(())
        }
        Err(Error::HttpRequestFailed)
    }

    pub fn check_body(&self) -> Result<()> {
        let body_opt = self.get_request_body();
        if body_opt.is_some() {
            let body = body_opt.unwrap();
            let json_obj : JsonValue = serde_json::from_str(body.as_str()).unwrap();
            if json_obj["code"].to_string() != "0" { return Err(Error::JsonParsingError(1)) }
            if json_obj["message"].as_str() != Some("OK") { return Err(Error::JsonParsingError(2)) }
            if json_obj["data"].as_object().unwrap().get("btcusdt").is_none() { return Err(Error::JsonParsingError(3)) }
            if json_obj["data"].as_object().unwrap().get("ethusdt").is_none() { return Err(Error::JsonParsingError(4)) }
            return Ok(());
        }
        Err(Error::HttpRequestFailed)
    }
}
