enum Value {
    Integer(i32),
    Text(String),
}

impl Value {
    fn add(lhs: &Value, rhs: &Value) -> Option<Value> {
        let Value::Integer(x) = lhs else {
            return None;
        };

        let y = match rhs {
            Value::Integer(i) => Some(i),
            _ => None,
        };

        if y.is_some() {
            Some(Value::Integer(x + y.expect("well, well, well")))
        } else {
            None
        }
    }

    fn ret_int(&self) -> Option<i32> {
        match self {
            Value::Integer(i) => Some(i.clone()),
            _ => None,
        }
    }

    fn ret_str(&self) -> Option<String> {
        match self {
            Value::Text(i) => Some(i.clone()),
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
    {
        let a = Value::Integer(69i32);
        let b = Value::Integer(420i32);

        match Value::add(&a, &b) {
            Some(x) => match x.ret_int() {
                Some(v) => println!("There is a value {}", v),
                None => println!("well, it's empty..."),
            },
            None => println!("We got none, init...."),
        }
    }

    {
        let a = Value::Integer(69i32);
        let b = Value::Text(String::from("ale bydle"));

        if let Some(str) = b.ret_str() {
            println!("dis a string {}", str);
        } else {
            println!("dis not a string");
        }

        match Value::add(&a, &b) {
            Some(x) => match x.ret_int() {
                Some(v) => println!("There is a value {}", v),
                None => println!("well, it's empty..."),
            },
            None => println!("We got none, init...."),
        }
    }
}
