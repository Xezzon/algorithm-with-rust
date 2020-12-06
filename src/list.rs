pub trait List<E> {
    fn push(&mut self, element: E) -> bool;
    fn insert(&mut self, index: i32, element: E);
    fn push_all(&mut self, elements: List<E>) -> bool;
    fn insert_all(&mut self, index: i32, elements: List<E>) -> bool;
    fn clear(&mut self);
    fn contains(&self, element: E) -> bool;
    fn contains_all(&self, elements: List<E>) -> bool;
    fn equals(&self, another: List<E>) -> bool;
    fn get(&self, index: i32) -> E;
    fn index_of(&self, element: E) -> i32;
    fn is_empty(&self) -> bool;
    // fn interator(&self)
    fn last_index_of(&self, element: E) -> i32;
    fn remove_i(&mut self, index: i32) -> E;
    fn remove(&mut self, element: E) -> bool;
    fn remove_all(&mut self, elements: List<E>) -> bool;
    // fn replace_all()
    fn retain_all(&mut self, elements: List<E>) -> bool;
    fn set(&mut self, index: i32, element: E) -> E;
    fn size(&self) -> i32;
    fn sub_list(&self, from_index: i32, to_index: i32);
    fn to_array(&self) -> [E];
}
