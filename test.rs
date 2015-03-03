pub enum Test
{
	Test1(i32),
	Missing,
}

fn main()
{
	let tes = Test::Test1(2);

	match tes
	{
		Test::Test1(s) => println!("Hell yeah"),
		Test::Missing => println!("haha nope"),
	}
}