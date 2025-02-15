use std::borrow::Cow;
use enigo::{Button, Enigo, Mouse};
use enigo::Direction::{Press, Release};

pub(crate) fn simulate_mouse_event(message: Cow<str>){
    let mut enigo = Enigo::new(&Default::default()).expect("Failed to create enigo");
    let pos = enigo.location().unwrap();
    if let Some((event_type, x, y)) = parse_message(&message) {
        match event_type {
            "MOVE" => {
                enigo.move_mouse(pos.0 + x, pos.1 + y, Default::default()).expect("TODO: panic message");
                println!("Event type: {}, x: {}, y: {}", event_type, x, y);
            }
            "LEFT_DOWN" => {
                enigo.button(Button::Left, Press).expect("Failed to press left button");
                println!("Left down");
            }
            "LEFT_UP" => {
                enigo.button(Button::Left, Release).expect("Failed to release left button");
                println!("Left up");
            }
            _ => {
                eprintln!("Invalid event type");
            }
        }
    } else {
        eprintln!("Invalid message");
    }
}

pub(crate) fn parse_message(message: &str) -> Option<(&str, i32, i32)> {
    let parts: Vec<&str> = message.split_whitespace().collect();
    if parts.len() == 3 && parts[0] == "MOVE" {
        if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
            return Some((parts[0], x, y));
        }
    }
    else if parts.len() == 1 {
        return Some((parts[0], 0, 0));
    }
    None
}