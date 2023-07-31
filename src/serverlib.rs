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
        if let Ok(cmd) = rx.try_recv(){//信息发送
            println!("Sended Message {:?}.",cmd);
            stream.write(&cmd.to_string().as_bytes())?;
            stream.write("\n".as_bytes())?;
            thread::sleep(time::Duration::from_secs(1)/2);
            if cmd.types == -1{
                break;
            }
        }
        let mut reader = BufReader::new(&stream);
        if let Ok(_) = reader.read_until(b'\n',&mut buf){//信息接收
            println!("Received Message.");
            tx.send(serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap()).unwrap();
            buf = Vec::new();
        }
    }
    Ok(())
}
