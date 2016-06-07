fn main() {
    println!("# Loops");
    loop_and_break();
    while_inside();
    for_inside();
    for_enumerates();
    loop_lable();
}

fn loop_and_break() {
	println!("## Loop and break!");
	let mut v = 0;
	loop {
		println!("Loop forever!");
		if v == 2 { break; }
		v += 1
	}
}

fn while_inside() {
	println!("## While inside!");
	let mut done = false;
	let mut v = 0;

	while !done {
		if v == 2 {
			println!("Done!");
			done = true;
		} else {
			println!("Not done!");
		}
		v +=1
	}
}

fn for_inside() {
	println!("## For inside!");
	for j in 3..6 {
		println!("j = {}", j);
	}
}

fn for_enumerates() {
	println!("## For enumerates!");
	for (i,j) in (3..6).enumerate() {
	    println!("i = {}, j = {}", i, j);
	}
}

fn loop_lable() {
	println!("## Loop lable!");
	'outer: for i in 3..7 {
	    'inner: for j in 3..7 {
	    	if i % 2 == 0 { continue 'outer; }
	    	if j % 2 == 0 { continue 'inner; }
	    	println!("i = {}, j = {}", i, j);
	    }
	}
}