use serde::{Deserialize,Serialize};
use csv::Reader;
use std::fs;
use anyhow::Result;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player{
    name:String,
    position:String,
    
    #[serde(rename="DOB")]
    dob:String,
    nationality:String,

    #[serde(rename="Kit Number")]
    kit:u8,
}

pub fn process_csv(input:&str,output:&str) ->Result<()>{
    //创建csv读取器
    let mut reader = Reader::from_path(input)?;
    //创建一个向量来存储结果
    let mut ret = Vec::with_capacity(128);
    //遍历csv中的每一行
    for result in reader.deserialize(){
        let record:Player = result?;
        ret.push(record);
    }

    let mut json = serde_json::to_string_pretty(&ret)?;
    fs::write(output,json)?;
    Ok(())
}