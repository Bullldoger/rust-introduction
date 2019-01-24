fn main() {

    let _x = fibbonachi(10, 1);
    
    println!("Fib(10) = {}", _x);
}

fn fibbonachi(n : i32, way : i8) -> i32 {
    
    fn fibbonachi_iterative(n : i32) -> i32 {
        let mut f1 = 0;
        let mut f2 = 1;
        
        for _i in 1..n {
        
            let _t = f1;
            
            f1 = f2;
            f2 = _t + f2;
            
        }
        
        f2
    }

    fn fibbonachi_recursive(n : i32) -> i32 {
        
        if n == 0 {
            let r : i32 = 0;
            return r
        }
        if n == 1 {
            let r : i32 = 1;
            return r
        }
        
        if n > 1 {
            let f1 : i32  = fibbonachi_recursive(n - 1);
            let f2 : i32  = fibbonachi_recursive(n - 2);
            return f1 + f2
        } else { return -1 };
    }
    
    if way == 0 { return fibbonachi_iterative(n) }
    else { return fibbonachi_recursive(n) };
    
}

