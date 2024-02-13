use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &str) -> Self {
        let mut data = HashMap::new();
        for sub_string in value.split('&') {
            let mut key = sub_string;
            let mut val = "";
            sub_string.find('=').map(|i| {
                key = &sub_string[..i];
                val = &sub_string[i + 1..];
            });
            data.entry(key)
                .and_modify(|existing: &mut Value| {
                    match existing {
                        Value::Single(prev_val) => {
                            *existing = Value::Multiple(vec![val, prev_val]);
                        }
                        Value::Multiple(vec) => {
                            vec.push(val);
                        }
                    }
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}