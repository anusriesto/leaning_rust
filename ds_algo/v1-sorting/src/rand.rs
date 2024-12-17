use std::sync::Mutex;
lazy_static::lazy_static!{
    static ref RG: Mutex<RanGen>=Mutex::new(RanGen::new(524456));
}

pub fn rand(max:usize)->usize{
    RG.lock().unwrap().next_v(max)
}

pub struct RanGen{
    curr:usize,
    mul:usize,
    inc:usize,
    modulo:usize
  
}

impl RanGen{
    pub fn new(curr:usize)->Self{
        RanGen{
            curr,
            mul:5637825,
            inc:9842322218,
            modulo:584231789,
        }
    }

    pub fn next_v(&mut self,max:usize)-> usize{
        self.curr=(self.curr*self.mul+self.inc)%self.modulo;
        self.curr %max
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use super::RanGen;

    #[test]
    fn test_rand_print(){
        let mut r=RanGen::new(12);
        for _ in 0..100{
            println!("--{}",r.next_v(100));

        }
        panic!();
    }

}