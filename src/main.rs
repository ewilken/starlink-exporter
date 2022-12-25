use prometheus::{Encoder, Registry, TextEncoder};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;
use warp::{
    http,
    hyper::{self, header::CONTENT_TYPE},
    Filter,
};

use crate::{error::Error, metrics::Metrics};
use starlink::proto::space_x::api::device::{
    device_client::DeviceClient,
    request,
    response,
    GetDeviceInfoRequest,
    Request,
};

mod error;
mod metrics;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let bind_address = dotenv::var("BIND_ADDRESS")
        .unwrap_or("0.0.0.0:9184".to_string())
        .parse::<SocketAddr>()
        .expect("parsing BIND_ADDRESS");
    let starlink_address = dotenv::var("STARLINK_ADDRESS").unwrap_or("http://192.168.100.1:9200".to_string());

    info!("connecting ro Starlink device on {}", &starlink_address);

    let mut client = DeviceClient::connect(starlink_address.clone()).await?;

    let req = tonic::Request::new(Request {
        request: Some(request::Request::GetDeviceInfo(GetDeviceInfoRequest {})),
        ..Default::default()
    });
    let res = client.handle(req).await?.into_inner();

    let mut labels = HashMap::new();

    if let Some(response::Response::GetDeviceInfo(r)) = res.response {
        if let Some(device_info) = r.device_info {
            if let Some(id) = device_info.id {
                info!("setting registry label id = {}", &id);
                labels.insert("id".to_string(), id);
            }
            if let Some(hardware_version) = device_info.hardware_version {
                info!("setting registry label hardware_version = {}", &hardware_version);
                labels.insert("hardware_version".to_string(), hardware_version);
            }
            // `software_version` & `country_code` are subject to change at runtime
            // if let Some(software_version) = device_info.software_version {
            //     info!("setting registry label software_version = {}", &software_version);
            //     labels.insert("software_version".to_string(), software_version);
            // }
            // if let Some(country_code) = device_info.country_code {
            //     info!("setting registry label country_code = {}", &country_code);
            //     labels.insert("country_code".to_string(), country_code);
            // }
        }
    }

    let registry = Registry::new_custom(Some("starlink".to_string()), Some(labels))?;

    let metrics = Metrics::new()?;
    metrics.register(&registry)?;
    let metrics = Arc::new(Mutex::new(metrics));

    let route = warp::get()
        .and(warp::path("metrics"))
        .and(warp::addr::remote())
        .and_then(move |addr: Option<SocketAddr>| {
            if let Some(addr) = addr {
                info!("incoming request from {}", addr);
            }

            let starlink_address = starlink_address.clone();
            let metrics = metrics.clone();
            let registry = registry.clone();

            async move {
                let mut metrics = metrics.lock().await;
                metrics.update(starlink_address).await?;

                let encoder = TextEncoder::new();

                let metric_families = registry.gather();
                let mut buffer = vec![];
                encoder
                    .encode(&metric_families, &mut buffer)
                    .map_err(|e| Error::from(e))?;

                let response = http::Response::builder()
                    .status(200)
                    .header(CONTENT_TYPE, encoder.format_type())
                    .body(hyper::Body::from(buffer))
                    .map_err(|e| Error::from(e))?;

                Ok(response) as Result<hyper::Response<hyper::Body>, warp::Rejection>
            }
        });

    info!("binding Prometheus exporter on http://{}", &bind_address);

    warp::serve(route).run(bind_address).await;

    Ok(())
}
