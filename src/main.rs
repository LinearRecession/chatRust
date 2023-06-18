use tokio::{net::{TcpListener}, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, sync::broadcast};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
let listener = TcpListener::bind("localhost:8080").await.unwrap();
//let (sndr, _rcvr )= broadcast::channel(10);
let (sndr, _rcvr) = broadcast::channel::<(String, SocketAddr)>(10);

loop {
let (mut socket, addr) = listener.accept().await.unwrap();
let sndr = sndr.clone();
let mut rcvr = sndr.subscribe();
tokio::spawn(async move{ 
let (reader, mut writer) = socket.split();
let mut reader: BufReader<tokio::net::tcp::ReadHalf<'_>> = BufReader::new(reader);
let mut line = String::new();
loop{
    tokio::select! {
    result = reader.read_line(&mut line) =>{ 
        if result.unwrap() == 0 {break;}
    
        sndr.send(line.clone(),addr).unwrap();
        line.clear();
    }

    result = rcvr.recv() => { 

    let (msg, other_addr) = result.unwrap();
    if addr != other_addr {writer.write_all(msg.as_bytes()).await.unwrap();}
                }
            }
        }
    });
  }
}

