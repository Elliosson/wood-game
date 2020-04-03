use super::Data;
//use futures::prelude::*;
use super::{Point, Renderable};

use rltk::RGB;

use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

pub const ASK_DATA_INTERVAL: u32 = 100;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn start_websocket(
    data: Arc<Mutex<Data>>,
    to_send: Arc<Mutex<Vec<String>>>,
) -> Result<WebSocket, JsValue> {
    // Connect to the game server
    let ws = WebSocket::new("ws://localhost:4321")?;
    //let ws = WebSocket::new("ws://51.68.141.5:4321")?;

    let cloned_ws = ws.clone();
    //send message to the serveer
    let cb = Closure::wrap(Box::new(move || {
        let mut to_send_guard = to_send.lock().unwrap();

        for message in to_send_guard.drain(..) {
            cloned_ws
                .send_with_str(&message)
                .expect("Unable to send message");
        }
    }) as Box<dyn FnMut()>);
    let _interval_id = setInterval(&cb, ASK_DATA_INTERVAL);
    cb.forget();

    let cloned_ws = ws.clone();

    // create callback
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // handle message

        let response = e
            .data()
            .as_string()
            .expect("Can't convert received data to a string");

        console_log!("message event, received data: {:?}", response);
        handle_responce(response.clone(), cloned_ws.clone(), data.clone());
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        match cloned_ws.send_with_str("open com") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(ws)
}

pub enum Message {
    Register,
    Play,
    Config,
    Position,
    Map,
    PlayerInfo,
    Unknown,
}

pub fn handle_responce(
    msg: String,
    ws: web_sys::WebSocket,
    data: Arc<Mutex<Data>>,
) -> Option<(Message, String)> {
    let mut parts = msg.split_whitespace();

    let command = parts.next()?;

    let message = match command {
        "register" => {
            let mut data_guard = data.lock().unwrap();
            let id = parts.next()?.to_string();
            data_guard.my_uid = id.clone();
            send_message(id, "play".to_string(), ws);

            Message::Register
        }
        "play" => {
            console_log!("received play");
            if parts.next()? == "ok" {
                console_log!("play is ok");
                let data_guard = data.lock().unwrap();
                let uid = data_guard.my_uid.clone();
                send_message(uid.clone(), "config".to_string(), ws.clone());
                send_message(uid.clone(), "side".to_string(), ws.clone());
                //start game

                let cloned_ws = ws.clone();
                // this closure will regulary send request to the server
                let cb = Closure::wrap(Box::new(move || {
                    send_message(uid.clone(), "map".to_string(), cloned_ws.clone());
                    send_message(uid.clone(), "player_info".to_string(), cloned_ws.clone());
                    console_log!("ask map and player_info");
                }) as Box<dyn FnMut()>);
                let _interval_id = setInterval(&cb, ASK_DATA_INTERVAL);
                cb.forget();
                //set timeout
            }
            Message::Play
        }
        "config" => {
            //lot of stuff
            Message::Config
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
            Message::Position
        }
        "player_info" => {
            let mut data_guard = data.lock().unwrap();

            let info: String = parts.collect::<String>();
            console_log!("{}", info);

            data_guard.info_string = info;
            Message::PlayerInfo
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
            Message::Map
        }
        _ => Message::Unknown,
    };
    return Some((message, "done".to_string()));
}

pub fn send_message(uid: String, message: String, ws: web_sys::WebSocket) {
    match ws.send_with_str(&format!("{} {}", uid, message)) {
        Ok(_) => console_log!("message successfully sent"),
        Err(err) => console_log!("error sending message: {:?}", err),
    }
}
