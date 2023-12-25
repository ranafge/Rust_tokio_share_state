use std::sync::Mutex;
use std::collections::HashMap;
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();
    *lock +=1;

    tokio::spawn(async move{
        do_someting_async().await;
    });
}

async fn do_someting_async() {
    println!("Hello World!");
    return;
}


// as we know server connection receive frame continuously . in our case as each key is independent mutext sharding 
// sharding will work well

type  ShardedDb = std::sync::Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;
// new_sharded_db function created mutex HashMap instance distinct amount of Shareddb type.
async fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards{
        db.push(Mutex::new(HashMap::new()));
    }
    std::sync::Arc::new(db)
}


struct CanIncrement {
    mutex: Mutex<i32>
}

impl CanIncrement {
    fn increment(&self) -> i32 {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;

        *lock
    }
}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(2);
    println!("{:?}",increment_and_do_stuff(&mutex).await);
   let result =  new_sharded_db(10).await;
   println!("{:?}", result);

   let c = CanIncrement {
    mutex: Mutex::new(3),
   };
   let res = c.increment();
   println!("{:?}", res);
}
