#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/**
 * flipperコントラクト
 */
#[ink::contract]
mod flipper {

    use ink_prelude::string::String;

    // 使用する変数の定義
    #[derive(Clone)] 
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
        count: u32,
        text: String
    }   

    // 関数の定義
    impl Flipper {
        /**
         * コンストラクター
         */
        #[ink(constructor)]
        pub fn new(init_value: bool, init_count: u32, init_text: String) -> Self {
            Self { 
                value: init_value,
                count: init_count,
                text: init_text
            }
        }

        /**
         * default メソッド
         */
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default(), Default::default())
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
         * 変数を加算させる関数
         */
        #[ink(message)]
        pub fn increment(&mut self) -> u32 {
            self.count += 1;
            self.count
        }

        /**
         * テキストを変更する関数
         */
        #[ink(message)]
        pub fn change_text(&mut self, new_text: String) {
            self.text = new_text;
        }

        ////////////////////////////////////////////////////////////////////////////////////
        // getter method
        ////////////////////////////////////////////////////////////////////////////////////

        /**
         * get メソッド
         */
         #[ink(message)]
         pub fn get(&self) -> bool {
            self.value
         }

        /**
         * 変数を取得する関数
         */
        #[ink(message)]
        pub fn get_number(&self) -> u32 {
            self.count
        }

        /**
         * 文字列を取得する関数
         */
        #[ink(message)]
        pub fn get_text(&self) -> String{
            let a = self.text.clone();
            a
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
            let mut flipper = Flipper::default();
            // check
            assert_eq!(flipper.get(), false);
            assert_eq!(flipper.get_number(), 0);
            assert_eq!(flipper.get_text(), "".to_string());
        }

        /**
         * flip用のテストコード
         */
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false, 0, "".to_string());
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        /**
         * 加算用のテストコード
         */
        #[ink::test]
        fn it_increment() {
            let mut flipper = Flipper::new(false, 0, "".to_string());
            assert_eq!(flipper.get_number(), 0);
            flipper.increment();
            assert_eq!(flipper.get_number(), 1);
        }

        /**
         * 加算用のテストコード 10回
         */
        #[ink::test]
        fn it_increment_10() {
            let mut flipper = Flipper::new(false, 0, "".to_string());
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

        /**
         * 文字列を更新するためのテストコード
         */
        #[ink::test]
        fn it_change_text() {
            let mut flipper = Flipper::new(false, 0, "".to_string());
            assert_eq!(flipper.get_text(), "".to_string());
            // change
            flipper.change_text("test".to_string());
            // check
            assert_eq!(flipper.get_text(), "test".to_string());
        }
    }
}
