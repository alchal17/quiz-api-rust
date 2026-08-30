pub trait Repository<T, E, W>: Send + Sync + std::fmt::Debug {
    fn get_all(&self) -> Result<Vec<T>, String>;
    fn get_by_id(&self, id: i32) -> Result<Option<T>, String>;
    fn create(&self, creating_enity: &E) -> Result<T, String>;
    fn update(&self, id: i32, updating_entity: &W) -> Result<T, String>;
    fn delete(&self, id: i32) -> Result<usize, String>;
}
