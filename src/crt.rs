use itertools::Itertools;
use num::Integer;

pub struct Input {
    // divisors
    pub ni: usize,
    // remainders
    pub ai: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Solutions {
    // minimal solution
    pub min: usize,
    // increment between solutions
    // (each min + k*n) is a solution
    pub n: usize,
}

pub fn chinese_remainder(inputs: &[Input]) -> Solutions {
    let ns = inputs.iter().map(|input| input.ni);
    assert!(inputs.iter().all(|d| d.ai < d.ni));
    assert!(ns.clone().permutations(2).all(|v| coprime(v[0], v[1])));
    let n = ns.clone().product();
    let es = ns.map(|ni| {
        let inv_ni = (n / ni) as isize;
        let ext_gcd = (ni as isize).extended_gcd(&(inv_ni));
        let vi = (ext_gcd.y + ni as isize) % ni as isize;
        vi * inv_ni
    });
    let min: isize = inputs
        .iter()
        .map(|input| input.ai)
        .zip(es)
        .map(|(ai, ei)| ai as isize * ei)
        .sum();
    Solutions {
        min: (min as usize) % n,
        n,
    }
}

pub fn coprime(a: usize, b: usize) -> bool {
    a.gcd(&b) == 1
}

#[cfg(test)]
mod test_crt {
    use crate::crt::{Input, Solutions};

    #[test]
    fn test_chinese_remainder() {
        let inputs = [
            Input { ni: 3, ai: 2 },
            Input { ni: 5, ai: 3 },
            Input { ni: 7, ai: 2 },
        ];
        assert_eq!(
            crate::crt::chinese_remainder(&inputs),
            Solutions { min: 23, n: 105 }
        );
    }
}
