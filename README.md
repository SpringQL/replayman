# replayman

Log agent to replay time-stamped log stream.

## Getting started

### Installation

```bash
cargo install replayman
```

### Prepare for your data

TSV with a header line is supported.

```bash
curl -O https://raw.githubusercontent.com/SpringQL/dataset/main/pseudo-in-vehicle/AirConditioner-30sec.tsv
```

### Usage

#### stdout

```bash
$ replayman \
  --timed-by Time \
  --initial-timestamp '2020-10-21T10:37:56.000+09:00' \
  AirConditioner-30sec.tsv
```

#### TCP socket

```bash
nc -l 19870
```

```bash
$ replayman \
  --timed-by Time \
  --initial-timestamp '2020-10-21T10:37:56.000+09:00' \
  --dest-tcp '127.0.0.1:19870' \
  AirConditioner-30sec.tsv
```

#### MQTT

Using [mosquitto](https://github.com/eclipse/mosquitto) here for example.

```bash
mosquitto_sub -h 127.0.0.1 -t replayman/test
```

```bash
$ replayman \
  --timed-by Time \
  --initial-timestamp '2020-10-21T10:37:56.000+09:00' \
  --dest-mqtt '127.0.0.1:1883' \
  --dest-mqtt-topic 'replayman/test' \
  AirConditioner-30sec.tsv
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in replayman by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Copyright (c) 2021-2022 TOYOTA MOTOR CORPORATION.
