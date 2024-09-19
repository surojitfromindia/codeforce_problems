// a116
// need to calculate maximum passenger count.

struct OutIn(u32, u32);

fn tram(out_in: Vec<OutIn> )-> u32 {
    // let out_in: Vec<OutIn> = vec![OutIn(0, 3), OutIn(2, 5), OutIn(4, 2), OutIn(4, 0)];

    let mut max_holding = 0;
    let mut now_holding = 0;
    for OutIn(o, i) in out_in {
        now_holding -= o;
        now_holding += i;
        if now_holding > max_holding {
            max_holding = now_holding;
        }
    }
    max_holding
}

fn tram_test() {
    let mut c_str = String::with_capacity(10);
    std::io::stdin().read_line(&mut c_str).unwrap();
    let n = c_str.trim().parse::<usize>().unwrap();

    let mut st: Vec<OutIn> = Vec::with_capacity(n);

    for i in std::io::stdin().lines() {

        if i.is_err() {
            break;
        }

        
        let line = i.unwrap();

        if line.is_empty() {
            break;
        }            
        let c :Vec<u32>=     line.trim()
            .split(" ")
            .map(|z| z.parse::<u32>().unwrap()).collect();
        st.push(OutIn(c[0], c[1]));
    }

    let res = tram(st);
    println!("{}", res);
}
