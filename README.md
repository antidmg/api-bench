# api-bench
This is a simple tool to help benchmark and load test [melnet](https://github.com/themeliolabs/melnet) endpoints.
It is a simple wrapper around [metered-rs](https://github.com/magnet/metered-rs) that exposes a CLI to pass in arguments.

## Usage
### Run a melnet server
First, you should have access to a melnet server's address (either remote or locally) in the form of `IP:host`.
To run the server locally, clone this [repo](gh repo clone themeliolabs/melnet) and `cargo run`. Note the IP and host in the console output.

### Run the load test
``` sh
api-bench 0.1.0
Command line arguments

USAGE:
    api-bench [OPTIONS] --host <HOST> --port <PORT>

OPTIONS:
    -b, --body <BODY>                [default: hello]
    -h, --host <HOST>                
        --help                       Print help information
    -i, --iterations <ITERATIONS>    [default: 1000]
    -n, --netname <NETNAME>          [default: gossip]
    -p, --port <PORT>                
    -t, --threads <THREADS>          [default: 10]
    -v, --verb <VERB>                [default: gossip]
    -V, --version                    Print version information
```

An example response will look like: 

``` sh
Benchmark done. Here are the metrics for that run:
hit_count{path = "metrics/send_melnet_request"} 2000
throughput_samples{path = "metrics/send_melnet_request"} 1
throughput_min{path = "metrics/send_melnet_request"} 1808
throughput_max{path = "metrics/send_melnet_request"} 1815
throughput_mean{path = "metrics/send_melnet_request"} 1812
throughput_stdev{path = "metrics/send_melnet_request"} 0
throughput{quantile = "0.9", path = "metrics/send_melnet_request"} 1815
throughput{quantile = "0.95", path = "metrics/send_melnet_request"} 1815
throughput{quantile = "0.99", path = "metrics/send_melnet_request"} 1815
throughput{quantile = "0.999", path = "metrics/send_melnet_request"} 1815
throughput{quantile = "0.9999", path = "metrics/send_melnet_request"} 1815
```
