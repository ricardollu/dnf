use std::{
    env,
    thread::{self},
};

use enigo::{
    Button, Coordinate,
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use inputbot::KeybdKey::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let number: u32 = args[1].parse().unwrap();

    EndKey.bind(|| {
        let enigo = Enigo::new(&Settings::default()).unwrap();
        println!("pos: {:?}@{:?}", enigo.location(), enigo.main_display());
    });

    PageUpKey.bind(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        for i in 1..number + 1 {
            open_box(&mut enigo);
            if i == number || i % 24 == 0 {
                decompose(&mut enigo);
            }
        }
    });

    inputbot::handle_input_events();
    Ok(())
}

fn decompose(enigo: &mut Enigo) {
    let timeout = 200;
    key_click(Key::Tab, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(972, 593, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    // 0..3
    for i in 0..3 {
        enigo
            .move_mouse(1146, 541 + 48 * i, Coordinate::Abs)
            .unwrap();
        mouse_click(Button::Left, enigo);
        thread::sleep(std::time::Duration::from_millis(timeout));
        for _ in 0..7 {
            enigo.move_mouse(48, 0, Coordinate::Rel).unwrap();
            mouse_click(Button::Left, enigo);
            thread::sleep(std::time::Duration::from_millis(timeout));
        }
    }

    // 分解
    enigo.move_mouse(849, 722, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));

    // 我已确认
    enigo.move_mouse(732, 631, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));

    // 确定
    enigo.move_mouse(748, 738, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(3000)); // 等待分解完成

    // 关闭
    key_click(Key::Escape, enigo);
}

fn open_box(enigo: &mut Enigo) {
    let timeout = 300;
    enigo.move_mouse(274, 1040, Coordinate::Abs).unwrap();
    mouse_click(Button::Right, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(914, 280, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(821, 313, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(861, 557, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(759, 778, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(916, 632, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(916, 632, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
    enigo.move_mouse(916, 632, Coordinate::Abs).unwrap();
    mouse_click(Button::Left, enigo);
    thread::sleep(std::time::Duration::from_millis(timeout));
}

fn mouse_click(button: Button, enigo: &mut Enigo) {
    enigo.button(button, Press).unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    enigo.button(button, Release).unwrap();
}

fn key_click(key: Key, enigo: &mut Enigo) {
    enigo.key(key, Press).unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    enigo.key(key, Release).unwrap();
}
