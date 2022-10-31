use crate::{
    java_mappings::java::io::PrintStream::PrintStream,
    jvm::traits::{JavaClass, JavaClassInit, JavaClassInitContext},
};
pub struct System {
    out: PrintStream,
}

impl System {}

impl JavaClassInit for System {
    fn construct(_ctx: JavaClassInitContext) -> Box<dyn JavaClass> {
        Box::new(Self {
            out: PrintStream {},
        })
    }
}

impl JavaClass for System {
    fn execute(&self, _ctx: crate::jvm::traits::JavaClassExecContext) {
        todo!()
    }
    fn get_class_field(&self, field_name: &str) -> Box<&dyn JavaClass> {
        match field_name {
            "out" => Box::new(&self.out),
            _ => panic!("A getter for the field \"{}\" doesn't exist", field_name),
        }
    }
}
