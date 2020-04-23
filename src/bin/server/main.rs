use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc};
use std::thread;

use message::Message;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

#[derive(Debug)]
struct Client {
    socket: TcpStream,
    addr: SocketAddr,
}

fn main() -> Result<(), std::io::Error> {
    let server = TcpListener::bind(LOCAL)?;
    // set to non blocking mode. Every IO operation could return ErrorKind::WouldBlock
    // if the operation has to be retried.
    server.set_nonblocking(true)?;

    // clients collects all connected clients.
    let mut clients: Vec<Client> = vec![];
    // the main loop is basically a select.
    // If a new connection is accepted a new thread is spawn to handle it,
    // otherwise it is checked if there is a message from any client.
    // In each client thread, an arriving message is sent through tx,
    // while it is checked if there is a message in the correponding rx.
    let (tx, rx) = mpsc::channel::<Message>();

    loop {
        // accepting a new connection
        if let Ok((mut socket, addr)) = server.accept() {
            println!("client {} connected", addr);

            let tx = tx.clone();
            match socket.try_clone() {
                Ok(s) => clients.push(Client {
                    socket: s,
                    addr: addr,
                }),
                Err(err) => {
                    println!("failed to clone: {}", err);
                    continue;
                }
            }

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).unwrap();

                        println!("{}: {:?}", addr, msg);
                        let mess = Message::new(MSG_SIZE, Arc::new(msg), addr);
                        // let mess = Message {
                        //     msg: msg,
                        //     sender: addr,
                        // };
                        tx.send(mess).expect("failed to send message");
                    }
                    Err(err) => match err.kind() {
                        ErrorKind::WouldBlock => (),
                        _ => {
                            println!("read error, closing connection with {}: {:?}", addr, err);
                            break;
                        }
                    },
                }

                thread::sleep(std::time::Duration::from_millis(100));
            });
        }

        // handling an arriving message
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                // for each client, check it it's not the sender.
                // If so, send to him the message.
                .filter_map(|mut client| -> Option<Client> {
                    if client.addr != msg.get_sender_addr() {
                        let buff = msg.raw();
                        match client.socket.write_all(&buff) {
                            Ok(_) => (),
                            Err(err) => println!("error: {:?}", err),
                        }
                    }
                    Some(client)
                })
                .collect::<Vec<_>>();
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }
}
