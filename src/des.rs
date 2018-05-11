extern crate serde;

use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};

struct Time(i32);

impl MyMapVisitor {
    fn new() -> Self {
        Time{ 0 }
    }
}

impl<'de> Visitor<'de> for Time
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special int")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = MyMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'de, K, V> Deserialize<'de> for MyMap<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MyMapVisitor::new())
    }
}