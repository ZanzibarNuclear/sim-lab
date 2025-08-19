use std::slice;

unsafe trait Foo {}
unsafe impl Foo for i32 {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // let r = &mut v[..];
    // let (a, b) = split_at_mut(r, 3);

    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // let mut num = 3;

    // let r1 = &raw const num;
    // let r2 = &raw mut num;

    // num = num + 5;

    // unsafe {
    //     println!("num is {}", num);
    //     println!("r1 is {}", *r1);
    //     println!("r2 is {}", *r2);
    // }

    // unsafe fn dangerous() {
    //     println!("What?");
    // }

    // unsafe {
    //     dangerous();
    // }
}

static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
