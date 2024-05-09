use anyhow::Result;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3];
    // let v = vec![1, 2, 3];
    println!("{:?}", v);
    let v2: Vec<i32> = my_vec!();
    println!("{:?}", v2);
    let v3 = my_vec!(1;4);
    println!("{:?}", v3);
    let v4: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?, "4".parse()?];
    println!("{:?}", v4);
    Ok(())
}

// my_vec!=my_vec!{1,2,3};// Vec<i32>
#[macro_export]
macro_rules! my_vec {
	() => {
		Vec::new()
	};
	($elem:expr; $n:expr) => {
		std::vec::from_elem($elem, $n)
	};
		($($x:expr),+ $(,)? ) => {
				{
					<[_]>::into_vec(Box::new([$($x),+]))
					// $crate::boxed::Box::new([$($x),+])
				}
		};
}
