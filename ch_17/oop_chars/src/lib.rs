mod app {

    pub struct AveragedCollection {
        list: Vec<i32>,
        pub average: f64,
    }
    
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }
        pub fn average(&self) -> f64 {
            self.average
        }
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::AveragedCollection;
    #[test]
    fn add_updates_average() {
        let mut averaged_collection = AveragedCollection::new();
        averaged_collection.add(10);
        assert_eq!(averaged_collection.average, 10.0)
    }
}
