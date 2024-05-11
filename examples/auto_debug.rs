use macros::AutoDebug;
#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothting: (),
    hello: u32,
}
fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothting: (),
        hello: 42,
    };
    println!("{:?}", resp);
}
