fn main() {
    let mut arr1 = vec![10, 11, 12, 13, 14]; // heap allocated

    let arr2 = [10, 11, 12, 13, 14]; // immutable array, that means it cannot grow but within the limits it can change values
    println!("{}", arr1.len());

    arr1.push(15);
    arr1.pop();
    println!("len:{}", arr1.len());
    println!("cap: {}", arr1.capacity());
    arr1.push(15);
    arr1.push(15);
    arr1.push(15);
    arr1.push(15);
    arr1.push(15);
    arr1.push(15);

    println!("sum arr1:{}", get_sum_of_array(&arr1)); // passing a vector
    println!("sum arr1:{}", get_sum_of_array(&arr2)); // array

    println!("len:{}", arr1.len());
    println!("cap: {}", arr1.capacity());

    println!("{:?}", arr2);

    arr1.remove(0);

    // Vector      array
    // ptr         ptr
    // len         len
    // cap
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];

    //println!("{}",matrix[0][2]);

    for row in &matrix {
        for column in row {
            println!("{}", column);
        }
    }

    println!("normal loop");

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            println!("{}", matrix[i][j]);
        }
    }

    // let mut i =0;
    // let mut j=0;
    // loop{
    //     loop{
    //         if j>=matrix[i].len(){
    //             break;
    //         }
    //         j+=1;
    //     }
    //     i+=1;
    // }
}

fn get_sum_of_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        println!("{}", v);
        sum += v;
    }
    return sum;
}

fn get_string(s1: &str) -> i32 {
    0
}
