pub struct MockInterpreter {
    pub code: Vec<MockInstruction>,
    pub pc: usize,
}

impl MockInterpreter {
    pub fn new(code: Vec<MockInstruction>) -> Self {
        Self { code, pc: 0 }
    }

    pub fn next(&mut self) {
        println!("next(), pc: {}", self.pc);

        self.pc += 1;
    }

    pub fn next_with_custom_handler<T>(&mut self, handler: CustomHandler<T>) -> T {
        println!("next_with_custom_handler(), pc: {}", self.pc);

        handler.0(self)
    }
}

pub struct CustomHandler<T>(Box<dyn FnOnce(&mut MockInterpreter) -> T>);

impl<T> CustomHandler<T> {
    pub fn new(handler: Box<dyn FnOnce(&mut MockInterpreter) -> T>) -> Self {
        Self(handler)
    }
}

pub enum MockInstruction {
    ShowText(usize, String),
    DeleteText(usize),
}
