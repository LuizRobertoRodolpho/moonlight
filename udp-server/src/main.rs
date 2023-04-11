use std::{sync::Arc, net::SocketAddr};

use moonlight_structs::moonlight_structs::{Message, Messaging};
use tokio::{net::UdpSocket, io, sync::mpsc};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:5000".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    // tokio::spawn(async move {
    //     while let Some((bytes, addr)) = rx.recv().await {
    //         let rec_msg = Message::deserialize_moon(bytes);

    //         // create function to handle message

    //         let len = s.send_to(&rec_msg.serialize_moon(), &addr).await.unwrap();
    //         println!("server {:?} bytes sent", len);
    //     }
    // });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("server {:?} bytes received from {:?}", len, addr);
        //let msg = String::from_utf8_lossy(&buf[..len]);
        //println!("server received: {}", msg);
        let rec_msg = Message::deserialize_moon(buf[..len].to_vec());

        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
