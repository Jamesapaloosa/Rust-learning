use std::time::{SystemTime, UNIX_EPOCH};
fn main(){
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let time_of_day = current_time%86400;
    let _morning=if time_of_day<44200
    {
        true
    }
    else
    {
        false
    };
    println!("is it morining? {}", _morning);
}
