use std::fmt::Display;
use std::usize;
use std::fmt;
use colored::ColoredString;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
pub mod serverlib;
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk{
    pub points : [i32;4]
}

impl Chunk{
    pub fn new(types:i32,point_count:usize)->Self{
        Chunk{
            points :{
                let mut temp = [-1,-1,-1,-1];
                for i in types..=(point_count as i32 + types - 1){
                    if i >= 4{
                        temp[i as usize - 4] = 0;
                    }else{
                        temp[i as usize] = 0;
                    }
                }
                temp
            }
        }
    }
    pub fn place(&mut self,player_index:i32,position:usize)->i32{
        for i in 0..3{
            if self.points[i] > 0 && self.points[i] !=player_index{
                return 0;
            }
        }
        if self.points[position] != 0{
            return 0;
        }
        self.points[position] = player_index;
        1
    }
    pub fn fill(&mut self,player_index:i32){
        for i in 0..3{
            if self.points[i] == 0{
                self.points[i] = player_index;
                break;
            };
        }
        for i in 0..3{
            if self.points[i] > 0{
                self.points[i] = player_index;
            }
        }
    }
    pub fn is_full(&self)->bool{
        for i in self.points{
            if i==0{
                return false;
            }
        }
        true
    }
    pub fn clear(&mut self){
        for i in 0..4{
            if self.points[i] != -1{
                self.points[i] = 0;
            }
        }
    }
    pub fn color_char(&self,index:usize)->ColoredString{
        if self.points[index] == -1{
            return "■".white();
        }else if self.points[index] == 0{
            return "□".white();
        }else if self.points[index] == 1{
            return "■".red();
        }else if self.points[index] == 2{
            return "■".blue();
        }else if self.points[index] == 3{
            return "■".green();
        }else if self.points[index] == 4{
            return "■".yellow();
        }else{
            return "■".black();
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map{
    pub scale : i32,
    pub map_data : Vec<Vec<Chunk>>,
}

impl Map{
    pub fn new(scale:i32)->Self{
        let mut map_data:Vec<Vec<Chunk>> = Vec::new();

        for i in 0..scale{
            let mut temp:Vec<Chunk> = Vec::new();
            for  j in 0..scale{
                if i == 0 && j==0{
                    temp.push(Chunk::new(1,2));
                }else if i == 0 && j == scale-1{
                    temp.push(Chunk::new(2,2));
                }else if j == scale-1 && i == scale-1{
                    temp.push(Chunk::new(3,2));
                }else if j == 0 && i == scale-1{
                    temp.push(Chunk::new(0,2));
                }else if i == 0{
                    temp.push(Chunk::new(1,3));
                }else if i == scale-1{
                    temp.push(Chunk::new(3,3));
                }else if j == 0{
                    temp.push(Chunk::new(0,3));
                }else if j == scale-1{
                    temp.push(Chunk::new(2,3));
                }else{
                    temp.push(Chunk::new(0,4));
                }
            }
            map_data.push(temp);
        }
        Map {
            scale,
            map_data
        }
    }
    pub fn place(&mut self,x:usize,y:usize,position:usize,player_index:i32)->bool{
        if x as i32 > self.scale -1 || y as i32 > self.scale - 1{
            return false;
        }
        let chunk: &mut Chunk = &mut self.map_data[x][y];
        if chunk.place(player_index,position) == 0{
            return false;
        }
        self.boom(x,y,player_index);
        true
    }

    pub fn boom(&mut self,x:usize,y:usize,player_index:i32){
        let chunk: &mut Chunk = &mut self.map_data[x][y];
        if chunk.is_full(){
            chunk.clear();
            if self.check_chunk_exist(x, y+1){
                self.map_data[x][y+1].fill(player_index);
                self.boom(x, y+1, player_index);
            }
            if self.check_chunk_exist(x, y-1){
                self.map_data[x][y-1].fill(player_index);
                self.boom(x, y-1, player_index);
            }
            if self.check_chunk_exist(x+1, y){
                self.map_data[x+1][y].fill(player_index);
                self.boom(x+1, y, player_index);
            }
            if self.check_chunk_exist(x-1, y){
                self.map_data[x-1][y].fill(player_index);
                self.boom(x-1, y, player_index);
            }
        }
    }
    fn check_chunk_exist(&self,x:usize,y:usize)->bool{
        if let None = self.map_data.get(x){
            false
        }else{
            if let None = self.map_data[x].get(y){
                false
            }else{
                true
            }
        }
    }
    /*fn check_win(&self){

    }*/
}


impl Display for Map{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp = String::from(" ");
        for i in 0..self.scale{
            temp+=&format!("   —{}—",i);
        }
        temp+="\n";

        for i in 0..self.scale{
            for _ in 0..self.scale{
                temp+="     ";
            }
            temp+="\n| ";
            for j in 0..self.scale{
                temp+=&format!(" ■{}■ ",self.map_data[i as usize][j as usize].color_char(0));
            }
            temp+="\n";
            temp+=&i.to_string();
            temp+=" ";
            for j in 0..self.scale{
                temp+=&format!(" {}■{} ",self.map_data[i as usize][j as usize].color_char(3)
                ,self.map_data[i as usize][j as usize].color_char(1));
            }
            temp+="\n| ";
            for j in 0..self.scale{
                temp+=&format!( " ■{}■ ",self.map_data[i as usize][j as usize].color_char(2));
            }
            temp+="\n|";
            for _ in 0..self.scale{
                temp+="     ";
            }
        }
        write!(f,
        "{}",temp
        )
    }
}

pub fn get_player_color(index:i32)->ColoredString{
    if index == 1{
        return "■".red();
    }else if index == 2{
        return "■".blue();
    }else if index == 3{
        return "■".green();
    }else if index == 4{
        return "■".yellow();
    }else{
        return "■".black();
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CMD{
    pub who:i32,// 0为服务端
    pub types:i32,
    /*
    -1游戏结束
    0开始游戏
    1为服务端发送下棋要求
    2为地图内容
    3为客户端下棋内容
    4为服务端发送客户端信息
     */
    pub content:String
}
impl CMD {
    pub fn new(who:i32,types:i32,content:String)->Self{
        CMD{
            who,types,content
        }
    }
    pub fn map_cmd(content:&Map)->Self{
        CMD{
            who:0,
            types:2,
            content: serde_json::to_string(content).unwrap()
        }
    }
    pub fn to_string(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}