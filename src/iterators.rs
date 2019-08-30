pub fn iterator_test() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = v.iter().map(|i| i + 1).collect();

    println!("{:?}", result);
}

#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u8,
    brand: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u8) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == my_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            brand: "Nike".to_string(),
        },
        Shoe {
            size: 12,
            brand: "Adidas".to_string(),
        },
        Shoe {
            size: 10,
            brand: "Prada".to_string(),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                brand: "Nike".to_string()
            },
            Shoe {
                size: 10,
                brand: "Prada".to_string()
            }
        ]
    );
}

// Creating our own iterator with the Iterator Trait

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_our_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}
