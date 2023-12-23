// use std::sync::Mutex;

// struct CanIncrement {
//     mutex: Mutex<i32>
// }

// impl CanIncrement {
//     fn increment(&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock +=1;
//     }
// }

// async fn increment_and_do_stuff(can_incr: &CanIncrement){
//     can_incr.increment();
//     do_something_async().await;
// }

// use tokio::sync::Mutex;

// async fn increment_and_to_stuff(mutex: &Mutex<i32>) {
//     let mut lock = mutex.lock().unwrap();
//     *lock +=1;
//     // do_something_async().await;
// }