use std::{sync::Arc, net::SocketAddr, collections::HashMap};

use moonlight_structs::moonlight_structs::{Message, Messaging, Player};
use tokio::{net::UdpSocket, sync::mpsc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sock = UdpSocket::bind("127.0.0.1:5000".parse::<SocketAddr>().unwrap()).await?;

    // HashMap to keep track of connected clients
    let mut clients: HashMap<SocketAddr, bool> = HashMap::new();

    let r = Arc::new(sock);
    let s = r.clone();
    let s2 = r.clone();

    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let rec_msg = Message::deserialize_moon(bytes);

            // create function to handle message

            let len = s2.send_to(&rec_msg.serialize_moon(), &addr).await.unwrap();
            println!("server {:?} bytes sent", len);
            
        }
    });

    loop {
        let mut buf = [0; 1024];
        let (len, src) = r.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, src);

        // Add the source address to the set of clients.
        clients.insert(src, true);
        let mut arc_clients = Box::new(clients.clone());

        // Handle the received data.
        let rec_msg = Message::deserialize_moon(buf[..len].to_vec());

        // Poll the HashMap to see if any clients have disconnected
        let mut disconnected_clients = Vec::new();

        // If necessary, remove disconnected clients.
        for (client, connected) in clients.clone() {
            // let ping_message = ping_message().serialize_moon();
            //match s.send_to(&ping_message, client).await {
            match s.send_to(&buf[..len].to_vec(), client).await {
                Ok(_size) => {
                    println!("Sent ping to {}, size {}", client, _size);
                }
                Err(_) => {
                    // Client is no longer connected
                    disconnected_clients.push(client);
                }
            }
        }

        // Remove disconnected clients from the HashMap
        for dc in disconnected_clients {
            arc_clients.remove(&dc);
            println!("Client {} disconnected.", dc);
        }
    }
}

fn ping_message() -> Message {
    let message = Message {
        message_id: 0,
        message_type: 10,
        player: Player {
            player_id: 0,
            player_name: "admin".to_string(),
            pos_x: 0,
            pos_y: 0
        }
    };

    return message;
}