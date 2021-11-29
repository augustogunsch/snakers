use std::io;
use std::thread;
use std::sync::mpsc;
use termion::input::TermRead;
use termion::event::Key;

pub struct Events {
    rx: mpsc::Receiver<Key>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();

            for key in stdin.keys() {
                match key {
                    Ok(key) => if let Err(_) = tx.send(key) { return; },
                    Err(_) => panic!("Failed to read stdin"),
                }
            }
        });

        Events {
            rx,
        }
    }

    pub fn next(&self) -> Option<Key> {
        let key = self.rx.try_recv();
        match key {
            Ok(key) => Some(key),
            Err(err) => match err {
                mpsc::TryRecvError::Empty => None,
                _ => panic!("Channel closed")
            },
        }
    }

    pub fn last(&self) -> Option<Key> {
        let mut key = self.next();
        while let Some(k) = self.next() {
            key = Some(k);
        }
        key
    }
}
