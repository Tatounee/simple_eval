
pub trait Eval {
    type Output;
    type Err;

    fn eval(&self) -> Result<Self::Output, Self::Err>;
}
