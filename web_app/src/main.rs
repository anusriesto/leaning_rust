mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;


pub fn main(){
    let done=Done::new("Shopping");
    println!("{}",done.super_struct.title);
    println!("{}",done.super_struct.status.stringify());
    let pending=Pending::new("laundry");
    println!("{}",pending.super_struct.title);
    println!("{}",pending.super_struct.status.stringify());


}