use std::net::TcpStream;
use std::io::{self, Read, BufReader, BufRead, Write};
use console::Term;
use slime_chess::*;
use std::str;

fn main(){
    colored::control::set_virtual_terminal(true).unwrap();
    let term = Term::stdout();
    let stdin = io::stdin();
    let mut map: Map = Map::new(0);
    let mut buf = Vec::new();
    let mut player_index:i32 = 0;

    let mut ip = String::new();
    println!("输入ip地址:");
    stdin.read_line(&mut ip).unwrap();

    println!("连接服务器中...");
    let mut stream = TcpStream::connect(ip.trim()).unwrap();
    stream.set_nonblocking(true).unwrap();
    let mut if_player = false;

    loop{
        if if_player{
            loop{
                println!("输入下棋位置:");
                let mut input = String::new();
                stdin.read_line(&mut input).unwrap();
                let input:Vec<&str> = input.trim().split(" ").collect();
                let mut inputs:Vec<usize> = Vec::new();
                for i in input{
                    inputs.push(i.parse::<usize>().unwrap());
                }
                if inputs.len() != 3{
                    println!("输入参数错误重新输入。")
                }else{
                    if map.place(inputs[0],inputs[1], inputs[2],player_index){
                        println!("下棋成功！");
                        stream.write(CMD::new(player_index,3,serde_json::to_string(&(inputs[0],inputs[1],inputs[2])).unwrap()).to_string().as_bytes()).unwrap();
                        stream.write("\n".as_bytes()).unwrap();
                        if_player = false;
                        break;
                    }else{
                        println!("此处不可下棋。");
                    }
                }
            }
        }
        let mut reader = BufReader::new(&stream);
        if let Ok(bytes_read) = reader.read_until(b'\n',&mut buf){
            let cmd: CMD = serde_json::from_str(str::from_utf8(&buf).unwrap()).unwrap();
            //println!("cmd:{:?}",cmd);
            if cmd.types == 0{
                println!("全部玩家已到齐游戏开始！");
            }else if cmd.types == 1{
                if cmd.content == player_index.to_string(){
                    println!("轮到你下棋了！{player_index}{}号玩家。",get_player_color(player_index));
                    if_player = true;
                }else{
                    let id = cmd.content.parse::<i32>().unwrap();
                    println!("轮到{id}{}号玩家下棋了。",get_player_color(id));
                    println!("请等待....");
                }
            }else if cmd.types == 2{
                map = serde_json::from_str(&cmd.content).unwrap();
                term.clear_screen().unwrap();
                println!("{map}");
            }
            else if cmd.types == 4{
                player_index = cmd.content.parse::<i32>().unwrap();
                println!("连接服务器成功");
                println!("您的玩家id为:{player_index}");
                println!("请等待所有玩家连接");
            }
            buf = Vec::new();
        }
    }
}