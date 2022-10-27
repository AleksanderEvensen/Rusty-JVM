pub struct JavaClassInitContext {}
pub struct JavaClassExecContext {}

pub trait JavaClass {
    fn execute(&self, ctx: JavaClassExecContext);
    fn create_class<T: JavaClass>(ctx: JavaClassInitContext) -> Box<T>;
}
