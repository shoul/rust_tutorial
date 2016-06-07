fn main() {
    println!("# ownership");
    let string = "
    When a variable binding goes out of scope, rust wil clean up everything 
    related to that binding.
    ";

    println!("{}", string);
    {
    	let v = vec![1, 2, 3];
    	println!("v is in scope and lookes like this: {:?}", v);
    }
    
    let s = "
    v now is out of scope now and completly removed. :)
    ";
    println!("{}", s);

    let s = "
    ## Move sematics

    If a variable move to another name, the origin variable is not accesable
    because the ownership is going to the new variable.

    EXAPTION!!!
    That behavior can be changed with implementing the `Copy` `trait` which ist
    implemented by default in simple types.

    we do:

    let v = vec![1, 2, 3];
    let v2 = v;
    ";
    println!("{}", s);

    let v = vec![1, 2, 3];
    let v2 = v;
    
    let s = "
    'v' is not availabel after that new binding.

    Binding changes also if a variable is given to a function.


    ## Copy implemented by default in simple Types!

    As said the copy implementation ist done for simple types like i32 and
    boolean Types.  The folowing code will run:

    let mut a = 5;
    let mut b = a;
    a += 1;
    
    After that `a` will be changed, but `b will not changed! `b` is a complete
    `Copy` of the origin `a`.
    ";

    println!("{}", s);
    let mut a = 5;
    let mut b = a;
    println!("init a: {}", a);
    println!("init b: {}", b);
    a += 1;
    println!("changed a: {}", a);
    println!("b is not changed! b: {}", b);
}
