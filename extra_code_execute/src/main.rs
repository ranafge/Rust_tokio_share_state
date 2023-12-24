use std::sync::Mutex;

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();
    *lock +=1;
    println!("{}", *lock);
    do_someting_async().await;
}

async fn do_someting_async() {
    println!("Hello World!");
    return;
}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(2);
    println!("{:?}",increment_and_do_stuff(&mutex).await);
}