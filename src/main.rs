use rdev::{Event, EventType, grab};
use enigo::*;

static SPEED: u8 = 5;
static mut ACTIVE: bool = false;
fn main() {
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> Option<Event> {
    /*println!("My callback {:?}", event);
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }*/
    let mut enigo = Enigo::new();
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyRelease(rdev::Key::ScrollLock) => { 
            unsafe {
                ACTIVE = !ACTIVE;
                println!("{}", ACTIVE);
            }
        }
        EventType::KeyPress(k) => {
            //println!("Event {:?}", k);
            match k {
                rdev::Key::Kp1 => { 
                    println!("k1");
                    enigo.mouse_move_relative(100, 100);
                }
                rdev::Key::Kp2 => println!("k2"),
                rdev::Key::Kp3 => println!("k3"),
                rdev::Key::Kp4 => println!("k4"),
                rdev::Key::Kp5 => println!("k5"),
                rdev::Key::Kp6 => println!("k6"),
                rdev::Key::Kp7 => println!("k7"),
                rdev::Key::Kp8 => println!("k8"),
                rdev::Key::Kp9 => println!("k9"),
                rdev::Key::KpPlus => println!("+"),
                rdev::Key::KpMinus => println!("-"),
                _ => (),
            }
        }
        //EventType::MouseMove => { 
         //   println!(event.x, event.y);
        //}
        _ => (),
    }
    return Some(event)
}
