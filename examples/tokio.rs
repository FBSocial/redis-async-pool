use std::error::Error;
use redis_cluster_async::{ Connection, Client };
use redis_async_pool::{RedisConnectionManager, RedisPool};
use redis_cluster_async::redis::{AsyncCommands, ConnectionLike};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // Create a pool of maximum 5 connections, checked on reuse without ttl.
    let mut client = Client::open(vec!["redis://10.100.1.36:6380","redis://10.100.1.36:6381","redis://10.100.1.36:6382"])?;
    client.set_password("idreamsky@123");
    let pool = RedisPool::new(RedisConnectionManager::new(client, true, None), 5, );

    // get a connection with the get() async method and use it as regular redis connection
    let mut con = pool.get().await?;
    con.set(b"key", b"value").await?;
    let value: Vec<u8> = con.get(b"key").await?;
    assert_eq!(value, b"value");

    Ok(())
}
