use std::collections::HashMap;

//a=1&b=2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    // Heap allocate
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub in s.split('&') {
            let mut key = sub;
            let mut val = "";
            if let Some(i) = sub.find('=') {
                key = &sub[..i];
                val = &sub[i + 1..];
            }

            // if key is existed in hashmap & closure applying
            data.entry(key)
                .and_modify(|existing: &mut Value| match  existing {
                    Value::Single(pre_val) => {
                        // make new vec for replace
                        // let mut vec = Vec::new();
                        // vec.push(val);
                        // vec.push(pre_val);
                        // swap with one in hashmap, dereference
                        *existing = Value::Multiple(vec![pre_val, val]);
                    },
                    Value::Multiple(vec) => vec.push(val)
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}