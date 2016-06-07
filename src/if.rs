fn main() {
    println!("# if");
    println!("IMPORTANT if is an expression.");
    standard_stuff();
    cool_stuff_because_if_is_an_expression();
}

fn standard_stuff() {
	println!("## Pretty standard if, esle, else if stuff.");

    let x = 5;
    if x == 5 {
    	println!("x = {}", x);
    }

    let x = 6;
    if x == 5 {
    	println!("x = {}", x);
    } else {
    	println!("x is not five.");
    }

   	let x = 7;
    if x == 5 {
    	println!("x = five");
    } else if x == 6 {
    	println!("x = six")
    } else {
    	println!("x is not five nor six");
    }
}

fn cool_stuff_because_if_is_an_expression() {
	println!("Cool stuff because if is an expression.");
	let x = 5;

	let y = if x == 5 { 10 } else { 15 };
	println!("x = {}, y = {}", x, y)
}