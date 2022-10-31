use crate::jvm::traits::JavaClass;

pub struct PrintStream {}

impl PrintStream {
    pub fn println() {}
    pub fn create_class() -> Box<dyn JavaClass> {
        Box::new(Self {})
    }
}

impl JavaClass for PrintStream {
    fn execute(&self, ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }
}
