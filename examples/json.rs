use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}
impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}
macro_rules! impl_from_num_for_json {
	( $( $t:ident )* ) => {
			$(
					impl From<$t> for Json {
							fn from(n: $t) -> Json {
									Json::Number(n as f64)
							}
					}
			)*
	};
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
											usize isize f32 f64);
macro_rules! json {
	(null) => {
			Json::Null
	};
	([ $( $element:tt ),* ]) => {
			Json::Array(vec![ $( json!($element) ),* ])
	};
	({ $( $key:tt : $value:tt ),* }) => {
			Json::Object(vec![
					$( ($key.to_string(), json!($value)) ),*
			].into_iter().collect())
	};
	( $other:tt ) => {
			Json::from($other)  // 处理布尔值、数值和字符串
	};
}

fn main() {
    let json = json!({
            "name": "json",
            "age": 1,
            "is_json": true,
            "nothing": null,
            "array": [1, 2, 3],
            "object": {
                    "key": "value"
            }
    });
    println!("{:#?}", json);
    let json = json!([1, 2, 3]);
    println!("{:?}", json);
    let json = json!(null);
    println!("{:?}", json);
    let json = json!(true);
    println!("{:?}", json);
    let json = json!(1);
    println!("{:?}", json);
    let json = json!(1.0);
    println!("{:?}", json);
    let json = json!("hello");
    println!("{:?}", json);
}
