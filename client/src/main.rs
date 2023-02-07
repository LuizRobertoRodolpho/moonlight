use std::sync::Arc;

use fltk::{
    app::{self, App},
    enums::Event,
    prelude::*,
    window::Window,
};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

#[tokio::main]
async fn main() {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:5000").await.unwrap();

    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");

    window.end();
    window.show();

    let stream_clone = Arc::new(Mutex::new(stream));

    window.handle(move |_widget, ev: Event| {
        match ev {
            Event::Move => {
                let msg = format!("({}.{})\n", app::event_coords().0, app::event_coords().1);
                println!("POS:! {}", msg);

                let stream_arc = Arc::clone(&stream_clone);
                tokio::spawn(async move {
                    let mut lock = stream_arc.lock().await;
                    lock.write_all(msg.as_bytes()).await.unwrap();
                });
                true
            }
            /* other events to be handled */
            _ => false,
        }
    });
    app.run().unwrap();
}
