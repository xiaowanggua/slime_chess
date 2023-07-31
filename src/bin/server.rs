use std::{net::TcpListener,io};
use std::thread;
use slime_chess::*;
use slime_chess::serverlib::*;

#[warn(unused_variables)]
fn main(){
    let stdin: io::Stdin = io::stdin();

    println!("请输入地图大小:");
    let mut scale = String::new();
    stdin.read_line(&mut scale).unwrap();
    let scale:i32 = scale.trim().parse::<i32>().expect("请输入数字");
    let mut map = Map::new(scale);

    println!("请输入玩家数量:");
    let mut player_count = String::new();
    stdin.read_line(&mut player_count).unwrap();
    let player_count:i32 = player_count.trim().parse::<i32>().expect("请输入数字");

    //server init
    let listener = TcpListener::bind("0.0.0.0:20000").unwrap();
    let mut threads:Vec<thread::JoinHandle<()>> = Vec::new();
    let mut connect_count: i32 = 0;
    for i in listener.incoming(){
        let s = i.unwrap();
        threads.push(thread::spawn(move ||{
            serverlib::handle_client(s).unwrap();
        }))
    }

}

