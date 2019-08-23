pub mod generic_structs {
    pub struct Colors<T> {
        yellow: T,
        blue: T,
        grey: T,
    }

    pub struct Point<T, U> {
        x: T,
        y: U,
    }

    pub fn render_colors() -> String {
        let colors = Colors {
            yellow: 10,
            blue: 34,
            grey: 51,
        };
        format!(
            "Yellow -> {}, Blue -> {}, Grey -> {}",
            colors.yellow, colors.blue, colors.grey
        )
    }

    pub fn render_hex_colors() -> String {
        let colors = Colors {
            yellow: "#FFEB3B".to_string(),
            blue: "#2196F3".to_string(),
            grey: "#9E9E9E".to_string(),
        };
        format!(
            "Yellow -> {}, Blue -> {}, Grey -> {}",
            colors.yellow, colors.blue, colors.grey
        )
    }

    pub fn render_points(x: u8, y: f32) -> String {
        let points: Point<u8, f32> = Point { x, y };
        format!("Uint8 x -> {}, Float32 y -> {}", points.x, points.y)
    }

    pub fn render_other_points(x: String, y: Vec<u8>) {
        let points: Point<String, Vec<u8>> = Point { x, y };
        println!("String x -> {}, Vector -> {:?}", points.x, points.y)
    }
}

pub mod generic_enums {
    #[derive(Debug)]
    pub struct UsernameAlreadyExist {
        message: String,
        type_error: String,
        severity: u8,
    }

    #[derive(Debug)]
    pub enum OurResult<T, U> {
        Ok(T),
        Error(U),
    }

    pub fn check_username(username: String) -> OurResult<bool, UsernameAlreadyExist> {
        let error = UsernameAlreadyExist {
            message: format!("the username {} is already taken", username),
            type_error: "custom_error".to_string(),
            severity: 3,
        };
        match username.as_str() {
            "john" => OurResult::Error(error),
            _ => OurResult::Ok(true),
        }

        // if username == "john" {
        //     let error = UsernameAlreadyExist {
        //         message: "the username is already taken".to_string(),
        //         type_error: "custom_error".to_string(),
        //         severity: 3,
        //     };
        //     OurResult::Error(error)
        // } else {
        //     OurResult::Ok(true)
        // }
    }
}

pub mod generic_method {
    #[derive(Debug)]
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    pub fn create_point() {
        let p = Point { x: 5, y: 10 };

        println!("the points are {:?} and x is {}", p, p.x());
    }

    // Other example for generic method
    #[derive(Debug)]
    pub struct Rectangle<T, U> {
        width: T,
        height: U,
    }

    impl<T, U> Rectangle<T, U> {
        fn create<V, W>(self, other: Rectangle<V, W>) -> Rectangle<T, W> {
            Rectangle {
                width: self.width,
                height: other.height,
            }
        }
    }

    pub fn create_rectangle(width: u8, height: u8) -> String {
        let rectangle = Rectangle {
            width,
            height,
        };
        let rectangle2 = Rectangle {
            width: 18,
            height: 42,
        };
        let mixed_rectangle = rectangle.create(rectangle2);
        format!("the rectangle mixed is {:?}", mixed_rectangle)
    }
}
