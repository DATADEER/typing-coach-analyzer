use postgres::{Client, Error as PostgresError, NoTls, Row};
use rdev::Key;

fn main() {

    let mut conn = Client::connect("host=localhost user=postgres password=filtrate-MENIAL-immobile-floppy dbname=main", NoTls).unwrap_or_else(|e| {
        eprintln!("Failed to open connection: {}", e);
        std::process::exit(1);
    });

    match get_key_events(&mut conn) {
        Ok(all_rows) => {

            let rows = all_rows.clone(); //[..=61].to_vec();

            for (index,row) in rows.iter().enumerate() {
                let value = row.get::<_, String>("value");
                let key = string_to_key(&row.get::<_,String>("key"));

                match key {
                    Some(Key::Space) => print!("{}", " "),
                    Some(Key::Return) => println!("{}",""),
                    _ => ()
                }

                if value.chars().all(char::is_alphabetic){
                    if should_print_alphabetic_key(index, rows.clone()){
                        print!("{}", value);
                    }


                }
                //println!("row: {:?}", row);

            }
        }
        Err(e) => {
            eprintln!("Failed to get key events: {}", e);
            std::process::exit(1);
        }
    }

}

fn should_print_alphabetic_key(key_index: usize, all_key_events: Vec<Row>) -> bool {

    let mut range_start_index: Option<usize> = None;
    let mut range_end_index: Option<usize> = None;
    let mut has_found_backspace= false;
    for (index,row) in all_key_events.iter().enumerate().skip(key_index) {
        let key = string_to_key(&row.get::<_, String>("key"));
        let value = row.get::<_, String>("value");

        // TODO: Handle what happens if Some(Key::Space) never happens
        if let Some(Key::Space) = key {
            range_end_index = Some(index - 1);
            break;
        }

        if let Some(Key::Backspace) = key {
            // TODO: Consider just checking for range_start_index exists instead of creating extra flag
            if !has_found_backspace {
                range_start_index = Some(index);
                has_found_backspace = true;
            }
        }

        if value.chars().all(char::is_alphabetic) && has_found_backspace {
            range_end_index = Some(index - 1);
            break;
        }
    }

    if range_start_index.is_none() || range_end_index.is_none() {
        return true;
    }


    // TODO: find out if these fallbacks make sense
    let start = range_start_index.unwrap_or(0);
    let end = range_end_index.unwrap_or(all_key_events.len() - 1);

    //println!(" index:{:?} range: {:?}/{:?} ", key_index,start, end);


    // TODO: Also consider that the previous key could be meta keys or such. skip those and offset them
    let deleted_chars_range: Vec<usize> = (1..=(end + 1 - start)).map(|offset| start - offset).collect();

    // println!("{:?}", deleted_chars_range);

    return !deleted_chars_range.contains(&key_index)
}

fn get_key_events(conn: &mut Client) -> Result<Vec<Row>, PostgresError> {
    return conn.query("SELECT * FROM key_events", &[])
}

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str {
        "Alt" => Some(Key::Alt),
        "AltGr" => Some(Key::AltGr),
        "Backspace" => Some(Key::Backspace),
        "CapsLock" => Some(Key::CapsLock),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "Delete" => Some(Key::Delete),
        "DownArrow" => Some(Key::DownArrow),
        "End" => Some(Key::End),
        "Escape" => Some(Key::Escape),
        "F1" => Some(Key::F1),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "Home" => Some(Key::Home),
        "LeftArrow" => Some(Key::LeftArrow),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Return" => Some(Key::Return),
        "RightArrow" => Some(Key::RightArrow),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "UpArrow" => Some(Key::UpArrow),
        "PrintScreen" => Some(Key::PrintScreen),
        "ScrollLock" => Some(Key::ScrollLock),
        "Pause" => Some(Key::Pause),
        "NumLock" => Some(Key::NumLock),
        "BackQuote" => Some(Key::BackQuote),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        "Num0" => Some(Key::Num0),
        "Minus" => Some(Key::Minus),
        "Equal" => Some(Key::Equal),
        "KeyQ" => Some(Key::KeyQ),
        "KeyW" => Some(Key::KeyW),
        "KeyE" => Some(Key::KeyE),
        "KeyR" => Some(Key::KeyR),
        "KeyT" => Some(Key::KeyT),
        "KeyY" => Some(Key::KeyY),
        "KeyU" => Some(Key::KeyU),
        "KeyI" => Some(Key::KeyI),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "LeftBracket" => Some(Key::LeftBracket),
        "RightBracket" => Some(Key::RightBracket),
        "KeyA" => Some(Key::KeyA),
        "KeyS" => Some(Key::KeyS),
        "KeyD" => Some(Key::KeyD),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "SemiColon" => Some(Key::SemiColon),
        "Quote" => Some(Key::Quote),
        "BackSlash" => Some(Key::BackSlash),
        "IntlBackslash" => Some(Key::IntlBackslash),
        "KeyZ" => Some(Key::KeyZ),
        "KeyX" => Some(Key::KeyX),
        "KeyC" => Some(Key::KeyC),
        "KeyV" => Some(Key::KeyV),
        "KeyB" => Some(Key::KeyB),
        "KeyN" => Some(Key::KeyN),
        "KeyM" => Some(Key::KeyM),
        "Comma" => Some(Key::Comma),
        "Dot" => Some(Key::Dot),
        "Slash" => Some(Key::Slash),
        "Insert" => Some(Key::Insert),
        "KpReturn" => Some(Key::KpReturn),
        "KpMinus" => Some(Key::KpMinus),
        "KpPlus" => Some(Key::KpPlus),
        "KpMultiply" => Some(Key::KpMultiply),
        "KpDivide" => Some(Key::KpDivide),
        "Kp0" => Some(Key::Kp0),
        "Kp1" => Some(Key::Kp1),
        "Kp2" => Some(Key::Kp2),
        "Kp3" => Some(Key::Kp3),
        "Kp4" => Some(Key::Kp4),
        "Kp5" => Some(Key::Kp5),
        "Kp6" => Some(Key::Kp6),
        "Kp7" => Some(Key::Kp7),
        "Kp8" => Some(Key::Kp8),
        "Kp9" => Some(Key::Kp9),
        "KpDelete" => Some(Key::KpDelete),
        "Function" => Some(Key::Function),
        _ => {
            if let Some(code) = key_str.strip_prefix("Unknown(").and_then(|s| s.strip_suffix(")")) {
                code.parse().ok().map(Key::Unknown)
            } else {
                None
            }
        }
    }
}
