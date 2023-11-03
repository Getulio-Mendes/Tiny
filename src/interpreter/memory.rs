use std::collections::HashMap;

pub fn write(map: &mut HashMap::<String,i32>,name: &String,data: i32){
  map.insert(name.clone(),data);
}

pub fn read(map: &HashMap::<String,i32>,name: &String) -> i32{
  map.get(name).expect("Unknow var").clone()
}