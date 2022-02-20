mod config;

mod pb;
mod error;
mod storage;
mod service;
mod network;

pub use config::*;
pub use network::*;
pub use pb::abi::*;
pub use error::KvError;
pub use storage::*;
pub use service::*;

use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::client;
use tracing::{info, instrument};

/// 通过配置创建 KV 服务器
#[instrument(skip_all)]
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    let acceptor =
        TlsServerAcceptor::new(&config.tls.cert, &config.tls.key, config.tls.ca.as_deref())?;

    let addr = &config.general.addr;
    // match &config.storage {
    //     StorageConfig::MemTable => start_tls_server(addr, MemTable::new(), acceptor).await?,
    //     StorageConfig::SledDb(path) => start_tls_server(addr, SledDb::new(path), acceptor).await?,
    // };
    start_tls_server(addr, MemTable::new(), acceptor).await?;

    Ok(())
}

/// 通过配置创建 KV 客户端
#[instrument(skip_all)]
pub async fn start_client_with_config(
    config: &ClientConfig,
) -> Result<ProstClientStream<client::TlsStream<TcpStream>>> {
    let addr = &config.general.addr;
    let tls = &config.tls;

    let identity = tls.identity.as_ref().map(|(c, k)| (c.as_str(), k.as_str()));
    let connector = TlsClientConnector::new(&tls.domain, identity, tls.ca.as_deref())?;
    let stream = TcpStream::connect(addr).await?;
    let stream = connector.connect(stream).await?;

    // 打开一个 stream
    Ok(ProstClientStream::new(stream))
}

async fn start_tls_server<Store: Storage> (
    addr: &str,
    store: Store,
    acceptor: TlsServerAcceptor,
) -> Result<()> where service::Service: From<service::ServiceInner<Store>> {
    let service: Service<MemTable> = ServiceInner::new(store).into();
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let tls = acceptor.clone();
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let stream = tls.accept(stream).await?;
        let stream = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move { stream.process().await });
    }
}