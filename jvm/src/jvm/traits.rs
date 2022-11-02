use crate::utils::{DescriptorResult, DescriptorTypes, DescriptorValues};

pub struct JavaClassInitContext {
    descriptor: DescriptorResult,
    values: Vec<DescriptorValues>,
}

impl JavaClassInitContext {
    pub fn empty() -> Self {
        Self {
            descriptor: DescriptorResult {
                return_value: DescriptorTypes::Void,
                parameters: vec![],
            },
            values: vec![],
        }
    }
}

pub struct JavaClassExecContext {}

pub trait JavaClassInit {
    fn construct(ctx: JavaClassInitContext) -> Box<dyn JavaClass>;
}
pub trait JavaClass {
    fn execute(&self, ctx: JavaClassExecContext);
    fn get_class_field(&self, field_name: &str) -> Box<&dyn JavaClass>;
}
