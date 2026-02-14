use tonic::Request;

pub mod node {
    tonic::include_proto!("node");
}
use node::node_monitor_client::NodeMonitorClient;
use node::CpuRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeMonitorClient::connect("http://127.0.0.1:50051").await?;
    let mut stream = client
        .stream_cpu(Request::new(CpuRequest { refresh_ms: 1000 }))
        .await?
        .into_inner();

    while let Some(reply) = stream.message().await? {
        println!(
            "CPUs: {} | Usage: {:.1}%",
            reply.cpu_count, reply.total_usage
        );
    }

    Ok(())
}
