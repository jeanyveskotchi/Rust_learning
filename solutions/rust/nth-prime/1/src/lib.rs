pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count: u32 = 1; // we already counted prime 2
    let mut current: u32 = 3;

    loop {
        let mut is_prime = true;
        let mut i = 2;

        while i * i <= current {
            if current % i == 0 {
                is_prime = false;
                break;
            }
            i += 1;
        }

        if is_prime {
            if count == n {
                return current;
            }
            count += 1;
        }

        current += 2; // skip even numbers
    }
}
