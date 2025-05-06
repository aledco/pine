use crate::env::Environment;

pub trait Execute {
    fn execute(&self, context: &mut Environment);
}
