fn main() {
	let s = String::from("hello world");
	let a = &s.as_bytes().iter().enumerate();
	println!("{:?}", &a);
	println!("{}", a[1]);
}
