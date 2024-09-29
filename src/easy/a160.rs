// a160
fn twins(mut coins: Vec<u32>) -> u32 {
    let sum: u32 = coins.iter().sum::<u32>();
    let half_sum = sum / 2;

    // sort by desc and take as many as possible
    let mut taken = 0;

    coins.sort();
    coins.reverse();

    let mut m_sum = 0;
    for coin in coins {
        if m_sum > half_sum {
            break;
        }
        m_sum += coin;
        taken += 1;
    }
    taken
}
pub fn twins_test() {
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();

    let mut coins_inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut coins_inp).unwrap();
    let coins: Vec<u32> = coins_inp
        .trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let res = twins(coins);
    println!("{res}")
}
