mod to_do;
mod state;
mod processes;
use to_do::to_do_factory;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;
use crate::to_do::traits::get::Get;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;
use processes::process_input;


fn main() {
    
    // let to_do_items = to_do_factory("washing", 
    //                                 TaskStatus::DONE);
    // match to_do_items {
    //     ItemTypes::Done(item) => {
    //         item.get(&item.super_struct.title);
    //         item.delete(&item.super_struct.title);
    //     },
    //     ItemTypes::Pending(item) => {
    //         item.get(&item.super_struct.title);
    //         item.set_to_done(&item.super_struct.title);
    //     }
    // }
    let args: Vec<String> = env::args().collect();
    let command:&String=&args[1];
    let title:&String=&args[2];
    let state:Map<String,Value>=read_file("./state.json");
    let status:String;
    match &state.get(*&title){
        
    }
 }
 