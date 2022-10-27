use crate::jvm::traits::JavaClass;

pub struct StringBuilder {}

impl JavaClass for StringBuilder {
    fn execute(&self, ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }

    fn create_class<T: JavaClass>(ctx: crate::jvm::traits::JavaClassInitContext) -> Box<T> {
        todo!()
    }
}
