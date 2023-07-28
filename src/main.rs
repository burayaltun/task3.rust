
struct FilterCondition<X>{
    value: X,

}

impl<X: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &X) -> bool {
        &self.value == item
    }
}

fn custom_filter<X>(collection: &Vec<X>, filter_condition: &FilterCondition<T>) -> Vec<&X> {
    collection.iter().filter(|item| filter_condition.is_match(item)).collect()
}

fn main() {
   
    let my_collection = vec![1, 2, 3, 4, 5, 4, 3, 2, 1];

   
    let filter_condition = FilterCondition { value: 3 };

    
    let filtered_result = custom_filter(&my_collection, &filter_condition);

    
    println!("The filtered result is: {}", filtered_result);
}