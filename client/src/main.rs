use std::sync::Arc;

use fltk::{
    app::{self, App},
    enums::Event,
    prelude::*,
    window::Window,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

fn send_message(msg: String, writer_mutex: Arc<Mutex<TcpStream>>) {
    println!("POS:! {}", msg);
    let stream_arc = Arc::clone(&writer_mutex);
    tokio::spawn(async move {
        let mut lock = stream_arc.lock().await;
        lock.write_all(msg.as_bytes()).await.unwrap();
        lock.shutdown().await.unwrap();
    });
}

fn message_handler(reader_mutex: Arc<Mutex<TcpStream>>)
{
    let stream_arc = Arc::clone(&reader_mutex);
    tokio::spawn(async move {
        loop {
            let mut lock = stream_arc.lock().await;
            let mut buffer = [0, 16];
            let len = lock.read(&mut buffer).await.unwrap();
            let message = String::from_utf8_lossy(&buffer[..len]);
            println!("received: {}", message);
        }
    });
}

#[tokio::main]
async fn main() {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:5000").await.unwrap();
    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");

    window.end();
    window.show();

    let writer_mutex = Arc::new(Mutex::new(stream));
    let reader_mutex = Arc::clone(&writer_mutex);

    message_handler(reader_mutex);

    window.handle(move |_widget, ev: Event| {
        match ev {
            Event::Move => {
                let msg = format!("({}.{})\n", app::event_coords().0, app::event_coords().1);
                send_message(msg, Arc::clone(&writer_mutex));
                true
            }
            Event::Push => {
                let msg = format!("[{}.{}]\n", app::event_coords().0, app::event_coords().1);
                send_message(msg, Arc::clone(&writer_mutex));
                true
            }
            /* other events to be handled */
            _ => false,
        }
    });
    app.run().unwrap();
}
