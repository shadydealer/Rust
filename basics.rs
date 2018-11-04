fn main() {
	let s: &str = "Rust rules";
	println!("string reference = {}", s);

	let c: char = 'd';
	println!("char = {}",c);

	let arr: [i32; 2] = [1,2];

	print!("[");
	for i in 0..2{
		print!("{}",arr[i]);
		if i != 1{
			print!(", ");
		}
	}	
	print!("]\n");

	let a: i32 = 8;
	let b: i16 = 4;

	let result = a - (b as i32);
	println!("{} - {} = {}",a,b,result);

	let mut integer_pi: i32 = 3.14_f32 as i32;
	println!("integer_pi = {:?}", integer_pi);

	integer_pi -=1;

	println!("integer_pi -=1 is {:?}", integer_pi );

	let mut the_one_ring: String = String::from("the one ring");
	
	/*this will cause Frodo to pick up the one ring*/
	the_one_ring = returns_the_precious(the_one_ring);
	println!("Frodo has obtained {}!", the_one_ring);

	/*this will cause Frodo to lend the one right to Gandalf*/
	lend_to_gandalf(&the_one_ring);
	println!("");
	println!("Sam: Share the load...");

	/*this will cause Frodo to lend the one ring to Sam*/
	borrows_the_precious(&mut the_one_ring);
	println!("Frodo: Give me back {}!", the_one_ring);

	/*this will cause Frodo to lose the one ring*/
	takes_the_precious(the_one_ring);

	// println!("{:?}", the_one_ring); => this will cause the universe to fold in half.
}

fn takes_the_precious(precious: String) -> (){
	println!("*Golum bites off Frodo's finger*");
	println!("My... precious... *pets {}*", precious);
}

fn returns_the_precious(precious: String) -> String {
	precious
}

fn borrows_the_precious(precious: &mut String) -> () {

	println!("*Sam waves {} infront of Golums face*", precious);

	/*Does not throw error because multiple immutable references to an object may exists during runtime*/
	show_golum(precious);
	
	precious.push_str("(scratched)");
	println!("Sam: oops...");
}

fn show_golum(_precious: & String) -> () {
	println!("*Golum Drools*");
}

fn lend_to_gandalf(precious: &String) -> () {
	println!("*Gandalf grasps {} tightly*", precious);
}
