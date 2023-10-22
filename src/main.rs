use rdev::{Event, EventType, grab};
use enigo::*;

static mut SPEED: i32 = 20;
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
    //println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyRelease(rdev::Key::ScrollLock) => { 
            unsafe {
                ACTIVE = !ACTIVE;
      //          println!("{}", ACTIVE);
            }
        }
        EventType::KeyPress(k) => unsafe {
            //println!("Event {:?}", k);
            if ACTIVE {
                match k {
                    rdev::Key::Kp1 => { 
                        println!("k1");
                        enigo.mouse_move_relative(-SPEED, 0);
                    }
                    rdev::Key::Kp2 => { 
                        println!("k2");
                        enigo.mouse_move_relative(0, SPEED);
                    }

                    rdev::Key::Kp3 => { 
                        println!("k3");
                        enigo.mouse_move_relative(SPEED, 0);
                    }

                    rdev::Key::Kp5 => { 
                        println!("k5");
                        enigo.mouse_move_relative(0, -SPEED);
                    }

                    rdev::Key::KpPlus => SPEED += 5,
                    rdev::Key::KpMinus => {
                        SPEED -= 5;
                        if SPEED < 0 { SPEED = 0; }
                    }
                    _ => (),
                }
            }
        }
        //EventType::MouseMove => { 
         //   println!(event.x, event.y);
        //}
        _ => (),
    }
    return Some(event)
}
