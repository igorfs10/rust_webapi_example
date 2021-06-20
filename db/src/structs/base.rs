pub struct Base<T> {
    result: T,
}

impl<T> Base<T> {
    pub fn new(item: T) -> Self {
        Base { result: item }
    }

    pub fn get_result(self) -> T {
        self.result
    }

    pub fn get_vec_result(list: Vec<Self>) -> Vec<T> {
        let mut new_list = Vec::new();
        for item in list {
            new_list.push(item.result);
        }
        new_list
    }
}
