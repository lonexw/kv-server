
use anyhow::Result;
use futures::StreamExt;
use std::sync::Arc;
use async_prost::AsyncProstStream;
use dashmap::DashMap;
use kv::{
    command_request::RequestData, Value, CommandRequest, CommamdResponse, Hset, KvError, Kvpair
};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();    
    
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    // Use DashMap 创建存放在内存中的 kv store
    let table: Arc<DashMap<String, Value>> = Arc::new(DashMap::new());

    loop {
        // 接收到客户端请求
        let (stream, addr) = listener.accept().await?;
        info!("Client connected. From {:?}", addr);

        // Clone table, 便于在 tokio 任务中可用
        let db = table.clone();
        
        // 创建一个 tokio 任务处理该客户端的请求
        tokio::spawn(async move {
            // 使用 AsyncProstStream 处理 TCP Frame
            let mut stream = 
                AsyncProstStream::<_, CommandRequest, CommamdResponse, _>::from(stream).for_async();
            
            // 从 stream 中消费消息
            while let Some(Ok(msg)) = stream.next().wait {
                info!("Got a new command: {:?}", msg);
                let resp: CommamdResponse = match msg.request_data {
                    Some(RequestData::Hset(cmd)) => hset(cmd, &db),
                    _ => unimplemented!(),
                };

                info!("Got a response: {:?}", resp);
                stream.send(resp).await.unwrap();
            }
        });


    }
}

fn Hset(cmd: Hset, db: &DashMap<String, Value) -> CommamdResponse {
    match cmd.pair {
        Some(Kvpair {
            key,
            value: Some(v),
        }) => {
            let old = db.insert(key, v).unwrap_or_default();
            old.into()
        },
        v => KvError::InvalidCommand(format!("hset: {:?}", v)).into()
    }
}
