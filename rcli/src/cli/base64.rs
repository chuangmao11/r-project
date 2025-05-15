use std::{fmt, str::FromStr};
use clap::Parser;
//从父模块导入
use super::{verify_input_file};

//构造命令
#[derive(Debug,Parser)]
pub enum Base64Command{
    #[command(name ="encode",about ="encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name ="decode",about ="decode a base64 string")]
    Decode(Base64DecodeOpts),
}
//构造参数
#[derive(Debug,Parser)]
pub struct Base64EncodeOpts{
    #[arg(short,long,value_parser = verify_input_file,default_value ="-")]
    pub input:String,
    #[arg(short,long,value_parser=parse_base64_format,default_value ="standard")]
    pub format:Base64Format,
}

#[derive(Debug,Parser)]
pub struct Base64DecodeOpts{
    #[arg(short,long,value_parser = verify_input_file,default_value ="-")]
    pub input:String,
    #[arg(short,long,value_parser=parse_base64_format,default_value ="standard")]
    pub format:Base64Format,
}

#[derive(Debug,Parser,Copy)]
pub enum Base64Format{
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error>{
    format.parse()
} 

impl FromStr for Base64Format{
    type Err =anyhow::Error;
    fn from_str(s:&str) -> Result<Self,Self::Err>{
        match s{
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid format: {}",s)),
        }
    }
}

impl From<Base64Format> for &'static str{
    fn from(format: Base64Format) -> Self{
        match format{
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self)
    }
}