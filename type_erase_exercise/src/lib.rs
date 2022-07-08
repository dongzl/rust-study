
#![allow(unused)]
mod start_simple;
mod second_try;

#[cfg(test)]
mod tests {
    use crate::start_simple::App;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_start_simple() {
        // fn f1() {
        //     println!("f1 called.");
        // }
        // fn f2() {
        //     println!("f2 called.");
        // }
        // fn f3() {
        //     println!("f3 called.");
        // }
        //
        // let app = App::new()
        //     .handler(f1)
        //     .handler(|| println!("lambda called."))
        //     .handler(f2)
        //     .handler(f3);
        // app.dispatch();

        use mockall::*;

        #[automock]
        pub trait Handler {
            fn f1();
            fn f2();
            fn f3();
        }

        let f1_ctx = MockHandler::f1_context();
        f1_ctx.expect().times(1).returning(|| {});

        let f2_ctx = MockHandler::f2_context();
        f2_ctx.expect().times(1).returning(|| {});

        let f3_ctx = MockHandler::f3_context();
        f3_ctx.expect().times(1).returning(|| {});

        let app = App::new()
            .handler(MockHandler::f1)
            .handler(|| MockHandler::f2())
            .handler(MockHandler::f3);
        app.dispatch();
    }
}
