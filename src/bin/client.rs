use std::net::TcpStream;
use std::io::{self, Read, BufReader, BufRead};
use console::Term;
use slime_chess::*;
use std::str;

fn main(){
    colored::control::set_virtual_terminal(true).unwrap();
    let term = Term::stdout();
    let stdin = io::stdin();
    let mut map: Map;
    let mut buf = Vec::new();
    let mut player_index:i32;

    let mut ip = String::new();
    println!("输入ip地址:");
    stdin.read_line(&mut ip).unwrap();

    println!("连接服务器中...");
    let mut stream = TcpStream::connect("127.0.0.1:20000").unwrap();
    stream.set_nonblocking(true).unwrap();
    let mut reader = BufReader::new(&stream);
    //let mut  if_player = false;

    loop{
        if let Ok(bytes_read) = reader.read_until(b'\n',&mut buf){
            let cmd: CMD = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            println!("cmd:{:?}",cmd);
                if cmd.types == 4{
                    player_index = cmd.content.parse::<i32>().unwrap();
                    println!("连接服务器成功");
                    println!("您的玩家id为:{player_index}");
                }
                if bytes_read == 0 {
                    return ();
                }
        }
    }
}