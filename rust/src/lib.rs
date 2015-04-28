#![feature(convert)]

extern crate rlibc;
extern crate rand;

use std::thread;
use rand::Rng;

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    inefficient_string().as_ptr()
}

/// `fill_slice` fills up a `buffer` with "Hello, world!"
///
/// # Unsafe
///
/// This function assumes that you've allocated at least fourteen bytes of memory at `buffer`. If
/// you haven't, bad things may happen.
#[no_mangle]
pub unsafe extern "C" fn fill_slice(buffer: *mut u8) {
    let s = inefficient_string();

    rlibc::memcpy(buffer, s.as_ptr(), 14);
}

#[no_mangle]
pub extern "C" fn rust_example_init() {
    thread::spawn(move || {
        loop {
            println!("############################## Thread tick ##############################");
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("{}", inefficient_string());
            println!("#########################################################################");

            thread::sleep_ms(1000);
        }
    });
}

fn inefficient_string() -> String {
    let random_number = rand::thread_rng().gen::<i32>();

    // Allocate a bunch of stuff
    let a = format!("{}-", &random_number);
    let b = format!("{}-", &random_number);
    let c = format!("{}-", &random_number);
    let d = format!("{}-", &random_number);
    let e = format!("{}-", &random_number);
    let f = format!("{}-", &random_number);
    let g = format!("{}-", &random_number);
    let h = format!("{}-", &random_number);
    let i = format!("{}-", &random_number);
    let j = format!("{}-", &random_number);
    let k = format!("{}-", &random_number);
    let l = format!("{}-", &random_number);
    let m = format!("{}-", &random_number);
    let n = format!("{}-", &random_number);
    let o = format!("{}-", &random_number);
    let p = format!("{}-", &random_number);
    let q = format!("{}-", &random_number);
    let r = format!("{}-", &random_number);
    let s = format!("{}-", &random_number);
    let t = format!("{}-", &random_number);
    let u = format!("{}-", &random_number);
    let v = format!("{}-", &random_number);
    let w = format!("{}-", &random_number);
    let x = format!("{}-", &random_number);
    let y = format!("{}-", &random_number);
    let z = format!("{}-", &random_number);

    let mut out = "".to_string();

    out.push_str(a.as_str());
    out.push_str(b.as_str());
    out.push_str(c.as_str());
    out.push_str(d.as_str());
    out.push_str(e.as_str());
    out.push_str(f.as_str());
    out.push_str(g.as_str());
    out.push_str(h.as_str());
    out.push_str(i.as_str());
    out.push_str(j.as_str());
    out.push_str(k.as_str());
    out.push_str(l.as_str());
    out.push_str(m.as_str());
    out.push_str(n.as_str());
    out.push_str(o.as_str());
    out.push_str(p.as_str());
    out.push_str(q.as_str());
    out.push_str(r.as_str());
    out.push_str(s.as_str());
    out.push_str(t.as_str());
    out.push_str(u.as_str());
    out.push_str(v.as_str());
    out.push_str(w.as_str());
    out.push_str(x.as_str());
    out.push_str(y.as_str());
    out.push_str(z.as_str());

    out
}
