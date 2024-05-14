use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let mut mymap: HashMap<String, String> = HashMap::new();
    mymap.insert("56086".to_string(), "Bangalore-Yeshvantpur".to_string());
    mymap.insert(
        "56096".to_string(),
        "Bangalore-Mahalakshmi Layout".to_string(),
    );
    mymap.insert(
        "56036".to_string(),
        "Bangalore-BTM".to_string(),
    );

    println!("{:?}", mymap);
    let pincode: String = "560861".to_string();
    let v: Option<&String> = mymap.get(&pincode);
    // why the value from the get is option?

    match v {
        Some(vr) => {
            println!("{}", vr);
        }
        None => {
            println!("No value associated with the key");
        }
    }

    for (key,value) in &mymap{
        println!("Key:{} Value:{}",key,value);
    }

    mymap.remove(&String::from("56036"));

    println!("Has it been removed?");
    for (key,value) in &mymap{
        println!("Key:{} Value:{}",key,value);
    }

}

//

// maps are heap allocated
// hashmaps -> values are mapped to keys
// what is a hash function? Hash Key?
// O(1) --?
// hash maps, it creates some buckets --> 8 buckets
// buckets are arrays or linkedlists
// key: 56086 value: "Bangalore-Yeshvantpur"
// 281ce516d315932803 25473d5cb5340c1983a83f
// segment             offset
// HashKey->281ce516d315932803 --> 4
// Hashkey->25473d5cb5340c1983a83f -> 8

// "Bangalore-Yeshvantpur" will be stored in 4th bucket 8th element
//  key: 56086 value: "Bangalore-Mahalakshi layout"
