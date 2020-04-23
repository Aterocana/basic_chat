use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() -> Result<(), std::io::Error> {
    let mut client = TcpStream::connect(LOCAL)?;
    client.set_nonblocking(true)?;

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                match std::str::from_utf8(&msg) {
                    Ok(text) => println!("received: {}", text),
                    Err(_) => (),
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => (),
                _ => {
                    eprintln!("read error, closing connection: {:?}", err);
                    return;
                }
            },
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).unwrap();
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Type a message (:quit to exit), ENTER to send.");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff)?;
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            return Ok(());
        }
    }
}
