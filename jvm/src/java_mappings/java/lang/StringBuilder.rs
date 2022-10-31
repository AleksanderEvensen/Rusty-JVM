use crate::jvm::traits::{JavaClass, JavaClassExecContext, JavaClassInit, JavaClassInitContext};

pub struct StringBuilder {}

impl JavaClassInit for StringBuilder {
    fn construct(ctx: JavaClassInitContext) -> Box<dyn JavaClass> {
        Box::new(Self {})
    }
}

impl JavaClass for StringBuilder {
    fn execute(&self, ctx: JavaClassExecContext) {
        todo!()
    }

    fn get_class_field(&self, field_name: &str) -> Box<&dyn JavaClass> {
        todo!()
    }
}
