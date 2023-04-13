fn main() {
    // let x: i8 = 10;
    // println!("{}", x);
// 
    // let y: u8 = 20;
    // println!("{}", y);
// 
    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary =0b1111_1111;
// 
    // println!("{}",decimal);
    // println!("{}", hex);
    // println!("{}", octal);
    // println!("{}", binary);
// 
    // let byte = b'A';
    // println!("{}",byte)

    // let _x =2.0; 
    // let _y: f32 = 1.0;

    // let _t = true;
    // let _f = false;

    // let c = 'c';
    // println!("{}", c);

    // let a = 10;
    // let b = 5;

    // println!("{}", a + b);
    // println!("{}", a-b);
    // println!("{}", a*b);
    // println!("{}", a/b);
    // println!("{}", a%b);


    // let tup = (500,"tup", true);
    // println!("{}", tup.2);
    // let (x,y,z) = tup;
    // println!("{}",x);
    // println!("{}",y);
    // println!("{}",z);

    // let array = [1,2,3];
    // println!("{}", array[1]);

    // let mut array2: [i32;3] = [4,5,6];
    // println!("{}", array2[0]);

    // array2[0] = 10;
    // println!("{}", array2[0]);

    // let mut nums = vec![1,2,3];
// 
    // nums.push(4);
    // println!("{:?}", nums);
    // nums.pop();
    // println!("{:?}", nums);
// 
    // let mut vec = Vec::new(); //vec!
    // vec.push("Test");
    // vec.push("String");
    // println!("{:?}", vec);
// 
    // vec.reverse();
    // println!("{:?}", vec);
// 
    // let vect = Vec::<i32>::with_capacity(2);
    // println!("{}", vect.capacity()); 
// 
    // let v: Vec<i32> = (0..5).collect();
    // println!("{:?}", v);
// 
    // let sv: &[i32] = &v[2..4];
// 
    // println!("{:?}", sv);

    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    // &str = "string slice" or "stir"

    let str1 = "hello";
    println!("{}",str1);

    let str2 = str1.to_string();
    let str3 = &str2;
    println!("{}", str2);
    println!("{}", str3);

    //compare string == or !=
    println!("{}", "ONE".to_lowercase()=="one");
}