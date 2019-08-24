#![allow(dead_code)]
#![allow(unused_variables)]

pub mod trait_example {

    pub trait User {
        fn get_user(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Article {
        id: u32,
        author: String,
        title: String,
        body: String,
        created_at: u32,
    }

    impl User for Article {
        fn get_user(&self) -> String {
            format!("the author is -> {}", &self.author)
        }
    }

    pub fn create_article(title: String, body: String) -> String {
        let article = Article {
            id: 1234,
            author: "john".to_string(),
            title,
            body,
            created_at: 1566581781,
        };
        format!("the article -> {:?} and {}", article, article.get_user())
    }

    pub struct NewsArticle {
        id: u32,
        title: String,
        body: String,
        author: String,
        created_at: u32,
    }

    // Trait Default Implementation
    pub trait Summary {
        fn summarize(&self) -> String {
            self.summarize_article()
        }

        fn summarize_article(&self) -> String;
    }

    impl Summary for NewsArticle {
        fn summarize_article(&self) -> String {
            format!("something, {}", &self.title)
        }
    }

    pub fn hello() {
        let article = NewsArticle {
            id: 1234,
            title: "this is an article".to_string(),
            body: "something".to_string(),
            author: "john".to_string(),
            created_at: 1566650182,
        };

        println!("the title of this article is -> {:?}", article.summarize());
    }

    // Trait Parameters
    // pub fn notify(item: impl Summary) {}

    // Trait Bound Syntax
    // pub fn notify<T: Summary>(item: T) {}

    // pub fn notify(item1: Summary, item2: Summary) {}

    // pub fn notify<T: Summary>(item1: T, item2: T) {}

    // Specifying Multiple Trait Bounds with the + Syntax
    // pub fn notify(item1: impl Summary + std::fmt::Display) {}

    // Clearer Trait Bounds with where Clauses
    // pub fn notify<T: Summary + std::fmt::Display, U: Summary + std::fmt::Write>(item1: T, item2: U) {}
    pub fn notify<T, U>(t: T, u: U) -> ()
    where
        T: Summary + std::fmt::Display,
        U: Summary + std::fmt::Write,
    {

    }

}
