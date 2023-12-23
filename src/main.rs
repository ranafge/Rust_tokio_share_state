use std::{sync::{Arc, Mutex}, collections::HashMap};

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]

async fn main() {
    // Bind the listener to the address of redis server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // listener continiusly listening network connection

    println!("\tLisinging\n");

    let db = Arc::new(Mutex::new(HashMap::new()));


    loop {
        // SOCKET IS FRAME FORM NETWORK AND _ IS IP AND PORT , IT'S MEAN NETWORK ADDRESS
        // listener move here in this scope
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone(); // share ownership referece count
        println!("\tAccepted\n");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;


    // byte stream
    let mut connection = Connection::new(socket);
    // byte to frame
    // let mut connection = connection.read_frame().await.unwrap(); // Option<Frame>
    while let  Some(frame)= connection.read_frame().await.unwrap()  {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key())  {
                    Frame::Bulk(value.clone())
                }else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented: {:?}", cmd),
        };
        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
        
    } 
}

