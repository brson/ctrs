fn main() {
    mod foo {

    pub trait Fn<Args> : FnMut<Args> {

        extern "rust-call" fn call(&self, args: Args) -> Self::Output;

    }

    

    pub trait FnMut<Args> : FnOnce<Args> {

        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;

    }

    

    pub trait FnOnce<Args> {

        type Output;

    

        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;

    }

    }

}
