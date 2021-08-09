pub trait NamedObject {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: &str);
}

pub struct NamedObjectVector<T: NamedObject> {
    objs: Vec<T>,
}

impl<T: NamedObject> NamedObjectVector<T> {
    pub fn new() -> Self { Self { objs: vec![] } }

    pub fn define(&mut self, obj: &T) -> &T { todo!() }

    pub fn defined(&self, name: &String) -> bool { false }

    pub fn size(&self) -> usize { todo!() }

    pub fn names(&self) -> Vec<String> { todo!() }

    pub fn values(&self) -> Vec<T> { todo!() }

    pub fn delete(&mut self, name: &String) {}
}

pub trait GetObject<K, V: NamedObject> {
    fn get(&self, key: &K) -> &V;
}

impl<V: NamedObject> GetObject<String, V> for NamedObjectVector<V> {
    fn get(&self, key: &String) -> &V { todo!() }
}

impl<V: NamedObject> GetObject<usize, V> for NamedObjectVector<V> {
    fn get(&self, key: &usize) -> &V { todo!() }
}

pub trait GetIndex<K> {
    fn index(&self, key: &K) -> usize;
}

impl<V: NamedObject> GetIndex<String> for NamedObjectVector<V> {
    fn index(&self, key: &String) -> usize { todo!() }
}

impl<V: NamedObject> GetIndex<V> for NamedObjectVector<V> {
    fn index(&self, key: &V) -> usize { todo!() }
}

pub trait GetName<K> {
    fn name(&self, key: &K) -> &String;
}

impl<V: NamedObject> GetName<usize> for NamedObjectVector<V> {
    fn name(&self, key: &usize) -> &String { todo!() }
}

impl<V: NamedObject> GetName<Box<dyn NamedObject>> for NamedObjectVector<V> {
    fn name(&self, key: &Box<dyn NamedObject>) -> &String { todo!() }
}

impl<V: NamedObject> ToString for NamedObjectVector<V> {
    fn to_string(&self) -> String { todo!() }
}

impl<V: NamedObject> Default for NamedObjectVector<V> {
    fn default() -> Self { Self::new() }
}
