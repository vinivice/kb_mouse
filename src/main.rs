use rdev::{Event, EventType, grab};
use enigo::*;

static mut SPEED: i32 = 20;
static mut ACTIVE: bool = false;
fn main() {
    if let Err(_) = grab(callback) {
      //  println!("Error: {:?}", error)
        std::process::exit(exitcode::SOFTWARE);
    }
}

fn callback(event: Event) -> Option<Event> {
    let mut enigo = Enigo::new();
    match event.event_type {
        EventType::KeyRelease(rdev::Key::ScrollLock) => { 
            unsafe {
                ACTIVE = !ACTIVE;
            }
        }
        EventType::KeyPress(k) => unsafe {
            if ACTIVE {
                match k {
                    rdev::Key::Kp1 => { 
                        enigo.mouse_move_relative(-SPEED, 0);
                        return None;
                    }
                    rdev::Key::Kp2 => { 
                        enigo.mouse_move_relative(0, SPEED);
                        return None;
                    }

                    rdev::Key::Kp3 => { 
                        enigo.mouse_move_relative(SPEED, 0);
                        return None;
                    }

                    rdev::Key::Kp4 => { 
                        enigo.mouse_click(MouseButton::Left);
                        return None;
                    }

                    rdev::Key::Kp5 => { 
                        enigo.mouse_move_relative(0, -SPEED);
                        return None;
                    }
                    rdev::Key::Kp6 => { 
                        enigo.mouse_click(MouseButton::Right);
                        return None;
                    }

                    rdev::Key::KpPlus => {
                        SPEED += 5;
                        return None;
                    }
                    rdev::Key::KpMinus => {
                        SPEED -= 5;
                        if SPEED < 0 { SPEED = 0; }
                        return None;
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
    return Some(event)
}
