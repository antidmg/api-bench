use api_bench::{load_test, LoadTester};
use clap::Parser;
use std::{collections::HashMap, sync::Arc};
pub use std::{fmt, net::SocketAddr, thread};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let tester = Arc::new(LoadTester::default());

    let handles = load_test(
        &tester,
        &args.host,
        &args.port,
        args.netname,
        args.verb,
        args.body,
        args.threads,
        args.iterations,
    )
    .await;

    // wait for threads to finish
    for handle in handles {
        let _ = handle.await.unwrap();
    }

    println!("Benchmark done. Here are the metrics for that run:");

    let serialized = serde_prometheus::to_string(&(*tester), None, HashMap::new()).unwrap();
    println!("{}", serialized);
}

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long)]
    pub host: String,

    #[clap(short, long)]
    pub port: String,

    #[clap(short, long, default_value_t = 10)]
    pub threads: u64,

    #[clap(short, long, default_value_t = 1000)]
    pub iterations: u64,

    #[clap(short, long, default_value = "gossip")]
    pub netname: String,

    #[clap(short, long, default_value = "gossip")]
    pub verb: String,

    #[clap(short, long, default_value = "hello")]
    pub body: String,
}
