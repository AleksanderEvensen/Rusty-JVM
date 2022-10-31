pub struct JavaClassInitContext {}
pub struct JavaClassExecContext {}

pub trait JavaClass {
    fn execute(&self, ctx: JavaClassExecContext);
}
