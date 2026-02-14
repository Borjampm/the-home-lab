use tokio::sync::mpsc;
use tonic::Request;

use crate::event::AppEvent;

pub mod node {
    tonic::include_proto!("node");
}

pub mod greeter {
    tonic::include_proto!("greeter");
}

use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use node::node_monitor_client::NodeMonitorClient;
use node::CpuRequest;

pub fn spawn_cpu_stream(tx: mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        let mut client = match NodeMonitorClient::connect("http://127.0.0.1:50051").await {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(AppEvent::Disconnected(e.to_string())).await;
                return;
            }
        };

        let mut stream = match client
            .stream_cpu(Request::new(CpuRequest { refresh_ms: 500 }))
            .await
        {
            Ok(resp) => resp.into_inner(),
            Err(e) => {
                let _ = tx.send(AppEvent::Disconnected(e.to_string())).await;
                return;
            }
        };

        while let Ok(Some(reply)) = stream.message().await {
            if tx
                .send(AppEvent::CpuUpdate {
                    cpu_count: reply.cpu_count,
                    total_usage: reply.total_usage,
                })
                .await
                .is_err()
            {
                break;
            }
        }

        let _ = tx.send(AppEvent::Disconnected("Stream ended".into())).await;
    });
}

pub fn send_greeting(name: String, tx: mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        let mut client = match GreeterClient::connect("http://127.0.0.1:50051").await {
            Ok(c) => c,
            Err(e) => {
                let _ = tx
                    .send(AppEvent::GreeterResponse(format!("Error: {e}")))
                    .await;
                return;
            }
        };

        match client
            .say_hello(Request::new(HelloRequest { name }))
            .await
        {
            Ok(resp) => {
                let _ = tx
                    .send(AppEvent::GreeterResponse(resp.into_inner().message))
                    .await;
            }
            Err(e) => {
                let _ = tx
                    .send(AppEvent::GreeterResponse(format!("Error: {e}")))
                    .await;
            }
        }
    });
}
