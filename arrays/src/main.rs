fn main() {
    println!("Hello, world!");

    let arr: [i32; 0x05] = [10, 123, 34, 343, 43];
    let arr2: [&str; 0b11] = ["Jiten", "Rahim", "Suby"];

    // values can also be given not only in decimal
    // binary or hexa

    for v in &arr {
        println!("{}", v);
    }
    // &arr
    // arr
    println!("Sum of arr:{}", get_sum_of_array(&arr));

    // 'outer: for x in 5..50 {
    //     for y in 0..10 {
    //         if x == y {
    //             break 'outer;
    //         }
    //     }
    // }

    let mut arr2: [i32; 11] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    {
        let slice1: &mut [i32] = &mut arr2[4..8]; // slice but not 8
        slice1[0] =300;
        println!("{:?}", slice1);

    }{
        let slice2:  &mut [i32] = &mut arr2[4..=8]; // include 8 as well
       // slice2[1]=400;
        println!("{:?}", slice2);
      //  println!("{:?}",slice1); // you cannot even read , the reason is if it be read,then
        // it leads to data races
    }{
       
        let slice3: &[i32] = &arr2[4..]; // from 4th to the end
        
        let slice4: &[i32] = &arr2[..]; // all elements
        let slice5: &[i32] = &arr2[..6]; // from 0th till 6th but not 6
        let slice6: &[i32] = &arr2[..=6]; // from 0th till 6th include 6 as well
       
       
        println!("slice3 {:?}", slice3);
        println!("{:?}", slice4);
        println!("{:?}", slice5);
        println!("{:?}", slice6);
    }

    arr2[0] = 100;

    //let arr3 = arr2; // deep copy
    // for n in slice1{
    //     println!("{}",n);
    // }
    let matrix:[[i32;3];2] =[[1,2,3],[4,5,6]];
    println!("{:?}",matrix);
    
}
fn get_sum_of_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        println!("{}", v);
        sum += v;
    }
    // let v = &arr[0];
    return sum;
}
