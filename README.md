# starlink-exporter

[![CI](https://github.com/ewilken/starlink-exporter/workflows/CI/badge.svg)](https://github.com/ewilken/starlink-exporter/actions?query=workflow%3ACI)
[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ewilken/starlink-exporter)

Prometheus exporter for the metrics exposed by the gRPC endpoint of the SpaceX Starlink user terminal.

Based on [`starlink-rs`](https://github.com/ewilken/starlink-rs).

## Metrics

Currently, the following metrics are exposed:

- `starlink_dish_alerts_motors_stuck`
- `starlink_dish_alerts_thermal_shutdown`
- `starlink_dish_alerts_thermal_throttle`
- `starlink_dish_downlink_throughput_bps`
- `starlink_dish_obstruction_stats_currently_obstructed`
- `starlink_dish_obstruction_stats_fraction_obstructed`
- `starlink_dish_obstruction_stats_last_24h_obstructed_s`
- `starlink_dish_obstruction_stats_valid_s`
- `starlink_dish_obstruction_stats_wedge_abs_fraction_obstructed`
- `starlink_dish_obstruction_stats_wedge_fraction_obstructed`
- `starlink_dish_pop_ping_drop_rate`
- `starlink_dish_pop_ping_latency_ms`
- `starlink_dish_seconds_to_first_nonempty_slot`
- `starlink_dish_snr`
- `starlink_dish_state`
- `starlink_dish_uplink_throughput_bps`
- `starlink_dish_uptime_s`

## Usage

Configuration happens via the following env vars:

- `BIND_ADDRESS`: Host and port to bind the HTTP server to. Defaults to `0.0.0.0:9184`.
- `STARLINK_ADDRESS`: Protocol, host and port of the Starlink dish. Defaults to `http://192.168.100.1:9200`.

### Local

    cargo run --release

### Docker

    # build yourself
    docker build -t ewilken/starlink-exporter .
    docker run --net=host ewilken/starlink-exporter

    # or pull from GHCR
    docker run --net=host ghcr.io/ewilken/starlink-exporter:latest

    # or cross compile with Docker Buildx
    docker buildx build --platform=linux/amd64,linux/arm64 -t ewilken/starlink-exporter .

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: starlink-exporter
  labels:
    k8s-app: starlink-exporter
spec:
  replicas: 1
  selector:
    matchLabels:
      k8s-app: starlink-exporter
  strategy:
    type: Recreate
  template:
    metadata:
      name: starlink-exporter
      labels:
        k8s-app: starlink-exporter
      annotations:
        prometheus.io/port: '9184'
        prometheus.io/scrape: 'true'
    spec:
      containers:
        - image: ghcr.io/ewilken/starlink-exporter:latest
          name: starlink-exporter
          ports:
            - containerPort: 9184
              protocol: TCP
          resources:
            requests:
              cpu: '0.001'
              memory: '10Mi'
            limits:
              cpu: '0.01'
              memory: '50Mi'
          env:
            - name: RUST_LOG
              value: starlink_exporter=info
            - name: BIND_ADDRESS
              value: '0.0.0.0:9184'
            - name: STARLINK_ADDRESS
              value: 'http://192.168.100.1:9200'
      hostNetwork: true
      dnsPolicy: ClusterFirstWithHostNet
```

## CI & Versioning

[Images hosted on GHCR](https://github.com/users/ewilken/packages/container/package/starlink-exporter) are built in CI from main tagged `latest` and with the short commit hash returned by `$(git log -1 --format=%h)`, e.g. `1354d30`.

## License

`starlink-exporter` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
