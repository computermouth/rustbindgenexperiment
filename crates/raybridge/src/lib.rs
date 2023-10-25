
mod internal;

pub fn safe_add(a: i32, b: i32) -> i32 {
    unsafe {
        return internal::add(a, b);
    }
}

pub struct Teststruct {
    pub a: i32,
}

pub fn safe_teststruct(a: Teststruct, b: Teststruct) -> Teststruct {
    
    let inta = internal::teststruct{a: a.a};
    let intb = internal::teststruct{a: b.a};
    
    let mut rf = Teststruct{a: 0};
    unsafe {
        let f: internal::teststruct = internal::teststruct_add(inta, intb);
        rf.a = f.a;
    }
    return rf;
}