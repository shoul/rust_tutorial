fn main() {
    println!("# vectors");
    vec_basic();
    vec_nuber_of_items();
    acessing_elements();
    usize_indexing_is_importent();
    iterating();
}

fn vec_basic() {
	println!("## Basic");
	let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
	println!("v = {:?}", v);
}

fn vec_nuber_of_items() {
	println!("## Number of items.");
   	let v = vec![0; 5]; // ten zeros
	println!("v = {:?}", v);
}

fn acessing_elements() {
	println!("## Acessing Elements");
	// Returns 3 because of the firs element is zero.
	let v = vec![1, 2, 3, 4, 5];
	println!("The third element is {}", v[2]);
}

fn usize_indexing_is_importent() {
	println!("Usize indexing is importent.");
	let v = vec![1, 2, 3, 4, 5];
	
	let i: usize = 0;
	let j: i32 = 0;

	println!("The first element is: {}", v[i]);

	// This would be panic
	//println!("The first element is: {}", v[j]);
}

fn iterating() {
	println!("## Iterating");

	let mut v = vec![1, 2, 3];

	println!("There are three versions of iterating through a vector:");
	for i in &v {
		println!("A reference to {}", i);
	}

	println!("The vector must be mutable if you whant to use `&mut`in iteration.");
	for i in &mut v {
		println!("A mutable reference to {}", i);
	}

	println!("If you take ownership, you can use the vector only once!");
	for i in v {
		println!("Take ownership of the vector and its elements {}", i);
	}
}