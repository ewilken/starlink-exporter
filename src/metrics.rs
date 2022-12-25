use prometheus::{Counter, Gauge, GaugeVec, Opts, Registry};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::error::Error;
use starlink::proto::space_x::api::device::{
    device_client::DeviceClient,
    request,
    response,
    GetStatusRequest,
    Request,
};

#[derive(Debug)]
pub struct Metrics {
    pub device_info: GaugeVec,

    pub uptime_s: Counter,

    pub state: Gauge,

    pub alert_motors_stuck: Gauge,
    pub alert_thermal_throttle: Gauge,
    pub alert_thermal_shutdown: Gauge,
    pub alert_mast_not_near_vertical: Gauge,
    pub alert_unexpected_location: Gauge,
    pub alert_slow_ethernet_speeds: Gauge,

    pub snr: Gauge,
    pub seconds_to_first_nonempty_slot: Gauge,
    pub pop_ping_drop_rate: Gauge,
    pub downlink_throughput_bps: Gauge,
    pub uplink_throughput_bps: Gauge,
    pub pop_ping_latency_ms: Gauge,

    pub obstruction_currently_obstructed: Gauge,
    pub obstruction_fraction_obstructed: Gauge,
    pub obstruction_last_24h_obstructed_s: Counter,
    pub obstruction_valid_s: Counter,
    pub obstruction_wedge_fraction_obstructed: GaugeVec,
    pub obstruction_wedge_abs_fraction_obstructed: GaugeVec,
}

impl Metrics {
    pub fn new() -> Result<Self, Error> {
        let metrics = Metrics {
            device_info: GaugeVec::new(Opts::new("device_info", "Device information. Exposing `software_version` and `country_code` as additional labels.").namespace("dish"), &[
                // "id",
                // "hardware_version",
                "software_version",
                "country_code",
            ])?,

            uptime_s: Counter::with_opts(Opts::new("uptime_s", "Dish uptime in seconds.").namespace("dish"))?,

            state: Gauge::with_opts(
                Opts::new(
                    "state",
                    "Dish state. 0: Unknown, 1: Connected, 2: Searching, 3: Booting.",
                )
                .namespace("dish"),
            )?,

            alert_motors_stuck: Gauge::with_opts(
                Opts::new("motors_stuck", "Alert: Motors stuck.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,
            alert_thermal_throttle: Gauge::with_opts(
                Opts::new("thermal_throttle", "Alert: Thermal throttle.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,
            alert_thermal_shutdown: Gauge::with_opts(
                Opts::new("thermal_shutdown", "Alert: Thermal shutdown.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,
            alert_mast_not_near_vertical: Gauge::with_opts(
                Opts::new("mast_not_near_vertical", "Alert: Mast not near vertical.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,
            alert_unexpected_location: Gauge::with_opts(
                Opts::new("unexpected_location", "Alert: Unexpected location.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,
            alert_slow_ethernet_speeds: Gauge::with_opts(
                Opts::new("slow_ethernet_speeds", "Alert: Slow ethernet speeds.")
                    .namespace("dish")
                    .subsystem("alert"),
            )?,

            snr: Gauge::with_opts(Opts::new("snr", "Signal-to-noise ratio.").namespace("dish"))?,
            seconds_to_first_nonempty_slot: Gauge::with_opts(
                Opts::new("seconds_to_first_nonempty_slot", "Seconds to first non-empty slot.").namespace("dish"),
            )?,
            pop_ping_drop_rate: Gauge::with_opts(
                Opts::new("pop_ping_drop_rate", "Pop ping drop rate.").namespace("dish"),
            )?,
            downlink_throughput_bps: Gauge::with_opts(
                Opts::new("downlink_throughput_bps", "Downlink throughput in Bps.").namespace("dish"),
            )?,
            uplink_throughput_bps: Gauge::with_opts(
                Opts::new("uplink_throughput_bps", "Uplink throughput in Bps.").namespace("dish"),
            )?,
            pop_ping_latency_ms: Gauge::with_opts(
                Opts::new("pop_ping_latency_ms", "Pop ping latency in ms.").namespace("dish"),
            )?,

            obstruction_currently_obstructed: Gauge::with_opts(
                Opts::new("currently_obstructed", "Obstruction: Currently obstructed.")
                    .namespace("dish")
                    .subsystem("obstruction"),
            )?,
            obstruction_fraction_obstructed: Gauge::with_opts(
                Opts::new("fraction_obstructed", "Obstruction: Obstructed fraction. Sum of obstructed fractions.")
                    .namespace("dish")
                    .subsystem("obstruction"),
            )?,
            obstruction_last_24h_obstructed_s: Counter::with_opts(
                Opts::new("last_24h_obstructed_s", "Obstruction: Obstructed seconds in the last 24 hours.")
                    .namespace("dish")
                    .subsystem("obstruction"),
            )?,
            obstruction_valid_s: Counter::with_opts(
                Opts::new("valid_s", "Obstruction: Valid seconds.")
                    .namespace("dish")
                    .subsystem("obstruction"),
            )?,
            obstruction_wedge_fraction_obstructed: GaugeVec::new(
                Opts::new("wedge_fraction_obstructed", "Obstruction: Wedge fraction obstructed. Measure of obstruction in twelve 30 degree wedges around the dish.")
                    .namespace("dish")
                    .subsystem("obstruction"),
                &["wedge"],
            )?,
            obstruction_wedge_abs_fraction_obstructed: GaugeVec::new(
                Opts::new(
                    "wedge_abs_fraction_obstructed",
                    "Obstruction: Wedge fraction obstruction average. Measure of average obstruction in twelve 30 degree wedges around the dish.",
                )
                .namespace("dish")
                .subsystem("obstruction"),
                &["wedge"],
            )?,
        };

        Ok(metrics)
    }

    pub fn register(&self, registry: &Registry) -> Result<(), Error> {
        registry.register(Box::new(self.device_info.clone()))?;

        registry.register(Box::new(self.uptime_s.clone()))?;

        registry.register(Box::new(self.state.clone()))?;

        registry.register(Box::new(self.alert_motors_stuck.clone()))?;
        registry.register(Box::new(self.alert_thermal_throttle.clone()))?;
        registry.register(Box::new(self.alert_thermal_shutdown.clone()))?;
        registry.register(Box::new(self.alert_mast_not_near_vertical.clone()))?;
        registry.register(Box::new(self.alert_unexpected_location.clone()))?;
        registry.register(Box::new(self.alert_slow_ethernet_speeds.clone()))?;

        registry.register(Box::new(self.snr.clone()))?;
        registry.register(Box::new(self.seconds_to_first_nonempty_slot.clone()))?;
        registry.register(Box::new(self.pop_ping_drop_rate.clone()))?;
        registry.register(Box::new(self.downlink_throughput_bps.clone()))?;
        registry.register(Box::new(self.uplink_throughput_bps.clone()))?;
        registry.register(Box::new(self.pop_ping_latency_ms.clone()))?;

        registry.register(Box::new(self.obstruction_currently_obstructed.clone()))?;
        registry.register(Box::new(self.obstruction_fraction_obstructed.clone()))?;
        registry.register(Box::new(self.obstruction_last_24h_obstructed_s.clone()))?;
        registry.register(Box::new(self.obstruction_valid_s.clone()))?;
        registry.register(Box::new(self.obstruction_wedge_fraction_obstructed.clone()))?;
        registry.register(Box::new(self.obstruction_wedge_abs_fraction_obstructed.clone()))?;

        Ok(())
    }

    pub async fn update(&mut self, starlink_address: String) -> Result<(), Error> {
        let mut client = DeviceClient::connect(starlink_address).await?;

        info!("updating metrics from Starlink device");

        debug!("sending GetStatusRequest to Starlink device");
        let req = tonic::Request::new(Request {
            request: Some(request::Request::GetStatus(GetStatusRequest {})),
            ..Default::default()
        });
        let res = client.handle(req).await?;
        debug!("received gRPC response: {:#?}", &res);
        let get_status_res = res.into_inner();

        if let Some(response::Response::DishGetStatus(response)) = get_status_res.response {
            if let Some(device_info) = response.device_info {
                let mut labels = HashMap::new();

                // `id` & `hardware_version` are set on program start to all metrics on the register level

                // let id: String;
                // if let Some(d_id) = device_info.id {
                //     id = d_id;

                //     info!("id: {}", &id);

                //     labels.insert("id", id.as_str());
                // }

                // let hardware_version: String;
                // if let Some(d_hardware_version) = device_info.hardware_version {
                //     hardware_version = d_hardware_version;

                //     info!("hardware_version: {}", &hardware_version);

                //     labels.insert("hardware_version", hardware_version.as_str());
                // }

                let software_version: String;
                if let Some(d_software_version) = device_info.software_version {
                    software_version = d_software_version;

                    info!("software_version: {}", &software_version);

                    labels.insert("software_version", software_version.as_str());
                }

                let country_code: String;
                if let Some(d_country_code) = device_info.country_code {
                    country_code = d_country_code;

                    info!("country_code: {}", &country_code);

                    labels.insert("country_code", country_code.as_str());
                }

                self.device_info.get_metric_with(&labels)?.set(1_f64);
            }

            if let Some(device_state) = response.device_state {
                if let Some(uptime_s) = device_state.uptime_s {
                    info!("uptime_s: {}", &uptime_s);

                    let previous_uptime_s = self.uptime_s.get();
                    if previous_uptime_s < uptime_s as f64 {
                        self.uptime_s.inc_by(uptime_s as f64 - previous_uptime_s);
                    } else if previous_uptime_s > uptime_s as f64 {
                        self.uptime_s.reset();
                        self.uptime_s.inc_by(uptime_s as f64);
                    }
                }
            }

            if let Some(state) = response.state {
                info!("state: {}", &state);

                self.state.set(state as f64);
            }

            if let Some(alerts) = response.alerts {
                if let Some(motors_stuck) = alerts.motors_stuck {
                    info!("alert_motors_stuck: {}", &motors_stuck);

                    self.alert_motors_stuck.set(bool_to_f64(motors_stuck));
                }
                if let Some(thermal_throttle) = alerts.thermal_throttle {
                    info!("alert_thermal_throttle: {}", &thermal_throttle);

                    self.alert_thermal_throttle.set(bool_to_f64(thermal_throttle));
                }
                if let Some(thermal_shutdown) = alerts.thermal_shutdown {
                    info!("alert_thermal_shutdown: {}", &thermal_shutdown);

                    self.alert_thermal_shutdown.set(bool_to_f64(thermal_shutdown));
                }
                if let Some(mast_not_near_vertical) = alerts.mast_not_near_vertical {
                    info!("alert_mast_not_near_vertical: {}", &mast_not_near_vertical);

                    self.alert_mast_not_near_vertical
                        .set(bool_to_f64(mast_not_near_vertical));
                }
                if let Some(unexpected_location) = alerts.unexpected_location {
                    info!("alert_unexpected_location: {}", &unexpected_location);

                    self.alert_unexpected_location.set(bool_to_f64(unexpected_location));
                }
                if let Some(slow_ethernet_speeds) = alerts.slow_ethernet_speeds {
                    info!("alert_slow_ethernet_speeds: {}", &slow_ethernet_speeds);

                    self.alert_slow_ethernet_speeds.set(bool_to_f64(slow_ethernet_speeds));
                }
            }

            if let Some(snr) = response.snr {
                info!("snr: {}", &snr);

                self.snr.set(snr as f64);
            }

            if let Some(seconds_to_first_nonempty_slot) = response.seconds_to_first_nonempty_slot {
                info!("seconds_to_first_nonempty_slot: {}", &seconds_to_first_nonempty_slot);

                self.seconds_to_first_nonempty_slot
                    .set(seconds_to_first_nonempty_slot as f64);
            }

            if let Some(pop_ping_drop_rate) = response.pop_ping_drop_rate {
                info!("pop_ping_drop_rate: {}", &pop_ping_drop_rate);

                self.pop_ping_drop_rate.set(pop_ping_drop_rate as f64);
            }

            if let Some(downlink_throughput_bps) = response.downlink_throughput_bps {
                info!("downlink_throughput_bps: {}", &downlink_throughput_bps);

                self.downlink_throughput_bps.set(downlink_throughput_bps as f64);
            }

            if let Some(uplink_throughput_bps) = response.uplink_throughput_bps {
                info!("uplink_throughput_bps: {}", &uplink_throughput_bps);

                self.uplink_throughput_bps.set(uplink_throughput_bps as f64);
            }

            if let Some(pop_ping_latency_ms) = response.pop_ping_latency_ms {
                info!("pop_ping_latency_ms: {}", &pop_ping_latency_ms);

                self.pop_ping_latency_ms.set(pop_ping_latency_ms as f64);
            }

            if let Some(obstruction_stats) = response.obstruction_stats {
                if let Some(currently_obstructed) = obstruction_stats.currently_obstructed {
                    info!("obstruction_currently_obstructed: {}", &currently_obstructed);

                    self.obstruction_currently_obstructed
                        .set(bool_to_f64(currently_obstructed));
                }
                if let Some(fraction_obstructed) = obstruction_stats.fraction_obstructed {
                    info!("obstruction_fraction_obstructed: {}", &fraction_obstructed);

                    self.obstruction_fraction_obstructed.set(fraction_obstructed as f64);
                }
                if let Some(last_24h_obstructed_s) = obstruction_stats.last_24h_obstructed_s {
                    info!("obstruction_last_24h_obstructed_s: {}", &last_24h_obstructed_s);

                    let previous_obstruction_last_24h_obstructed_s = self.obstruction_last_24h_obstructed_s.get();
                    if previous_obstruction_last_24h_obstructed_s < last_24h_obstructed_s as f64 {
                        self.obstruction_last_24h_obstructed_s
                            .inc_by(last_24h_obstructed_s as f64 - previous_obstruction_last_24h_obstructed_s);
                    } else if previous_obstruction_last_24h_obstructed_s > last_24h_obstructed_s as f64 {
                        self.obstruction_last_24h_obstructed_s.reset();
                        self.obstruction_last_24h_obstructed_s
                            .inc_by(last_24h_obstructed_s as f64);
                    }
                }
                if let Some(valid_s) = obstruction_stats.valid_s {
                    info!("obstruction_valid_s: {}", &valid_s);

                    let previous_obstruction_valid_s = self.obstruction_valid_s.get();
                    if previous_obstruction_valid_s < valid_s as f64 {
                        self.obstruction_valid_s
                            .inc_by(valid_s as f64 - previous_obstruction_valid_s);
                    } else if previous_obstruction_valid_s > valid_s as f64 {
                        self.obstruction_valid_s.reset();
                        self.obstruction_valid_s.inc_by(valid_s as f64);
                    }
                }

                for (i, v) in obstruction_stats.wedge_fraction_obstructed.into_iter().enumerate() {
                    info!("obstruction_wedge_fraction_obstructed: wedge {}: {}", &i, &v);

                    let mut m = HashMap::new();
                    let i = format!("{}", i);
                    m.insert("wedge", i.as_str());

                    self.obstruction_wedge_fraction_obstructed
                        .get_metric_with(&m)?
                        .set(v as f64);
                }
                for (i, v) in obstruction_stats.wedge_abs_fraction_obstructed.into_iter().enumerate() {
                    info!("obstruction_wedge_abs_fraction_obstructed: wedge {}: {}", &i, &v);

                    let mut m = HashMap::new();
                    let i = format!("{}", i);
                    m.insert("wedge", i.as_str());

                    self.obstruction_wedge_abs_fraction_obstructed
                        .get_metric_with(&m)?
                        .set(v as f64);
                }
            }
        }

        info!("updated metrics from Starlink device");
        debug!("{:#?}", &self);

        Ok(())
    }
}

fn bool_to_f64(v: bool) -> f64 {
    match v {
        true => 1_f64,
        false => 0_f64,
    }
}
