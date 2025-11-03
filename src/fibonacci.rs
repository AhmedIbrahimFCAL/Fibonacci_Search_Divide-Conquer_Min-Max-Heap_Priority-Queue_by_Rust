use std::ops::{MulAssign};

// first fn naive fibonacci
pub fn fibonacci(n:u32)->u64{
    if n<2 {
        return 1;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}


#[derive(Debug, Clone, Copy)]
struct fibonacci_matrix{
    a:u64,    b:u64,
    c:u64,    d:u64
}

impl fibonacci_matrix{
    fn new()->fibonacci_matrix{
        Self{
            a:1, b:1,
            c:1, d:0
        }
    } 
 
    fn pow(&mut self, exp:u32){
        if exp==0{
            // identity matrix
            self.a=1; self.b=0;
            self.c=0; self.d=1;
            return;
        }
        if exp==1 {
            return;
        }
        let mat = self.clone();
        self.pow( exp>>1);

        *self *= *self;
        if exp&1==1 {
            *self*= mat;
        }
    }   

}
impl MulAssign<fibonacci_matrix> for fibonacci_matrix{
    fn mul_assign(&mut self, rhs: fibonacci_matrix) {
        let A = self.a * rhs.a + self.b * rhs.c;
        let B = self.a * rhs.b + self.b * rhs.d;
        let C = self.c * rhs.a + self.d * rhs.c;
        let D = self.c * rhs.b + self.d * rhs.d;       
        self.a = A;
        self.b = B;
        self.c = C;
        self.d = D;
    }
}

// second fn divide & conquer fibonacci
pub fn divide_conquer_fibonacci(n:u32)->u64{
    let mut mat = fibonacci_matrix::new();
    mat.pow(n);
    mat.a
}

fn dp_fibonacci(n:u32, DP:&mut Vec<u64>) -> u64{
    if n<2 {
        return 1;
    }
    if DP[n as usize] == 0 {
        DP[n as usize] = dp_fibonacci(n-1,DP) + dp_fibonacci(n-2,DP);
    }
    return DP[n as usize];
}
// third fn dynamic programming fibonacci
pub fn dynamic_programming_fibonacci(n:u32)->u64{
    
    let mut DP = vec![0 as u64; n as usize +2];
    dp_fibonacci(n, &mut DP)
}
pub fn test_fibonacci(){
    let n = 12;
    println!();
    println!("Test Fibonacci");
    println!("fibonacci of {n} is {} using naive fibonacci.",fibonacci(n));
    println!("fibonacci of {n} is {} using divide & conquer fibonacci.",divide_conquer_fibonacci(n));
    println!("fibonacci of {n} is {} using naive fibonacci.",dynamic_programming_fibonacci(n));
    println!();
}