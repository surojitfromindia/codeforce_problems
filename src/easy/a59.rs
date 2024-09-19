fn soldier_and_banana(k:u64,n:u64,w:u64)-> u64{
    let total_cost = k*(w * (w+1))/2;
    return if total_cost <= n  {0} else {total_cost -n};
}
 
 
pub fn soldier_and_banana_test(){
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();
 
    let values : Vec<&str> = inp.trim().split(" ").collect();
 
    let k = values[0].parse::<u64>().unwrap();
    let n = values[1].parse::<u64>().unwrap();
    let w  = values[2].parse::<u64>().unwrap();
 
    println!("{}", soldier_and_banana(k,n,w));  
}