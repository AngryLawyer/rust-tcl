pub struct LinkedVariable<T> {
    value: Box<T>
}

impl <T + Copy> LinkedVariable<T + Copy> {

    pub fn new(boxed: Box<T>) -> LinkedVariable<T> {
        LinkedVariable {
            value: boxed
        }
    }

    pub fn set(&mut self, val: T) {
        *self.value = val;
    }

    pub fn get(&self) -> T {
        (*self.value).clone()
    }
}
