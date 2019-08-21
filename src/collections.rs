pub mod vectors {
    pub fn list() {
        let v = vec![1, 2, 3, 4];
        let mut v2: Vec<u8> = Vec::new();
        v2.push(1);
        v2.push(2);
        v2.push(3);
        v2.push(4);

        for i in v2 {
            println!("element {}", i);
        }

        match v.get(2) {
            Some(n) => println!("Element selected is {}", n),
            None => println!("There is no elements in the vector"),
        };
    }

    pub fn multiple_types() {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        };

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(1.2345611),
            SpreadsheetCell::Text(String::from("hello world")),
        ];

        println!("row -> {:?}", row);
    }
}

pub mod strings {
    pub fn general_string() {
        let s = String::from("this is a string");
        let data = "this is another string";
        data.to_string();
        let _j = "the method works on a literal directly".to_string();

        let mut p = String::from("foo");
        p.push_str("bar");
        p.push_str(&s);
        p.push_str(data);

        let one = String::from("yep");
        let two = s + &one;
        println!("{}", one);
        format!("{}-{}", two, p);

        // slices strings
        let o = String::from("there is a blue sky");
        println!("{}", &o[2..6]);

        for i in "something".chars() {
            println!("{}", i);
        }
    }
}

pub mod hash_maps {
    use std::collections::HashMap;
    pub fn hash() {
        let mut h = HashMap::new();
        h.insert(String::from("move"), 11);
        h.insert(String::from("jump"), 13);

        let teams = vec![String::from("yellow"), String::from("green")];
        let initial_scores = vec![10, 20];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);

        // Only Inserting a Value If the Key Has No Value
        let mut v = HashMap::new();
        v.insert(String::from("something"), 30);
        v.entry(String::from("blue")).or_insert(66);
        v.entry(String::from("something")).or_insert(11);
        println!("{:?}", v);

        // Updating a Value Based on the Old Value
        let text = "hello world wonderful world";
        let mut h = HashMap::new();
        for word in text.split_whitespace() {
            let count = h.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", h);
    }
}
