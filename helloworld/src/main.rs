fn main() {
    // i8,u8
    // i16,u16
    // i32,u32
    // i64,u64
    // f32,f64
    // isize,usize

    let a = 1 + 20;
    let b = 2 - 20;
    let c = 3 * 20;
    let d = 4 / 20;
    let e = 5 % 20;

    //char

    let xyz : char = 'c';

    //tuples
    let tup: (i32, f64, char ) = (44, 1.15, 'D');
    let (p, q, r) = tup;
    
    //array
    let arr = [1,2,3,4,5,6,7,8,9,0]; 
    let array1 = arr[0]; // index of the array

    println!("The Integers Variables : {}, {},  {}, {}, {} ", a,b,c,d,e);
    println!("The Character Variable : {}",xyz);
    println!("The Tuple Values : {:?}",tup);
    println!("The borrow tuple varibales : {}, {} ,{}",p,q,r);
    println!("The Array Values : {:?}, the index value of array {:#?} ", arr,array1);
}
