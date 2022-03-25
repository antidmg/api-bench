use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    net::{SocketAddr, TcpListener},
    thread,
};
use tic::{Interest, Receiver, Sample};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let args = Args::parse();

    let addr_details = format!("{}:{}", args.host, args.port);
    let addr: SocketAddr = addr_details
        .parse()
        .expect("Unable to parse socket address");

    let mut handles = Vec::new();
    for _ in 0..args.threads {
        let netname = args.netname.clone();
        let verb = args.verb.clone();

        let handle = tokio::spawn(async move {
            for i in 0..args.iterations {
                let req = MelnetRequest {
                    id: i as u128,
                    body: "hello".into(),
                };

                let response = melnet::request::<MelnetRequest, MelnetResponse>(
                    addr,
                    netname.as_str(),
                    verb.as_str(),
                    req,
                )
                .await;
                println!("response: {:?}", response);
            }
        });

        handles.push(handle);
    }

    // wait for threads to finish
    for handle in handles {
        let _ = handle.await.unwrap();
    }

    println!("benchmark done.");

    for stream in listener.incoming() {
        let _ = stream.unwrap();
    }
}

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    host: String,

    #[clap(short, long)]
    port: String,

    #[clap(short, long, default_value_t = 10)]
    threads: i8,

    #[clap(short, long, default_value_t = 1000)]
    iterations: i64,

    #[clap(short, long)]
    netname: String,

    #[clap(short, long)]
    verb: String,
}

#[derive(Clone, Serialize, Debug)]
struct MelnetRequest {
    id: u128,
    body: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
struct MelnetResponse {}

// define an enum of stats labels
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Metric {
    Success,
}

// implement the fmt::Display trait
impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Metric::Success => write!(f, "ok"),
        }
    }
}
