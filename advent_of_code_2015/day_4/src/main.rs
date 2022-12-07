use md5;
fn main(){
    let input = "yzbqklnj".to_owned();
    for x in 0..10000000000 as i64{
        
        let encoded = input.clone() + &x.to_string();
        let hashed = md5::compute(encoded.clone());
        let first_five = format!("{:x}", hashed)[0..5].to_owned();
        let first_six = format!("{:x}", hashed)[..6].to_owned();
        if first_five == "00000"{
            println!("{}", encoded);
        }
        if first_six == "000000"{
            println!("{}", encoded);
            break
        }
    }
}