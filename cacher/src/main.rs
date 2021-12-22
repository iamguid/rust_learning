use std::collections::HashMap;

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(value) = self.values.get(&arg) {
            return *value;
        } else {
            let v = (self.calculation)(arg);
            self.values.insert(arg, v);
            return v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}

fn main() {
    println!("Hello, world!");
}
