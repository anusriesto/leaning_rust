use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use crate::state::write_to_file;
use super::super::enums::TaskStatus;
pub trait Edit {
    fn set_to_done(&self, title: &str,state:&mut Map<String,Value>) {
        let item:Option<&Value>=state.get(title);
        match item{
        Some(result)=>{
            println!("\n\nItem: {}", title);
            println!("Status: {}\n\n", result);
        },
        None=>  println!("item: {} was not found",title),

    }

        println!("{} is being set to done", title);
    }
    fn set_to_pending(&self, title: &str) {
        println!("{} is being set to pending", title);
    }
}
