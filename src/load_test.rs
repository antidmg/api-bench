use std::{net::SocketAddr, sync::Arc};

use metered::{HitCount, Throughput};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Default, Debug, serde::Serialize)]
pub struct LoadTester {
    metrics: LoadTestMetrics,
}

#[metered::metered(registry = LoadTestMetrics)]
#[measure([HitCount, Throughput])]
impl LoadTester {
    #[measure]
    pub async fn send_melnet_request(
        &self,
        addr: SocketAddr,
        id: u128,
        netname: &str,
        verb: &str,
        body: &str,
    ) {
        let req = MelnetRequest {
            id,
            body: body.into(),
        };
        let _ = melnet::request::<MelnetRequest, MelnetResponse>(addr, netname, verb, req).await;
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn load_test(
    tester: &Arc<LoadTester>,
    host: &str,
    port: &str,
    netname: String,
    verb: String,
    body: String,
    threads: u64,
    iterations: u64,
) -> Vec<JoinHandle<()>> {
    let addr_details = format!("{}:{}", host, port);
    let addr: SocketAddr = addr_details
        .parse()
        .expect("Unable to parse socket address");

    let mut handles = Vec::new();
    for _ in 0..threads {
        let netname = netname.clone();
        let verb = verb.clone();
        let body = body.clone();
        let tester = Arc::clone(tester);

        let handle = tokio::spawn(async move {
            for i in 0..iterations {
                tester
                    .send_melnet_request(
                        addr,
                        i as u128,
                        netname.as_ref(),
                        verb.as_ref(),
                        body.as_ref(),
                    )
                    .await;
            }
        });

        handles.push(handle);
    }

    handles
}
#[derive(Clone, Serialize, Debug)]
pub struct MelnetRequest {
    id: u128,
    body: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct MelnetResponse {}
