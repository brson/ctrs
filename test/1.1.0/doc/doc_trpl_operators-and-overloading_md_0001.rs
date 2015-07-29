fn main() {
    mod foo {

    pub trait Add<RHS = Self> {

        type Output;

    

        fn add(self, rhs: RHS) -> Self::Output;

    }

    }

}
