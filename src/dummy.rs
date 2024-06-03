pub struct Dummy {
    value: u32,
}

impl Dummy {
    pub fn new(value: u32) -> Dummy {
        Dummy { value }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_increment() {
        let mut dummy = Dummy::new(5);
        dummy.increment();
        assert_eq!(dummy.get_value(), 6);
    }

    #[test]
    fn test_dummy_initial_value() {
        let dummy = Dummy::new(10);
        assert_eq!(dummy.get_value(), 10);
    }
}
