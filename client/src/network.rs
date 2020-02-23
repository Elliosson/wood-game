use super::Data;
//use futures::prelude::*;
use super::{Point, Renderable};
use core::time::Duration;
use futures::executor::block_on;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_timer::{Delay, Instant, TryFutureExt};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

pub const ASK_DATA_INTERVAL: u32 = 100;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<FnMut()>, time: u32) -> i32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn start_websocket(data: Arc<Mutex<Data>>) -> Result<WebSocket, JsValue> {
    // Connect to the game server
    let ws = WebSocket::new("ws://localhost:4321")?;

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
        match cloned_ws.send_with_str("register") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    /*
        std::thread::sleep(Duration::from_millis(500));
        console_log!("be wait");
        block_on(ball_wait());
    */

    /* loop {
        //sendMessage("ball".to_string(), ws.clone());
        //let id = "c70da2eb-1213-4815-afbe-5f0d3d0c0ed3";
        //ws.send_with_str(&format!("{} {}", id, "ball"));
        //wasm_timer::sleep(Duration::from_millis(500));
    }*/
    Ok(ws)
}

async fn ball_wait() {
    Delay::new(Duration::from_millis(100)).await.unwrap();
}

pub struct Ball {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
pub enum Message {
    Register,
    Play(),
    Config(),
    Side(),
    Points(),
    Opponent(),
    Ball(Ball),
    Move(f32),
    Exit(),
}

pub fn handle_responce(
    msg: String,
    ws: web_sys::WebSocket,
    data: Arc<Mutex<Data>>,
) -> Option<(Message, String)> {
    let mut parts = msg.split_whitespace();

    let command = parts.next()?;

    match command {
        "register" => {
            let mut data_guard = data.lock().unwrap();
            let id = parts.next()?.to_string();
            data_guard.my_uid = id.clone();
            sendMessage(id, "play".to_string(), ws);
        }
        "play" => {
            console_log!("received play");
            if parts.next()? == "ok" {
                console_log!("play is ok");
                let mut data_guard = data.lock().unwrap();
                let uid = data_guard.my_uid.clone();
                sendMessage(uid.clone(), "config".to_string(), ws.clone());
                sendMessage(uid.clone(), "side".to_string(), ws.clone());
                //start game

                let cloned_ws = ws.clone();
                let cb = Closure::wrap(Box::new(move || {
                    sendMessage(uid.clone(), "map".to_string(), cloned_ws.clone());
                    //sendMessage(uid.clone(), "position".to_string(), cloned_ws.clone());
                    console_log!("ask map and positions");
                }) as Box<FnMut()>);
                let interval_id = setInterval(&cb, ASK_DATA_INTERVAL);
                cb.forget();

                //set timeout
            }
        }
        "config" => {
            //lot of stuff
        }
        "ball" => {
            let mut data_guard = data.lock().unwrap();
            let x: f32 = parts.next()?.parse().unwrap();
            let y: f32 = parts.next()?.parse().unwrap();
            console_log!("receive ball position");
            data_guard.ball_x = x as i32;
            data_guard.ball_y = y as i32;
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
        }
        "map" => {
            //the date should be received as (x, y, glyph, fg, bg, render_order)
            let mut data_guard = data.lock().unwrap();
            let mut map = &mut data_guard.map;

            let infos: Vec<&str> = parts.collect();

            // TODO for now the all map is received, try to just update what we need
            map.clear();
            for window in infos.chunks(10) {
                let pos = Point {
                    x: window[0].parse().unwrap(),
                    y: window[1].parse().unwrap(),
                };
                console_log!("x {}, y{}", pos.x, pos.y);
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
        }
        _ => {}
    };
    return Some((Message::Exit(), "done".to_string()));
}

pub fn sendMessage(uid: String, message: String, ws: web_sys::WebSocket) {
    match ws.send_with_str(&format!("{} {}", uid, message)) {
        Ok(_) => console_log!("message successfully sent"),
        Err(err) => console_log!("error sending message: {:?}", err),
    }
}
