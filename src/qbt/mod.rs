pub mod sync;
pub mod torrents;
pub mod transfer;

use http::header::COOKIE;
use serde_json::Value;

use self::sync::{MainData, SyncMainDataFull, SyncMainDataPartial};
use self::torrents::TorrentSummary;

pub static BASE_QBT_URL: &str = "http://localhost:9090/api/v2";
pub static TORRENTS_API: &str = "/torrents";
pub static INFO_API: &str = "/info";
pub static SYNC_API: &str = "/sync";
pub static MAINDATA_API: &str = "/maindata";

pub async fn get_torrents_info(sid: String) -> Result<Vec<TorrentSummary>, reqwest::Error> {
    let url = format!("{}{}{}", BASE_QBT_URL, TORRENTS_API, INFO_API);
    let client = reqwest::Client::builder().build()?;

    // let cookie: Cookie = Cookie::build(("SID", sid)).build();

    // Make an initial request to set some cookies
    let response = client
        .get(url)
        .header(COOKIE, format!("SID={}", sid).to_string())
        .send()
        .await?;

    response.json().await
}

pub async fn get_sync_maindata(sid: String, rid: u64) -> Result<MainData, reqwest::Error> {
    let url = format!("{}{}{}?rid={}", BASE_QBT_URL, SYNC_API, MAINDATA_API, rid);
    let client = reqwest::Client::builder().build()?;

    // let cookie: Cookie = Cookie::build(("SID", sid)).build();

    // Make an initial request to set some cookies
    let response = client
        .get(url)
        .header(COOKIE, format!("SID={}", sid).to_string())
        .send()
        .await?;

    let data = response.json::<Value>().await?;
    let is_full_update = match data.get("full_update") {
        Some(full_update) => serde_json::from_value(full_update.to_owned()).unwrap(),
        None => false,
    };

    if is_full_update == true {
        let data: SyncMainDataFull = serde_json::from_value(data).unwrap();
        Ok(MainData::Full(data))
    } else {
        let data: SyncMainDataPartial = serde_json::from_value(data).unwrap();
        Ok(MainData::Partial(data))
    }
}
