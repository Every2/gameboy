use crate::ram::Ram;

pub struct Hardware {
    pub ram: Ram,
}

impl Hardware {
    pub fn new() -> Hardware {
        Hardware {
            ram: Ram::new(),
        }
    }

}