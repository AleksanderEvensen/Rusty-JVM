use crate::jvm::traits::JavaClass;

pub struct PrintStream {}

impl JavaClass for PrintStream {
    fn execute(&self, ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }

    fn create_class<T: JavaClass>(ctx: crate::jvm::traits::JavaClassInitContext) -> Box<T> {
        todo!()
    }
}
