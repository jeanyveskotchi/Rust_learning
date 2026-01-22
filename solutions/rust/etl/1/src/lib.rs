use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
 let mut map : BTreeMap<char, i32> =BTreeMap::new();
    for (key,values) in h {
        for &c in values{
        map.insert(c.to_ascii_lowercase(),*key);
    }
    }
    map
}
