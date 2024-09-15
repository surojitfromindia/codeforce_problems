fn traffic_cost(commands: Vec<String>) -> usize {
    // initally we have no contacts
    let mut person_count = 0;
    let mut cost = 0;

    for cmd in commands {
        if cmd.starts_with("+") {
            person_count += 1;
        } else if cmd.starts_with("-") {
            person_count -= 1;
        } else {
            if let Some(msg) = cmd.split(":").last() {
                cost += msg.len() * person_count;
            }
        }
    }
    cost
}

pub fn traffic_cost_test() {
    // non-trim strings
    let mut commands = Vec::new();

    for line in std::io::stdin().lines() {
        let lm = line.unwrap();
        if lm.is_empty() {
            break;
        }
        // trim and append
        commands.push(lm);
    }

    // println!("{:?}", commands);
    println!("{}", traffic_cost(commands));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn traffic_cost_test() {
        let b = vec!["+Mike", "+Kate", "Kate:hi"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(traffic_cost(b), 4);
    }
}
