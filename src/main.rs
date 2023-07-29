use slime_chess::{Map, get_player_color};
use console::Term;
use std::io;
fn main() {
    colored::control::set_virtual_terminal(true).unwrap();
    let stdin = io::stdin();
    let term = Term::stdout();

    println!("请输入地图大小:");
    let mut scale = String::new();
    stdin.read_line(&mut scale).unwrap();
    let scale:i32 = scale.trim().parse::<i32>().expect("请输入数字");
    let mut map = Map::new(scale);

    println!("请输入玩家数量:");
    let mut player_count = String::new();
    stdin.read_line(&mut player_count).unwrap();
    let player_count:i32 = player_count.trim().parse::<i32>().expect("请输入数字");
    
    loop{
        for i in 1..(player_count+1){
            term.clear_screen().unwrap();
            print!("{map}");
            loop {
                println!("现在是{}{}号玩家游玩。",i,get_player_color(i));
                println!("输入下棋位置:");
                let mut input = String::new();
                stdin.read_line(&mut input).unwrap();
                let input:Vec<&str> = input.trim().split(" ").collect();
                let mut inputs:Vec<usize> = Vec::new();
                for i in input{
                    inputs.push(i.parse::<usize>().unwrap());
                }
                if inputs.len() > 3{
                    println!("输入参数错误重新输入。")
                }else{
                    if map.place(inputs[0],inputs[1], inputs[2],i) == true{
                        break;
                    }else{
                        println!("此处不可下棋。");
                    }

                }
            }
        }
    }

}
