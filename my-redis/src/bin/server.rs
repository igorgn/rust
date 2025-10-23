use std::collections::HashMap;

use bytes::Bytes;
use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame, Result,
};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let db = db.clone();
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(socket, db).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }
}

async fn process(socket: TcpStream, db: Db) -> Result<()> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            _ => panic!("unimplemented"),
        };

        connection.write_frame(&response).await?;

        // let response = Frame::Error("unimplemented".to_string());
        // connection.write_frame(&response).await?;
    }

    Ok(())
}
