#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/**
 * flipperコントラクト
 */
#[ink::contract]
mod flipper {

    // 使用する変数の定義
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
        count: u32
    }   

    // 関数の定義
    impl Flipper {
        /**
         * コンストラクター
         */
        #[ink(constructor)]
        pub fn new(init_value: bool, init_count: u32) -> Self {
            Self { 
                value: init_value,
                count: init_count
            }
        }

        /**
         * default メソッド
         */
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        /**
         * flipメソッド
         */
        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
            self.value
        }

        /**
         * get メソッド
         */
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /**
         * 変数を加算させる関数
         */
        #[ink(message)]
        pub fn increment(&mut self) -> u32 {
            self.count += 1;
            self.count
        }

        /**
         * 変数を取得する関数
         */
        #[ink(message)]
        pub fn get_number(&self) -> u32 {
            self.count
        }
    }

    /**
     * 静的テストコード
     */
    #[cfg(test)]
    mod tests {
       
        use super::*;
        use ink_lang as ink;

        /**
         * デフォルト値のテストコード
         */
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            // check
            assert_eq!(flipper.get(), false);
            assert_eq!(flipper.get_number(), 0);
        }

        /**
         * flip用のテストコード
         */
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false, 0);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        /**
         * 加算用のテストコード
         */
        #[ink::test]
        fn it_increment() {
            let mut flipper = Flipper::new(false, 0);
            assert_eq!(flipper.get_number(), 0);
            flipper.increment();
            assert_eq!(flipper.get_number(), 1);
        }

        /**
         * 加算用のテストコード 10回
         */
        #[ink::test]
        fn it_increment_10() {
            let mut flipper = Flipper::new(false, 0);
            assert_eq!(flipper.get_number(), 0);

            // 10回加算
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            flipper.increment();
            
            assert_eq!(flipper.get_number(), 10);
        }
    }
}
