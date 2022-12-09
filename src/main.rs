#![allow(dead_code)]

mod euler;
mod p14;

unsafe fn t(n: i32) -> i32
{
    static mut X: i32 = 0;

    X += n;
    return X;
}

unsafe fn t1(_str: &str) -> &str
{
    static mut S: String = String::new();

    S.push_str(_str);
    return &S;
}

fn main()  
{
    p14::p14();
}