use std::{sync::Arc, net::SocketAddr};

use tokio::{net::UdpSocket, io, sync::mpsc};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:5000".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("server {:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("server {:?} bytes received from {:?}", len, addr);
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("server received: {}", msg);

        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
