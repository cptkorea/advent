pub fn num_digits(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut cnt = 0;
    while n > 0 {
        n /= 10;
        cnt += 1;
    }

    cnt
}
