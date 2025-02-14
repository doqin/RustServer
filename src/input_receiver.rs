use std::borrow::Cow;
use std::fmt::Debug;
use enigo::{Enigo, Mouse};

pub(crate) fn simulate_mouse_event(message: Cow<str>){
    let mut enigo = Enigo::new(&Default::default()).expect("Failed to create enigo");
    let pos = enigo.location().unwrap();
    if let Some((event_type, x, y)) = parse_message(&message) {
        println!("Event type: {}, x: {}, y: {}", event_type, x, y);
        match event_type {
            "MOVE" => {
                enigo.move_mouse(pos.0 + x, pos.1 + y, Default::default()).expect("TODO: panic message");
            }
            "LEFT_DOWN" => {
                println!("Left down");
            }
            "LEFT_UP" => {
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
    None
}