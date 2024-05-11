use macros::AutoDeref;
#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(mutable = true, field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothting: (),
}
fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothting: (),
    };
    println!("{:?}", resp);
}
