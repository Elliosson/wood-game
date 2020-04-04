use super::Data;
use super::{Point, Renderable};
use rltk::RGB;
use std::sync::{Arc, Mutex};

pub enum Message {
    Register,
    Play(String),
    Config,
    Position,
    Map,
    PlayerInfo,
    Unknown,
}

pub fn handle_responce<F>(
    msg: String,
    data: Arc<Mutex<Data>>,
    message_sender: F,
) -> Option<(Message, String)>
where
    F: Fn(String, String) + Clone + 'static,
{
    let mut parts = msg.split_whitespace();

    let command = parts.next()?;

    let message = match command {
        "register" => {
            let mut data_guard = data.lock().unwrap();
            let id = parts.next()?.to_string();
            data_guard.my_uid = id.clone();
            message_sender(id, "play".to_string());

            Message::Register
        }
        "play" => {
            println!("received play");
            if parts.next()? == "ok" {
                println!("play is ok");
                let data_guard = data.lock().unwrap();
                let uid = data_guard.my_uid.clone();
                message_sender(uid.clone(), "config".to_string());
                message_sender(uid.clone(), "side".to_string());
                //start game
                Message::Play(uid.clone())
            } else {
                Message::Unknown
            }
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
            println!("{}", info);

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
