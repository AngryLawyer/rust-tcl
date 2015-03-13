pub struct LinkedVariable {
    value: Box<i32>
}

impl LinkedVariable {

    pub fn new(boxed: Box<i32>) -> LinkedVariable {
        LinkedVariable {
            value: boxed
        }
    }

    pub fn set(&mut self, val: i32) {
        *self.value = val;
    }

    pub fn get(&self) -> i32 {
        *self.value
    }
}
