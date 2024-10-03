enum Register{
    A,B,C,M,SP,PC,BP,FLAGS,
}

trait Addressable{
    fn read(addr:u16)->Option<u8>;
    fn write(addr:u16,value:u8)->bool;
    fn read2(addr:u16)->Option<u16>

}

struct Machine{
    registers:[u16;7],
    memory:[u8;5000],

}

impl Machine{
    pub fn new()->Self{
        Self{
            registers:[0;8],
            memory:[0;5000]
        }
    }
    pub fn step(&mut self)->Result<(),&'static str>{
        let pc =self.registers[Register::PC];
        self.memory.read
    }
}