pub mod custom_structs {
    #[derive(Debug)]
    pub struct User {
        username: String,
        email: String,
        password: String,
        age: u8,
        active: bool,
    }
    pub fn create_user(username: String, password: String, email: String) -> User {
        User {
            username,
            email,
            password,
            age: 22,
            active: true,
        }
    }

    #[derive(Debug)]
    pub struct Author {
        id: u32,
        username: String,
        age: u8,
    }

    #[derive(Debug)]
    pub struct Message {
        id: u32,
        message: String,
        author: Author,
    }

    pub fn send_message() -> Message {
        let sender = Author {
            id: 19911,
            username: String::from("John"),
            age: 22,
        };

        Message {
            id: 2341119,
            message: String::from("this is a message .."),
            author: sender,
        }
    }

}
