pub struct JavaClassInitContext {}
pub struct JavaClassExecContext {}

pub trait JavaClassInit {
    fn construct(ctx: JavaClassInitContext) -> Box<dyn JavaClass>;
}
pub trait JavaClass {
    fn execute(&self, ctx: JavaClassExecContext);
    fn get_class_field(&self, field_name: &str) -> Box<&dyn JavaClass>;
}
