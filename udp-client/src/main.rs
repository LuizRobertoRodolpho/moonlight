use fltk::{
    app::{self, App},
    enums::Event,
    prelude::*,
    window::Window,
};
use tokio::{net::UdpSocket, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:5001").await?;

    let remote_addr = "127.0.0.1:5000";
    sock.connect(remote_addr).await?;
    // let mut buf = [0; 1024];
    // loop {
    //     sock.try_send(b"hello world");
    //     // let len = sock.recv(&mut buf).await?;
    //     // println!("{:?} bytes received from {:?}", len, remote_addr);

    //     // let len = sock.send(&buf[..len]).await?;
    //     // println!("{:?} bytes sent", len);
    // }

    let app = App::default();
    let mut window = Window::new(20, 20, 800, 600, "Moonlight");

    window.end();
    window.show();
    window.handle(move |_widget, ev: Event| {
        match ev {
            Event::Move => {
                let msg = format!("({}.{})\n", app::event_coords().0, app::event_coords().1);
                match sock.try_send(b"hello world") {
                    Ok(n) => {
                        println!("client sent {} bytes", n);
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
                match sock.try_send(b"hello world") {
                    Ok(n) => {
                        println!("client sent {} bytes", n);
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
