
struct App {
    services: Vec<Box<dyn Service>>,
}
impl App {
    pub fn new() -> Self {
        Self { services: vec![] }
    }

    pub fn handler<F>(mut self, f: F) -> Self
        where
            F: Service + 'static,
    {
        self.services.push(Box::new(f));
        self
    }

    pub fn dispatch(&self, req: &Request) {
        for f in self.services.iter() {
            f.handle_request(&req);
        }
    }
}

trait Service {
    fn handle_request(&self, req: &Request);
}

impl<F> Service for F
    where
        F: Fn() -> (),
{
    fn handle_request(&self, req: &Request) {
        (self)()
    }
}

/// 请求对象，内部是个字符串
struct Request {
    s: String,
}

impl Request {
    pub fn new(s: impl Into<String>) -> Self {
        Self { s: s.into() }
    }
}

#[test]
fn test_second_try() {
    use mockall::*;

    #[automock]
    pub trait Handler {
        // 测试无参数
        fn f0();
        // 测试获取一个 String
        fn f1(s: String);
        // 测试获取两个参数
        fn f2(n: u32, s: String);
        // 反过来也可以
        fn f3(t: (), s: String, n: u32);
    }

    let f0_ctx = MockHandler::f0_context();
    f0_ctx.expect().times(1).returning(|| {});

    let f1_ctx = MockHandler::f1_context();
    f1_ctx.expect().times(1).returning(|s: String| {
        assert_eq!(s, "123");
    });

    let f2_ctx = MockHandler::f2_context();
    f2_ctx.expect().times(1).returning(|n: u32, s: String| {
        assert_eq!(n, 123);
        assert_eq!(s, "123");
    });

    let f3_ctx = MockHandler::f3_context();
    f3_ctx
        .expect()
        .times(1)
        .returning(|t: (), s: String, n: u32| {
            assert_eq!(s, "123");
            assert_eq!(n, 123);
        });

    let app = App::new()
        .handler(MockHandler::f0)
        .handler(|s: String| MockHandler::f1(s))
        .handler(MockHandler::f2)
        .handler(MockHandler::f3);
    app.dispatch(Request::new("123"));
}