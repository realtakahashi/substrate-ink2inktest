#![cfg_attr(not(feature = "std"), no_std)]

//pub use self::call_flipper::CallFlipper;
use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod call_flipper {
    //use flipper::flipper::Flipper;
    use ink_core::env::call::FromAccountId;
    use ink_core::storage;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct CallFlipper {
        /// Stores a single `bool` value on the storage.
        value: storage::Value<bool>,
        lflipper: storage::Value<flipper::Flipper>,
        //lflipper: flipper::Flipper,
    }

    impl CallFlipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        fn new(&mut self, init_value: bool, flipper: flipper::Flipper) {
            self.value.set(init_value);
            //self.lflipper.set(flipper::Flipper::from_account_id(_flipper));
            //self.lflipper.set(FromAccountId::from_account_id(_flipper));
            //self.lflipper = FromAccountId::from_account_id(_flipper);
            self.lflipper.set(flipper)
        }

        /// call test
        ///
        ///
        #[ink(message)]
        fn call_flip(&mut self) {
            self.lflipper.flip();
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // fn default(&mut self) {
        //     self.new(false)
        // }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // fn flip(&mut self) {
        //     *self.value = !self.get();
        // }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get(&self) -> bool {
            *self.value
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
        #[test]
        fn default_works() {
            // Note that even though we defined our `#[ink(constructor)]`
            // above as `&mut self` functions that return nothing we can call
            // them in test code as if they were normal Rust constructors
            // that take no `self` argument but return `Self`.
            let call_flipper = CallFlipper::default();
            assert_eq!(call_flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut call_flipper = CallFlipper::new(false);
            assert_eq!(call_flipper.get(), false);
            ///call_flipper.flip();
            assert_eq!(call_flipper.get(), true);
        }
    }
}
