use std::{net::TcpStream, io::{Write},io::{self, BufReader, BufRead}, sync::mpsc::{Sender, Receiver}, time, thread};
use serde_json::*;
use std::str;
use crate::CMD;

pub fn handle_client(mut stream: TcpStream,tx:Sender<CMD>,rx:Receiver<CMD>,index:i32) -> io::Result<()>{
    thread::sleep(time::Duration::from_secs(1));
    let mut buf = Vec::new();
    stream.write(&CMD::new(0,4,index.to_string()).to_string().as_bytes())?;//发送玩家初始化数据
    stream.write("\n".as_bytes())?;
    loop {
        if let Ok(cmd) = rx.recv(){//信息发送
            stream.write(&cmd.to_string().as_bytes())?;
            stream.write("\n".as_bytes())?;
            if cmd.types == -1{
                return  Ok(());
            }
        }
        let mut reader = BufReader::new(&stream);
        if let Ok(bytes_read) = reader.read_until(b'\n',&mut buf){//信息接收
            tx.send(serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap()).unwrap();
            if bytes_read == 0 {
                return Ok(());
            }
        }
    }
}
