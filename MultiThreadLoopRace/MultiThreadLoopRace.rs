use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::f32;

fn main(){
    println!("Starting tests.");
    //let _join_handle: thread::JoinHandle<_> = thread::spawn(|| forLoopMethod(10000.0));
    let _join_handle: thread::JoinHandle<_> = thread::spawn(|| while_loop_method(10000.0));
    let _join_handle: thread::JoinHandle<_> = thread::spawn(|| break_loop_method(10000.0));
    println!("Ending tests.");
}

// fn forLoopMethod(loops_to_do: f32)-> u64{
//     let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
//     let mut i = 1.0;
//     let mut _sqrt;
//     for i in 1.0..loops_to_do
//     {
//         let sqrt = i.sqrt();
//     }
//     let finish_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
//     let _dif = finish_time - start_time;
//     println!("for loop speed {}", _dif);
//     return _dif;
// }

fn while_loop_method(loops_to_do: f32)-> u64{
    println!("Running while loop.");
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let mut i = 1.0;
    let mut _sqrt;
    while i<= loops_to_do
    {
        _sqrt = i.sqrt();
        i+= 1.0;
    }
    let finish_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let _dif = finish_time - start_time;
    println!("while loop speed {}", _dif);
    return _dif;
}

fn break_loop_method(loops_to_do: f32)-> u64{
    println!("Running break loop.");
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let mut i = 1.0;
    let mut _sqrt;
    loop{
        if i >= loops_to_do
        {
            break;
        }
        _sqrt = i.sqrt();
        i += 1.0;
    }
    let finish_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let _dif = finish_time - start_time;
    println!("break loop speed {}", _dif);
    return _dif;
}
