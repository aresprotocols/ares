use std::collections::HashMap;
use std::pin::Pin;
use jsonrpc_derive::rpc;
use jsonrpc_core as rpc;
use serde::{Deserialize, Serialize};
use sp_runtime::{
    offchain::{http, Duration},
};
use chrono::prelude::*;
use sc_service::{Role};

use parking_lot::RwLock;
/// Re-export the API for backward compatibility.
pub use sc_rpc_api::offchain::*;
use sc_rpc_api::{DenyUnsafe, UnsafeRpcError};
use sp_core::{
    offchain::{OffchainStorage, StorageKind},
    Bytes,
};
use std::sync::Arc;
use sp_runtime::offchain::Timestamp;

// pub const LOCAL_STORAGE_PRICE_REQUEST_DOMAIN: &[u8] = ;
// pub const LOCAL_HOST_KEY: &[u8] = b"are-ocw::local_host_key";

use ares_oracle_provider_support::{LOCAL_STORAGE_PRICE_REQUEST_DOMAIN, LOCAL_HOST_KEY};

#[cfg(test)]
mod tests;

mod request_check;

use request_check::AresOracleCheckResult;

/// Offchain RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Offchain RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Unavailable storage kind error.
    #[error("This storage kind is not available yet.")]
    UnavailableStorageKind,
    /// Call to an unsafe RPC was denied.
    #[error(transparent)]
    UnsafeRpcCalled(#[from] UnsafeRpcError),
    /// Call to request error.
    #[error("Attempt to request with `warehouse` request failed.")]
    HttpRequestFailed,
    /// Need http scheme
    #[error("Ares-Oracle only supports http requests, not https or other.")]
    SchemeNotHttp,
    /// Json parsing or format error
    #[error("Json parsing or format error.")]
    JsonParsingError(u8),

}

/// Base error code for all offchain errors.
const BASE_ERROR: i64 = 5000;

impl From<Error> for rpc::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::UnavailableStorageKind => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 1),
                message: "This storage kind is not available yet".into(),
                data: None,
            },
            Error::HttpRequestFailed => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 2),
                message: "Attempt to request a `Token` through `warehouse` request failed".into(),
                data: None,
            },
            Error::SchemeNotHttp => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 4),
                message: "Ares-Oracle only supports http requests, not https or other.".into(),
                data: None,
            },
            Error::JsonParsingError(x) => rpc::Error {
                code: rpc::ErrorCode::ServerError(BASE_ERROR + 5),
                message: format!("Json parsing or format error. (#{})", x).into(),
                data: None,
            },
            Error::UnsafeRpcCalled(e) => e.into(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeRole {
    /// Regular full node.
    Full,
    /// Regular light node.
    Light,
    /// Actual authority.
    Authority,
}

impl From<Role> for NodeRole {
    fn from(e: Role) -> Self {
        match e {
            Role::Full => { NodeRole::Full }
            Role::Light => { NodeRole::Light }
            Role::Authority => { NodeRole::Authority }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AresOracleInfos {
    warehouse: Option<String>,
    xray: Option<Bytes>,
    request_scheme_checked: String,
    request_status_checked: String,
    request_body_checked: String,
    node_role: NodeRole,
}

impl AresOracleInfos {
    pub fn new(
        warehouse: Option<String>,
        xray: Option<Bytes>,
        request_scheme_checked: String,
        request_status_checked: String,
        request_body_checked: String,
        node_role: Role,
    ) -> Self {
        Self {
            warehouse,
            xray,
            request_scheme_checked,
            request_status_checked,
            request_body_checked,
            node_role: node_role.into(),
        }
    }
}

#[rpc]
pub trait AresToolsApi {

    /// Set offchain local storage under given key and prefix.
    #[rpc(name = "ares_localStorageSet")]
    fn set_local_storage(&self, kind: StorageKind, key: Bytes, value: Bytes) -> Result<()>;

    /// Get offchain local storage under given key and prefix.
    #[rpc(name = "ares_localStorageGet")]
    fn get_local_storage(&self, kind: StorageKind, key: Bytes) -> Result<Option<Bytes>>;

    /// Get Warehouse configuration data
    #[rpc(name = "ares_getWarehouse")]
    fn get_warehouse(&self) -> Result<Option<String>>;

    /// Set Warehouse configuration data.
    #[rpc(name = "ares_setWarehouse")]
    fn set_warehouse(&self, value: String) -> Result<()>;

    /// Ares getXray
    #[rpc(name = "ares_getXray")]
    fn get_xray(&self) -> Result<Option<Bytes>>;

    /// Ares getInfos
    #[rpc(name = "ares_getInfos")]
    fn get_infos(&self) -> Result<AresOracleInfos>;

    /// Ares TestRequest
    #[rpc(name = "ares_tryRequest")]
    fn try_get_request(&self) -> Result<AresOracleCheckResult>;
}

#[derive(Debug)]
pub struct AresToolsStruct<T: OffchainStorage> {
    /// Offchain storage
    storage: Arc<RwLock<T>>,
    deny_unsafe: DenyUnsafe,
    role: Role,
}

impl<T: OffchainStorage> AresToolsStruct<T> {
    /// Create new instance of Offchain API.
    pub fn new(storage: T, deny_unsafe: DenyUnsafe, role: Role) -> Self {
        AresToolsStruct {
            storage: Arc::new(RwLock::new(storage)),
            deny_unsafe,
            role
        }
    }
}

impl<T: OffchainStorage + 'static> AresToolsApi for AresToolsStruct<T> {

    // type Output = Pin<Box<dyn Future<Output = io::Result<()>>>>;

    /// Set offchain local storage under given key and prefix.
    fn set_local_storage(&self, kind: StorageKind, key: Bytes, value: Bytes) -> Result<()> {
        self.deny_unsafe.check_if_safe()?;

        let prefix = match kind {
            StorageKind::PERSISTENT => sp_offchain::STORAGE_PREFIX,
            StorageKind::LOCAL => return Result::Err(Error::UnavailableStorageKind),
        };
        self.storage.write().set(prefix, &*key, &*value);
        Ok(())
    }

    /// Get offchain local storage under given key and prefix.
    fn get_local_storage(&self, kind: StorageKind, key: Bytes) -> Result<Option<Bytes>> {
        self.deny_unsafe.check_if_safe()?;

        let prefix = match kind {
            StorageKind::PERSISTENT => sp_offchain::STORAGE_PREFIX,
            StorageKind::LOCAL => return Result::Err(Error::UnavailableStorageKind),
        };
        Ok(self.storage.read().get(prefix, &*key).map(Into::into))
    }

    fn get_warehouse(&self) -> Result<Option<String>> {
        self.deny_unsafe.check_if_safe()?;

        let result = self.get_local_storage(StorageKind::PERSISTENT, Bytes(LOCAL_STORAGE_PRICE_REQUEST_DOMAIN.to_vec()));
        if let Ok(x) = result {
            if let Some(b) = x {
                let result_str = String::from_utf8(b.to_vec());
                return Ok(result_str.ok());
            }
        }
        Ok(None)
    }

    fn set_warehouse(&self, value: String) -> Result<()> {
        self.set_local_storage(StorageKind::PERSISTENT, Bytes(LOCAL_STORAGE_PRICE_REQUEST_DOMAIN.to_vec()), Bytes(value.as_bytes().to_vec()))
    }

    fn get_xray(&self) -> Result<Option<Bytes>> {
        self.get_local_storage(StorageKind::PERSISTENT, Bytes(LOCAL_HOST_KEY.to_vec()))
    }

   fn get_infos(&self) -> Result<AresOracleInfos> {
       self.deny_unsafe.check_if_safe()?;

       let check_res = self.try_get_request();
       let mut request_scheme_checked = "Failed".to_string();
       let mut request_status_checked = "Failed".to_string();
       let mut request_body_checked = "Failed".to_string();

       if let Ok(res) = check_res {
           request_scheme_checked = res.check_scheme().map(|x|"Ok".to_string()).unwrap_or("Failed".to_string());
           request_status_checked = res.check_status().map(|x|"Ok".to_string()).unwrap_or("Failed".to_string());
           let check_body_res = res.check_body();
           if(check_body_res.is_ok()){
               request_body_checked = "Ok".to_string();
           }else{
               let body_err = check_body_res.err();
               request_body_checked = format!("Failed({:?})", rpc::Error::from(body_err.unwrap()).message).to_string()
           }
       }

        Ok(
            AresOracleInfos::new(
                self.get_warehouse().unwrap(),
                self.get_xray().unwrap(),
                request_scheme_checked,
                request_status_checked,
                request_body_checked,
                self.role.clone(),
            )
        )
    }

    fn try_get_request(&self) -> Result<AresOracleCheckResult> {
        self.deny_unsafe.check_if_safe()?;
        let mut request_url = self.get_warehouse()?.unwrap_or("http://".to_string());
        let request_path = "/api/getBulkCurrencyPrices?currency=usdt&symbol=btc_eth";
        request_url.push_str(request_path);

        let resp = reqwest::blocking::get(request_url);

        if let Ok(res) = resp {
            return Ok(AresOracleCheckResult::new(
                res.url().scheme().to_string(),
                res.status().to_string(),
                res.url().path().to_string(),
                res.url().query().map(|x|x.to_string()),
                res.text().ok(),
            ))
        }
        return Err(Error::HttpRequestFailed);
    }
}
