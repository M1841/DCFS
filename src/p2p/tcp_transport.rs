use std::{
  collections::HashMap, 
  error::Error, 
  net::SocketAddr, 
};
use tokio::{
  net::{TcpListener, TcpStream},
  sync::Mutex,
};

use super::transport::Peer;

struct TcpTransport {
  listener_address: String,
  listener: TcpListener,
  peers: Mutex<HashMap<SocketAddr, Box<dyn Peer + Send + Sync>>>
}

impl TcpTransport {
  async fn new(listener_address: &str) -> Result<TcpTransport, Box<dyn Error>> {
    let listener = TcpListener::bind(listener_address).await?;

    Ok(TcpTransport {
      listener_address: String::from(listener_address),
      listener,
      peers: Mutex::new(HashMap::new())
    })
  }

  async fn start_accept_loop(&self) {
    loop {
      let connection = self.listener.accept().await;
      tokio::spawn(async move {
        match connection {
          Ok(connection) => {
              Self::handle_connection(connection).await;
          }
          Err(error) => {
            eprintln!("TCP Accept Error: {error}")
          }
        }
      });
    }
  }

  async fn handle_connection(connection: (TcpStream, SocketAddr)) {
    dbg!(connection);
  }

}

#[cfg(test)]
mod tests {
    use super::TcpTransport;

  #[tokio::test]
  async fn test_tcp_transport() {
    let listener_address = "127.0.0.1:8080";
    let transport = TcpTransport::new(listener_address).await.unwrap();

    assert_eq!(transport.listener_address, listener_address);

    transport.start_accept_loop().await;
  }
}
