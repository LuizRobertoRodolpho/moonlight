use std::{sync::Arc, net::SocketAddr};
use fltk::{
    app::{self, App},
    enums::Event,
    window::Window, prelude::{WidgetBase, WidgetExt, GroupExt},
};
use rand::prelude::*;
use tokio::{io, net::UdpSocket};
use moonlight_structs::{self, moonlight_structs::{Player, Message, Messaging}};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:5001".parse::<SocketAddr>().unwrap()).await?;
    let remote_addr = "127.0.0.1:5000";
    let r = Arc::new(sock);
    let s = r.clone();
    s.connect(remote_addr).await?;
    let mut buf = [0; 1024];

    tokio::spawn(async move {
        loop
        {
            let (len, addr) = r.recv_from(&mut buf).await.unwrap();
            let rec_msg = Message::deserialize_moon(buf[..len].to_vec());
            // create fltk draw function

        }
    });

    let mut rng = rand::thread_rng();
    let client_id: u32 = rng.gen();

    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");

    window.end();
    window.show();
    window.handle(move |_widget, ev: Event| {
        match ev {
            Event::Move => {
                let player_message = Message {
                    message_id: 1,
                    message_type: 0,
                    player: Player {
                        player_id: client_id,
                        player_name: "betolino".to_string(),
                    },
                    pos_x: app::event_coords().0,
                    pos_y: app::event_coords().1,
                };

                match s.try_send(&player_message.serialize_moon()) {
                    Ok(n) => {
                        //println!("client {} sent {} bytes", client_id, n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        println!("ddd")
                    }
                    Err(e) => {
                        println!("ddd")
                    }
                }
                true
            }
            Event::Push => {
                let player_message = Message {
                    message_id: 1,
                    message_type: 1,
                    player: Player {
                        player_id: client_id,
                        player_name: "betolino".to_string(),
                    },
                    pos_x: app::event_coords().0,
                    pos_y: app::event_coords().1,
                };
                
                match s.try_send(&player_message.serialize_moon()) {
                    Ok(n) => {
                        //println!("client {} sent {} bytes", client_id, n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        println!("ddd")
                    }
                    Err(e) => {
                        println!("ddd")
                    }
                }
                true
            }
            /* other events to be handled */
            _ => false,
        }
    });
    
    app.run().unwrap();

    Ok(())
}
