use crate::jvm::traits::{JavaClass, JavaClassExecContext, JavaClassInit, JavaClassInitContext};

pub struct PrintStream {}

impl JavaClassInit for PrintStream {
    fn construct(_ctx: JavaClassInitContext) -> Box<dyn JavaClass> {
        Box::new(Self {})
    }
}

impl JavaClass for PrintStream {
    fn execute(&self, _ctx: JavaClassExecContext) {
        todo!()
    }

    fn get_class_field(&self, _field_name: &str) -> Box<&dyn JavaClass> {
        todo!()
    }
}
