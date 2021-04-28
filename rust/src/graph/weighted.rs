// trait for elements that have a weighting associated with them
pub trait Weighted<T: Ord + Eq> {
    // returns the weight of the type
    fn weight(&self) -> &T;

    // sets the weight of the type
    fn set_weight(&mut self, new_weight: T);
}