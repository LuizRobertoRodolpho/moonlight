use fltk::{
    app::{self, redraw},
    button::Button,
    draw,
    enums::{Color, Event},
    frame::Frame,
    input::Input,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use moonlight_structs::{
    self,
    moonlight_structs::{Message, Messaging, Player, PlayerTrait},
};
use rand::{prelude::*, Error};
use std::{net::SocketAddr, sync::{Arc, RwLock, RwLockReadGuard}};
use tokio::{io, net::UdpSocket, sync::Mutex};

static CAPTURE_MOUSE_MOVE: bool = false;
static CAPTURE_MOUSE_CLICK: bool = false;
static CAPTURE_KEYUP: bool = true;

#[tokio::main]
async fn main() -> io::Result<()> {
    // me as player =)
    let player: Player = Player::new("BetolinoRox".to_string());
    let shared_player = Arc::new(RwLock::new(player));
    let write_player = shared_player.clone();

    // initialize fltk
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let window = Window::new(20, 20, 800, 600, "Moonlight");
    let arc_window = Arc::new(Mutex::new(window.clone()));

    // TODO: Use random port when creating trait new to Network object
    let mut rng = thread_rng();
    let num = rng.gen_range(5000..6000);

    // connect to server
    let s = connect(num, arc_window, write_player).await.unwrap();

    // fltk render beta window
    main_render(s, app, window, shared_player);

    Ok(())
}

fn main_render(socket: Arc<UdpSocket>, 
                app: app::App, 
                mut window: Window, 
                read_player: Arc<RwLock<Player>>) {
    
    // TODO: Create new window/widget for Start Screen
    let mut frame = Frame::new(0, 0, 400, 200, "");
    // let player_name_input = Input::new(100, 20, 80, 40, "Player Name");
    // let port_settings_input = Input::new(230, 20, 80, 40, "Port");
    // let mut connect_button = Button::new(330, 20, 80, 40, "Connect");

    window.handle(move |_widget, ev: Event| {

        let mut temp_player = read_player.read().unwrap().clone();

        match ev {
            Event::Move => {
                if CAPTURE_MOUSE_MOVE {
                    // TODO: create function/trait to create new message
                    temp_player.pos_x = app::event_coords().0;
                    temp_player.pos_y = app::event_coords().1;

                    let player_message = Message {
                        message_id: 1,
                        message_type: 0,
                        player: temp_player
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
                }

                true
            }
            Event::Push => {
                if CAPTURE_MOUSE_CLICK {
                    // TODO: create function/trait to create new message
                    temp_player.pos_x = app::event_coords().0;
                    temp_player.pos_y = app::event_coords().1;
                    
                    let player_message = Message {
                        message_id: 1,
                        message_type: 0,
                        player: temp_player
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
                }

                true
            }
            Event::KeyUp => {
                if CAPTURE_KEYUP {
                    match app::event_key().to_char() {
                        Some(key) => {
                            let key_val = key as u8 as char;
                            // let mut delta_x = 0;
                            // let mut delta_y = 0;
                            if key_val == 'w' || key_val == 'W' {
                                temp_player.pos_y -= 10;
                            } else if key_val == 's' || key_val == 'S' {
                                temp_player.pos_y += 10
                            } else if key_val == 'a' || key_val == 'A' {
                                temp_player.pos_x -= 10;
                            } else if key_val == 'd' || key_val == 'D' {
                                temp_player.pos_x += 10;
                            }

                            // TODO: create function/trait to create new message
                            let player_message = Message {
                                message_id: 1,
                                message_type: 2,
                                player: temp_player
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
                        },
                        _ => println!("Key not found."),
                    }
                }
                true
            }
            /* other events to be handled */
            _ => false,
        }
    });

    window.end();
    window.show();
    
    // TODO: create a trait to handle this
    // button clicks and message pattern
    let (sender, receiver) = app::channel::<UIMessage>();

    //connect_button.emit(sender, UIMessage::Connect);

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                UIMessage::Connect => {
                    // TODO: create pointer to socket (future Network trait)
                    //      client.connect(5001);
                    //      let rt = Runtime::new().unwrap();
                    //      rt.block_on(socket2.connect(player_name_input.value().as_str())).unwrap();
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

async fn connect(
    client_port: u16, 
    window: Arc<Mutex<Window>>, 
    player: Arc<RwLock<Player>>) -> Result<Arc<UdpSocket>, Error> {
    
    let local_address = format!("127.0.0.1:{}", client_port);
    let remote_address = "127.0.0.1:5000";
    let socket = UdpSocket::bind(local_address.parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    let socket_arc = Arc::new(socket);
    let udp_socket = socket_arc.clone();
    udp_socket.connect(remote_address).await.unwrap();
    let mut buf = [0; 1024];

    // receive server messages - render
    tokio::spawn(async move {
        let mut rect_x = 50;
        let mut rect_y = 50;
        let rect_width = 100;
        let rect_height = 100;

        loop {
            let (len, addr) = socket_arc.recv_from(&mut buf).await.unwrap();
            let rec_msg = Message::deserialize_moon(buf[..len].to_vec());

            // update player real position
            write_position(
                player.clone(), 
                rec_msg.player.pos_x,
                rec_msg.player.pos_y).await;

            // create fltk draw function
            let mut locked_window = window.lock().await;
            
            locked_window.draw(move |_| {
                draw::set_draw_color(Color::Blue);
                draw::draw_rect(rec_msg.player.pos_x, rec_msg.player.pos_y, rect_width, rect_height);
                redraw();
            });
            locked_window.redraw();
            drop(locked_window);

            println!("server confirmed message");
        }
    });
    // let udp_clone = udp_socket.clone();
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

async fn write_position(obj: Arc<RwLock<Player>>, pos_x: i32, pos_y: i32) {
    // acquire a write lock on the object
    let mut write_guard = obj.write().unwrap();

    // modify the object
    write_guard.pos_x = pos_x;
    write_guard.pos_y = pos_y;
    println!("Async function modified object: {:?}", write_guard);
}