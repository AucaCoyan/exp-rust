pub mod lottery;

pub mod prediction {
    #[derive(Debug)]
    pub struct Prediction {
        number: Vec<i8>,
    }

    impl Prediction {
        /// Creates a new [`Prediction`].
        pub fn new(number: Vec<i8>) -> Self {
            Self { number }
        }
    }
}
