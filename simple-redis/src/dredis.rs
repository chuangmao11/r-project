use anyhow::Result;
use tokio::{net::TcpListener,io::AsyncWriteExt};
use tracing::{info,warn};
use std::{io,net::SocketAddr};

const BUF_SIZE:usize =4096;

#[tokio::main]
async fn main() ->Result<()>{
    tracing_subscriber::fmt::init();
    let addr ="0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}",addr);

    loop{
        let (stream,raddr) = listener.accept().await?;
        info!("Accepted connection from: {}",raddr);
        tokio::spawn(async move{
            let raddr_clone = raddr.clone();
            if let Err(e) =process_redis_conn(stream, raddr_clone).await{
                warn!("Error processing connection: {}:{:?}",raddr,e);
            }
        });
    }
}

async fn process_redis_conn(mut stream:tokio::net::TcpStream, raddr:SocketAddr)->Result<()>{
    loop{
        stream.readable().await?;
        let mut buf =Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf){
            Ok(0) =>break,
            Ok(n) =>{
                info!("read {} bytes",n);
                let line =String::from_utf8_lossy(&buf);
                info!("{:?}",line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", raddr);
    Ok(())
}
