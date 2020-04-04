extern crate websocket;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use super::Data;
use super::{Point, Renderable};
use rltk::RGB;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://localhost:4321";

pub enum MessageReceived {
    Register,
    Play,
    Config,
    Position,
    Map,
    PlayerInfo,
    Unknown,
}

pub fn start_websocket(data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    println!("Connecting to {}", CONNECTION);

    let client = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();

    println!("Successfully connected");

    let (mut receiver, mut sender) = client.split().unwrap();

    let (tx, rx): (Sender<OwnedMessage>, Receiver<OwnedMessage>) = channel();

    let tx_1 = tx.clone();

    let _to_send_loop = thread::spawn(move || loop {
        {
            let mut to_send_guard = to_send.lock().unwrap();

            for message in to_send_guard.drain(..) {
                let _ = tx_1.send(OwnedMessage::Text(message));
            }
        }
        thread::sleep(Duration::from_millis(50));
    });

    let _send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let tx_2 = tx.clone();
    let _receive_loop = thread::spawn(move || {
        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_2.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_2.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_2.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                OwnedMessage::Text(message) => {
                    handle_responce(message.clone(), data.clone(), tx_2.clone());
                }
                _ => {
                    println!("Receive Loop: {:?}", message);
                }
            }
        }
    });
    /*
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        let message = match trimmed {
            "/close" => {
                // Close the connection
                let _ = tx.send(OwnedMessage::Close(None));
                break;
            }
            // Send a ping
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            // Otherwise, just send text
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match tx.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                break;
            }
        }
    }

    // We're exiting

    println!("Waiting for child threads to exit");

    let _ = to_send_loop.join();
    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Exited");
    */
}

pub fn handle_responce(
    msg: String,
    data: Arc<Mutex<Data>>,
    tx_2: Sender<OwnedMessage>,
) -> Option<(MessageReceived, String)> {
    let mut parts = msg.split_whitespace();

    let command = parts.next()?;

    let message = match command {
        "register" => {
            let mut data_guard = data.lock().unwrap();
            let id = parts.next()?.to_string();
            data_guard.my_uid = id.clone();
            send_message(id, "play".to_string(), tx_2);

            MessageReceived::Register
        }
        "play" => {
            println!("received play");
            if parts.next()? == "ok" {
                println!("play is ok");
                let data_guard = data.lock().unwrap();
                let uid = data_guard.my_uid.clone();
                send_message(uid.clone(), "config".to_string(), tx_2.clone());
                send_message(uid.clone(), "side".to_string(), tx_2.clone());
                //start game

                let tx_3 = tx_2.clone();
                // this thread will regulary send request to the server
                let _ask_loop = thread::spawn(move || loop {
                    send_message(uid.clone(), "map".to_string(), tx_3.clone());
                    send_message(uid.clone(), "player_info".to_string(), tx_3.clone());
                    println!("ask map and player_info");
                    thread::sleep(Duration::from_millis(50));
                });
                //set timeout
            }
            MessageReceived::Play
        }
        "config" => {
            //lot of stuff
            MessageReceived::Config
        }
        "positions" => {
            //TODO create a filter to separate the vec in 2 vec and iterate the two at the same time
            // Or find a way to iterate 2 at a time
            //Very Ugly
            let mut data_guard = data.lock().unwrap();
            let mut x: i32 = 0;
            data_guard.characters.clear();
            for (i, pos) in parts.enumerate() {
                if i % 2 == 0 {
                    x = pos.parse().unwrap();
                } else {
                    data_guard.characters.push(Point {
                        x: x,
                        y: pos.parse().unwrap(),
                    });
                }
            }
            MessageReceived::Position
        }
        "player_info" => {
            let mut data_guard = data.lock().unwrap();

            let info: String = parts.collect::<String>();
            println!("{}", info);

            data_guard.info_string = info;
            MessageReceived::PlayerInfo
        }
        "map" => {
            //the date should be received as (x, y, glyph, fg, bg, render_order)
            let mut data_guard = data.lock().unwrap();
            let map = &mut data_guard.map;

            let infos: Vec<&str> = parts.collect();

            // TODO for now the all map is received, try to just update what we need
            map.clear();
            for window in infos.chunks(10) {
                let pos = Point {
                    x: window[0].parse().unwrap(),
                    y: window[1].parse().unwrap(),
                };
                //console_log!("x {}, y{}", pos.x, pos.y);
                let renderable = Renderable {
                    glyph: window[2].parse().unwrap(),
                    fg: RGB::named((
                        window[3].parse().unwrap(),
                        window[4].parse().unwrap(),
                        window[5].parse().unwrap(),
                    )),
                    bg: RGB::named((
                        window[6].parse().unwrap(),
                        window[7].parse().unwrap(),
                        window[8].parse().unwrap(),
                    )),
                    render_order: window[9].parse().unwrap(),
                };

                map.push((pos, renderable));
            }
            MessageReceived::Map
        }
        _ => MessageReceived::Unknown,
    };
    return Some((message, "done".to_string()));
}

pub fn send_message(uid: String, message: String, tx: Sender<OwnedMessage>) {
    tx.send(OwnedMessage::Text(format!("{} {}", uid, message)))
        .expect("Unable to send message to channel");
}
