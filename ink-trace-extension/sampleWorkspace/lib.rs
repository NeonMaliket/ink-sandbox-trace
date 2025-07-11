#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }


    #[cfg(test)]
    mod drink_tests {
        use std::error::Error;

        use drink::session::{Session, NO_ARGS, NO_ENDOWMENT, NO_SALT};

        #[drink::contract_bundle_provider]
        enum BundleProvider {}

        #[drink::test]
        fn initialization(mut session: Session) -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::local()?;
            let init_value: bool = session
                .deploy_bundle_and(contract, "new", &["true"], NO_SALT, NO_ENDOWMENT)?
                .call_and("get", NO_ARGS, NO_ENDOWMENT)?
                .record()
                .last_call_return_decoded()?
                .expect("Call was successful");

            assert_eq!(init_value, true);

            Ok(())
        }

        #[drink::test]
        fn flipping(mut session: Session) -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::Flipper.bundle()?;
            let init_value: bool = session
                .deploy_bundle_and(contract, "new", &["true"], NO_SALT, NO_ENDOWMENT)?
                .call_and("flip", NO_ARGS, NO_ENDOWMENT)?
                .call_and("flip", NO_ARGS, NO_ENDOWMENT)?
                .call_and("flip", NO_ARGS, NO_ENDOWMENT)?
                .call_and("get", NO_ARGS, NO_ENDOWMENT)?
                .record()
                .last_call_return_decoded()?
                .expect("Call was successful");

            assert_eq!(init_value, false);

            Ok(())
        }
    }
}
