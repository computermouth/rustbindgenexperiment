use raybridge;

fn main() {
    println!("Hello, {}!", raybridge::safe_add(1, 2));
    
    let f1 = raybridge::Teststruct { a: 0 };
    let f2 = raybridge::Teststruct { a: 1 };
    
    println!("Hello Teststruct, {}!", raybridge::safe_teststruct(f1, f2).a );
    
}
