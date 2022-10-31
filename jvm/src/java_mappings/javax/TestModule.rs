use crate::jvm::traits::JavaClass;

pub struct TestModule;

impl TestModule {
    pub fn create_class() -> Box<dyn JavaClass> {
        Box::new(Self {})
    }
}

impl JavaClass for TestModule {
    fn execute(&self, ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }
}
