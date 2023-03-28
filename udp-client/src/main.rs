use chrono::{Local};
use fltk::{
    app::{self, App},
    enums::Event,
    window::Window, prelude::{WidgetBase, WidgetExt, GroupExt},
};
use rand::prelude::*;
use tokio::{io, net::UdpSocket};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:5001").await?;

    let remote_addr = "127.0.0.1:5000";
    sock.connect(remote_addr).await?;

    let mut rng = rand::thread_rng();
    let client_id: u16 = rng.gen();

    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");

    window.end();
    window.show();
    window.handle(move |_widget, ev: Event| {
        match ev {
            Event::Move => {
                let timestamp = Local::now().format("%H:%M:%S%.3f").to_string();
                let msg = format!(
                    "[{}] coords ({}.{})\n",
                    timestamp,
                    app::event_coords().0,
                    app::event_coords().1
                );
                match sock.try_send(msg.as_bytes()) {
                    Ok(n) => {
                        println!("client {} sent {} bytes", client_id, n);
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
                let msg = format!("[{}.{}]\n", app::event_coords().0, app::event_coords().1);
                match sock.try_send(msg.as_bytes()) {
                    Ok(n) => {
                        println!("client {} sent {} bytes", client_id, n);
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
