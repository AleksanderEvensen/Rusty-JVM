use crate::jvm::traits::JavaClass;

pub struct StringBuilder {}

impl StringBuilder {
    pub fn create_class() -> Box<dyn JavaClass> {
        Box::new(Self {})
    }
}

impl JavaClass for StringBuilder {
    fn execute(&self, ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }
}
