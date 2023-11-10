use anyhow::Result;
use std::{process, thread};
use tray_item::{IconSource, TrayItem};

enum Message {
    Blue,
    Green,
    Orange,
    Purple,
    Red,
    Quit,
}

const TITLE: &str = "Dygma Layer Switcher";

pub fn load() -> Result<()> {
    let mut tray = TrayItem::new(TITLE, IconSource::Resource("blue"))?;

    tray.add_label(TITLE)?;

    // tray.add_menu_item("Test", || {
    //     println!("Test");
    // })?;

    tray.inner_mut().add_separator()?;

    let (tx, rx) = crossbeam_channel::bounded(1);

    let blue_tx = tx.clone();
    tray.add_menu_item("Blue", move || {
        blue_tx.send(Message::Blue).unwrap();
    })?;

    let green_tx = tx.clone();
    tray.add_menu_item("Green", move || {
        green_tx.send(Message::Green).unwrap();
    })?;

    let orange_tx = tx.clone();
    tray.add_menu_item("Orange", move || {
        orange_tx.send(Message::Orange).unwrap();
    })?;

    let purple_tx = tx.clone();
    tray.add_menu_item("Purple", move || {
        purple_tx.send(Message::Purple).unwrap();
    })?;

    let red_tx = tx.clone();
    tray.add_menu_item("Red", move || {
        red_tx.send(Message::Red).unwrap();
    })?;

    tray.inner_mut().add_separator()?;

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })?;

    thread::spawn(move || loop {
        match rx.recv() {
            Ok(Message::Blue) => tray.set_icon(IconSource::Resource("blue")).unwrap(),
            Ok(Message::Green) => tray.set_icon(IconSource::Resource("green")).unwrap(),
            Ok(Message::Orange) => tray.set_icon(IconSource::Resource("orange")).unwrap(),
            Ok(Message::Purple) => tray.set_icon(IconSource::Resource("purple")).unwrap(),
            Ok(Message::Red) => tray.set_icon(IconSource::Resource("red")).unwrap(),
            Ok(Message::Quit) => process::exit(0),
            _ => {}
        }
    });

    Ok(())
}
