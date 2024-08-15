// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, _config: &Config) -> Flow<()> {
    let headers_state = request_state.into_headers_state().await;
    let header = _config.header_key.clone();
    logger::debug!("Request recevied");
    match headers_state.handler().header(&header) {
	Some(v) =>  logger::info!("Header value is {v}"),
	None => {
	    logger::info!("None") ;
	    return Flow::Break(Response::new(401).with_body("No required header ".to_owned() + &header));
	}
    };
    Flow::Continue(())
    // Log the header value
    
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
