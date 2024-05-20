// use std::ops::Add;
//
// // 为Point结构体派生Debug特征，用于格式化输出
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
//     x: T,
//     y: T,
// }
//
// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;
//
//     fn add(self, p: Point<T>) -> Point<T> {
//         Point{
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }
//
// fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
//     a + b
// }
//
// trait Base {}
//
// trait Derived : Base {}
//
// struct T;
//
// impl Derived for T {
//
// }
//
// impl Base for T {
//
// }
//
// fn main() {
//     // pub trait Summary {
//     //     fn summarize(&self) -> String;
//     // }
//     // pub struct Post {
//     //     pub title: String, // 标题
//     //     pub author: String, // 作者
//     //     pub content: String, // 内容
//     // }
//     //
//     // impl Summary for Post {
//     //     fn summarize(&self) -> String {
//     //         format!("文章{}, 作者是{}", self.title, self.author)
//     //     }
//     // }
//     //
//     // pub struct Weibo {
//     //     pub username: String,
//     //     pub content: String
//     // }
//     //
//     // impl Summary for Weibo {
//     //     fn summarize(&self) -> String {
//     //         format!("{}发表了微博{}", self.username, self.content)
//     //     }
//     // }
//     //
//     // let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
//     // let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
//     //
//     // println!("{}",post.summarize());
//     // println!("{}",weibo.summarize());
//
//     // let p1 = Point{x: 1.1f32, y: 1.1f32};
//     // let p2 = Point{x: 2.1f32, y: 2.1f32};
//     // println!("{:?}", add(p1, p2));
//     //
//     // let p3 = Point{x: 1i32, y: 1i32};
//     // let p4 = Point{x: 2i32, y: 2i32};
//     // println!("{:?}", add(p3, p4));
// }

pub trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {


    fn summarize(&self) -> String {
        format!("summarize======{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("summarize_author======={}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("summarize======={}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("summarize_author=========={}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable_1() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        let na  = NewsArticle {
            headline: String::from("headline"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("content"),
        };
        Box::new(na)
    } else {
        let t = Tweet {
            username: String::from("username"),
            content: String::from("content"),
            reply: true,
            retweet: true,
        };
        Box::new(t)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize_author());
}