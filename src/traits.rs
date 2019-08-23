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
        created_at: u32
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
            created_at: 1566581781
        };
        format!("the article -> {:?} and {}", article, article.get_user())
    }

}