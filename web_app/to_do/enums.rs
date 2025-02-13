use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus{
    pub fn stringify(&self)->String{
        match &self{
            &self::DONE=>{"DONE".to_string()},
            &self::PENDING=>{"PENDING".to_string()}
        }
    }
}

impl fmt::Display for TaskStatus{
    fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result{
        match &self{
            &self::DONE=>{write!(f,"DONE")},
            &self::PENDING=>{write!(f,"PENDING")}
        }
    }
}
