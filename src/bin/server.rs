use std::sync::mpsc::{self,Sender};
use std::{net::TcpListener,io};
use std::{thread, time};
use slime_chess::*;
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

    println!("请输入端口:");
    let mut port = String::new();
    stdin.read_line(&mut port).unwrap();
    let port = port.trim();

    //server init
    let ip = String::from("0.0.0.0:")+port;
    let listener = TcpListener::bind(&ip).unwrap();
    println!("Server open on {}",ip);
    let mut threads:Vec<thread::JoinHandle<()>> = Vec::new();
    let mut rxv:Vec<Sender<CMD>> = Vec::new();//服务端命令发送器vec
    let mut connected_count: i32 = 0; //连接数量
    let (tx,rx) = mpsc::channel::<CMD>();//服务端命令接受器
    listener.set_nonblocking(true).unwrap();

    let mut ifstartonce = true;
    let mut player_now = 1;
    let mut if_next_player = false;
    let mut play_count = 0;
    let mut win = 0;
    loop{
        if player_count <= connected_count{
            if win != 0{
                let cmd1 = CMD::map_cmd(&map);
                let cmd2 = CMD::new(0,-1,win.to_string());
                for i in &rxv{
                    i.send(cmd1.clone()).unwrap();
                    i.send(cmd2.clone()).unwrap();
                }
                println!("Game end.");
                thread::sleep(time::Duration::from_secs(10));
                break;
            }
            if ifstartonce{
                println!("Game start");
                let cmd1 = CMD::new(0,0,String::from(""));//发送开始命令
                let cmd2 = CMD::map_cmd(&map);//发送开始地图初始化
                for i in &rxv{
                    i.send(cmd1.clone()).unwrap();
                    i.send(cmd2.clone()).unwrap();
                }
                if_next_player = true;
                ifstartonce = false;
            }
            if if_next_player{
                for i in&rxv{
                    let cmd1 = CMD::map_cmd(&map); //地图更新
                    let cmd2 = CMD::new(0,1,player_now.to_string()); //发送下棋要求
                    i.send(cmd1.clone()).unwrap();
                    i.send(cmd2.clone()).unwrap();
                }
                if_next_player = false;
            }
            if let Ok(p_cmd) = rx.try_recv(){
                println!("message from {} received",p_cmd.who);
                let (x,y,position) = serde_json::from_str(&p_cmd.content).unwrap();
                map.place(x, y, position, p_cmd.who);
                player_now+=1;
                play_count+=1;
                if play_count >= player_count{
                    win = map.check_win();
                }
                if_next_player = true;
            }
            if player_now > player_count{
                player_now = 1;
            }
        }else{
            let listens = listener.accept();
            if let Ok((s,_)) = listens{
                connected_count+=1;
                let ntx = tx.clone();
                let (ttx,rrx) = mpsc::channel::<CMD>();//客户端线程命令发送器
                rxv.push(ttx);
                threads.push(thread::spawn(move ||{
                    serverlib::handle_client(s,ntx,rrx,connected_count).unwrap();
                }));
                println!("Player Count now:{connected_count}");
            }
        }
    }
    println!("Server end.");
}

