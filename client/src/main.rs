use fltk::{
    app::{App, self},
    window::Window,
    enums::Event,
    prelude::*
};
use tokio::{
    io::{AsyncWriteExt },
    net::{TcpStream},
};
async fn send_message(mut stream: TcpStream, msg: String) -> Result<(), Box<dyn std::error::Error>> {
    stream.write_all(msg.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:5000").await.unwrap();

    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");
    
    window.end();
    window.show();

    window.handle(move |widget, ev: Event| {
        match ev {
            Event::Move => {                
                println!("Pushed! {}", app::event_coords().0);
                println!("Pushed! {}", app::event_coords().1);

                let (_, mut writer) = stream.split();
                let msg = format!("{}.{}\n", app::event_coords().0, app::event_coords().1);
                writer.write_all(msg.as_bytes()); // make blocking
                true
            },
            /* other events to be handled */
            _ => false,
        }
    });
    app.run().unwrap();
}

