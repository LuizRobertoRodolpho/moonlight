use fltk::{
    app::{self, redraw},
    button::Button,
    draw,
    enums::{self, Event, Key, Color},
    frame::Frame,
    input::Input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    window::Window,
    valuator,
};
use moonlight_structs::{
    self,
    moonlight_structs::{Message, Messaging, Player},
};
use rand::{prelude::*, Error};
use std::{cell::RefCell, net::SocketAddr, rc::Rc, sync::Arc, time::Duration, borrow::Borrow};
use tokio::{io, net::UdpSocket, runtime::Runtime, sync::Mutex, time};

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let window = Window::new(20, 20, 800, 600, "Moonlight");
    let arc_window = Arc::new(Mutex::new(window.clone()));

    let mut rng = thread_rng();
    let num = rng.gen_range(5000..6000);
    let s = connect(num, arc_window).await.unwrap();

    main_render(s, app, window);

    Ok(())
}

fn main_render(socket: Arc<UdpSocket>, app: app::App, mut window: Window) {
    let socket2 = socket.clone();
    let mut rng = rand::thread_rng();
    let client_id: u32 = rng.gen();


    let mut frame = Frame::new(0, 0, 400, 200, "");
    let player_name_input = Input::new(100, 20, 80, 40, "Player Name");
    let port_settings_input = Input::new(230, 20, 80, 40, "Port");
    let mut connect_button = Button::new(330, 20, 80, 40, "Connect");

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

                // if client.is_connected
                match socket.try_send(&player_message.serialize_moon()) {
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
                // else
                // {
                //     println!("client not connected");
                // }
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

                // if client.is_connected
                match socket.try_send(&player_message.serialize_moon()) {
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
                // else
                // {
                //     println!("client not connected");
                // }
                true
            }
            Event::KeyUp => {
                match app::event_key().to_char() {
                    Some(key) => {
                        let key_val = key as u8 as char;
                        let mut delta_x = 0;
                        let mut delta_y = 0;
                        if key_val == 'w' || key_val == 'W' {
                            delta_x -= 1;
                        } else if key_val == 's' || key_val == 'S' {
                            delta_x += 1;
                        } else if key_val == 'a' || key_val == 'A' {
                            delta_y -= 1;
                        } else if key_val == 'd' || key_val == 'D' {
                            delta_y += 1;
                        }
                        let player_message = Message {
                            message_id: 1,
                            message_type: 2,
                            player: Player {
                                player_id: client_id,
                                player_name: "betolino".to_string(),
                            },
                            pos_x: delta_x,
                            pos_y: delta_y,
                        };

                        // if client.is_connected
                        match socket.try_send(&player_message.serialize_moon()) {
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
                        println!(
                            "Key down event detected! Keycode: {}, Key value: {}",
                            key, key_val
                        );
                    }
                    _ => (),
                }
                true
            }
            /* other events to be handled */
            _ => false,
        }
    });

    window.end();
    window.show();

    let (sender, receiver) = app::channel::<UIMessage>();

    connect_button.emit(sender, UIMessage::Connect);

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                UIMessage::Connect => {
                    // frame.set_label(player_name_input.value().as_str());
                    // client.connect(5001);
                    //let rt = Runtime::new().unwrap();
                    //rt.block_on(socket2.connect(player_name_input.value().as_str())).unwrap();
                }
            }
        }
    }

    app.run().unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum UIMessage {
    Connect,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub is_connected: bool,
    pub socket: Arc<UdpSocket>,
}

pub trait ClientTrait {
    fn new() -> Self;
    fn connect(&mut self, client_port: u16);
    // fn disconnect(&mut self);
}

// impl ClientTrait for Client {
//     fn new() -> Self {
//         let rt = Runtime::new().unwrap();
//         Client {
//             is_connected: false,
//             socket: rt.block_on(connect(5001)).unwrap(),
//         }
//     }
//     fn connect(&mut self, client_port: u16) {
//         let rt = Runtime::new().unwrap();
//         let result = rt.block_on(connect(client_port));

//         self.socket = result.unwrap();
//         self.is_connected = true;
//     }
//     // fn disconnect(&mut self) {
//     //     self.is_connected = false;
//     // }
// }

async fn connect(client_port: u16, mut window: Arc<Mutex<Window>>) -> Result<Arc<UdpSocket>, Error> {
    let local_address = format!("127.0.0.1:{}", client_port);
    let remote_address = "127.0.0.1:5000";
    let socket = UdpSocket::bind(local_address.parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    let socket_arc = Arc::new(socket);
    let udp_socket = socket_arc.clone();
    udp_socket.connect(remote_address).await.unwrap();
    let mut buf = [0; 1024];

    // receive server messages
    tokio::spawn(async move {
        let mut rect_x = 50;
        let mut rect_y = 50;
        let rect_width = 100;
        let rect_height = 100;

        loop {
            let (len, addr) = socket_arc.recv_from(&mut buf).await.unwrap();
            let rec_msg = Message::deserialize_moon(buf[..len].to_vec());

            // create fltk draw function
            let mut locked_window = window.lock().await;
            
            let mut square = Square { x: 50, y: 50, size: 50 };

            locked_window.draw(move |_| {
                draw::set_draw_color(Color::Blue);
                draw::draw_rect(rec_msg.pos_x, rec_msg.pos_y, rect_width, rect_height);
            });
            locked_window.redraw();

            println!("server confirmed message");
        }
    });
    let udp_clone = udp_socket.clone();
    // heartbeat
    // tokio::spawn(async move {
    //     loop {
    //         let heartbeat_message = Message {
    //             message_id: 1,
    //             message_type: 0,
    //             player: Player {
    //                 player_id: 0,
    //                 player_name: "betolino".to_string(),
    //             },
    //             pos_x: 900,
    //             pos_y: 900,
    //         };
    //         let heartbeat = heartbeat_message.serialize_moon();
    //         let _ = udp_clone.send(&heartbeat).await;
    //         time::sleep(time::Duration::from_secs(2)).await;
    //     }
    // });

    return Ok(udp_socket);
}


// DRAWING
#[repr(i32)]
#[derive(Copy, Clone)]
enum Direction {
    Positive = 1,
    Negative = -1,
}

struct Ball {
    wid: valuator::FillDial,
    pos: (i32, i32),             // x and y positions
    dir: (Direction, Direction), // x and y directions
}
struct Square {
    x: i32,
    y: i32,
    size: i32,
}

// impl d for Square {
//     fn draw(&self) {
//         drawing::set_draw_color(drawing::Color::from_rgb(255, 0, 0));
//         drawing::draw_rect(self.x, self.y, self.size, self.size);
//     }
// }