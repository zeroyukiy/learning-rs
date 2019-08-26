pub mod closures_test {
    use std::thread;
    use std::time::Duration;
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    pub fn main() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    // Storing Closures Using Generic Parameters and the Fn Traits
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn generate_workout(intensity: u32, random_number: u32) {
        // Refactoring Using Functions
        // let expensive_result = simulated_expensive_calculation(intensity);
        // Refactoring Using Cacher struct and generic closure
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly..");
            thread::sleep(Duration::from_secs(2));
            num
        });
        // Refactoring with Closures to Store Code
        // let expensive_closure = |num| {
        //     println!("calculating slowly..");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };

        // Closure Type Inference and Annotation
        // let expensive_closure = |num: u32| -> u32 {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };

        // fn add_one_v1(x: u32) -> u32 {
        //     x + 1
        // }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x| x + 1;
        // let add_one_v4 = |x| x + 1;

        if intensity < 25 {
            println!("Today, do {} pushups", expensive_result.value(intensity));
            println!("Next, do {} situps", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
}
