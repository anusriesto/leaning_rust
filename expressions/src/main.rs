use std::io;
//farhenhite to celsius and vice versa
fn main() {
    println!("Conversion karna hai \n press 1 for converting far to cel \n press 2 for converting cel to far");

    let mut command=String::new();

    io::stdin().read_line(&mut command).expect("unable to read lines");
    let command:u32=command.trim().parse().expect("number likho bhai");

    if command==1{
        println!("you are converting farhenhite to celsius");
        println!("input your value");
        let mut value=String::new();
        io::stdin().read_line(&mut value).expect("unable to read lines");
        let value:f32=value.trim().parse().expect("number likho bhai");
        let celsius:f32=(5.0/9.0)*(value-32.0);
        println!("ho gaya convert in celsius {celsius}");

    }
    else {
        println!("you are converting celsius to farhenhite");
        println!("input your value");
        let mut value=String::new();
        io::stdin().read_line(&mut value).expect("unable to read lines");
        let value:f32=value.trim().parse().expect("number likho bhai");
        let far:f32=(9.0/5.0)*value+32.0;
        println!(" ho gaya convert in farhenhite {far} ");
        
    }

    
}