# kafgrind

This is benchmarking tool for kafka to run producers and consumers as separate threads to measure kafka latency and throughput.

User has ability to set the size of payload and number of records that need to be produced. See Binary Usage section to see examples and options.

## Installation

One way of using this binary is to install rust and cargo using rustup and then use cargo to install kafgrind binary.

## Rust up

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Install using Cargo

`cargo install kafgrind`

## Binary Usages

### Available options

`kafgrind --help` or `kafgrind -h`
```text
kafgrind -h
Simple to use Kafka echo benchmarking tool

Usage: kafgrind [OPTIONS] --rate <RATE> --size-in-bytes <SIZE_IN_BYTES> --number-of-records <NUMBER_OF_RECORDS> --brokers <BROKERS> --topic <TOPIC>

Options:
-r, --rate <RATE>                            target throughput rate
-s, --size-in-bytes <SIZE_IN_BYTES>          payload size in bytes
-n, --number-of-records <NUMBER_OF_RECORDS>  payload size in bytes
-b, --brokers <BROKERS>                      brokers urls separated by `,`
-t, --topic <TOPIC>                          brokers urls separated by `,`
-u, --user <USER>                            kafka user
-p, --password <PASSWORD>                    kafka password
-h, --help                                   Print help (see more with '--help')
-V, --version                                Print version
```


### Examples

example without authentication

`kafgrind -r 100 -s 1024 -n 1000 -b localhost:9092 -t test.topic`

example with authentication

`kafgrind -r 100 -s 1024 -n 1000 -b localhost:9092 -t test.topic -u kafka_user -p kafka_password`

Note: In case of kafka authentication we set the defaults for `security.protocol` to `SASL_PLAINTEXT`
and `sasl.mechanisms` to  `SCRAM-SHA-512`.


## Output

Here is the sample output which supports md file support to render table.


| P0 | P50 | P90 | P95 | P98 | P99 | P99.9 |messageCount|Throughput|
|----|-----|-----|-----|-----|-----|-------|------------|----------|
| 5 | 7 | 10 | 11 | 13 | 14 | 18 | 1000 | 100 |

