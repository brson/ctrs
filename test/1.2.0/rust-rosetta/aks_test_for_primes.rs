// http://rosettacode.org/wiki/AKS_test_for_primes



pub fn is_prime(p: u32) -> bool {
    if p<2 {
        false
    } else {
        let mut c = coefficients(p as usize);
        c[0] -= 1;
        for i in (0..(c.len() + 1) / 2) {
            if (c[i] % (p as i64)) != 0 {
                return false
            }
        }
        true
    }
}

// need to allow dead code because this is used as a library
// by pernicious numbers
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    for p in 0..8 {
        println!("{}: {:?}", p, coefficients(p));
    }

    for p in (1..51).filter(|&x| is_prime(x)) {
        print!("{} ", p);
    }
}

fn coefficients(p: usize) -> Vec<i64> {
    if p==0 {
        vec![1]
    } else {
        let mut result = vec![1, -1];
        let zero = Some(0i64);
        for _ in (1..p) {
            result = {
                let a = result.iter().chain(zero.iter());
                let b = zero.iter().chain(result.iter());
                a.zip(b).map(|(x, &y)| *x-y).collect()
            };
        }
        result
    }
}

#[test]
fn test_solution() {
    let exp_coefficients =  vec![
                            vec![1i64],
                            vec![1, -1],
                            vec![1, -2, 1],
                            vec![1, -3, 3, -1],
                            vec![1, -4, 6, -4, 1],
                            vec![1, -5, 10, -10, 5, -1],
                            vec![1, -6, 15, -20, 15, -6, 1],
                            vec![1, -7, 21, -35, 35, -21, 7, -1]];
    let exp_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    for (i, exp) in exp_coefficients.iter().enumerate() {
        assert_eq!(*exp, coefficients(i));
    }

    let primes: Vec<u32> = (1..51).filter(|&i| is_prime(i)).collect();
    assert_eq!(exp_primes, &primes[..]);
}
