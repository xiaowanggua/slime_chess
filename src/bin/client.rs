use slime_chess::*;
use std::str;
fn main(){
    let map = Map::new(9);
    let n = str::from_utf8(serde_json::to_string(&map).unwrap().as_bytes()).unwrap().to_owned().len();
    //let nm:Map = serde_json::from_str(&n).unwrap();
    println!("{n}");
}