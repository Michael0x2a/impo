#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SortedLookup<K, V>
where
    K: Eq + PartialEq + Ord 
{
    data: Vec<(K, V)>,
}

impl<K, V> SortedLookup<K, V> 
where 
    K: Eq + PartialEq + Ord 
{
    pub fn from_vec(mut data: Vec<(K, V)>) -> SortedLookup<K, V> {
        //data.sort_by_key(|(k, _)| k.clone());
        //SortedLookup{data: data}
        unimplemented!()
    }

    pub fn get(&self, key: K) -> Option<V> {
        unimplemented!()
    }
}