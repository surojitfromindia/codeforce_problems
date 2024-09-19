mod easy;
fn main() {}
fn word(inp: String)->String{
    // if smaller count is greater then lower count. convert to small
    
    
    
    let mut small_count : u8= 0 ;
    let mut upper_count : u8= 0 ;
    
    
     for c in inp.chars() {
       // small case start from 'a'.
       let ascii_c = c as u8;
       if ascii_c < b'a' {
           upper_count+=1;
       }
       else{
           small_count+=1
       }
     }
    
   //   println!("s {} u {} ", small_count, upper_count);
    
     if small_count >=upper_count  {inp.to_ascii_lowercase()} else  {inp.to_ascii_uppercase()}
   }
    
    
    
    
    
    
   fn word_test(){
       let mut inp = String::with_capacity(10);
       std::io::stdin().read_line(&mut inp).unwrap();
       println!("{}", word(inp.trim().to_string()));
   }
    
    
    
      
    
    
    
   fn soldier_and_banana(k:u64,n:u64,w:u64)-> u64{
       let total_cost = k*(w * (w+1))/2;
       return if total_cost <= n  {0} else {total_cost -n};
   }
    
    
   fn soldier_and_banana_test(){
       let mut inp = String::with_capacity(10);
       std::io::stdin().read_line(&mut inp).unwrap();
    
       let values : Vec<&str> = inp.trim().split(" ").collect();
    
       let k = values[0].parse::<u64>().unwrap();
       let n = values[1].parse::<u64>().unwrap();
       let w  = values[2].parse::<u64>().unwrap();
    
       println!("{}", soldier_and_banana(k,n,w));  
   }