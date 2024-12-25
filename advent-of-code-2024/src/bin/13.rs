advent_of_code::solution!(13);

pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0i64 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}


fn solve(a: &Vec<i64>, b : &Vec<i64>, prize : &Vec<i64>) -> Option<i64> {
    //first make sure about the gcd
    let gcd0 = fast_gcd(a[0], b[0]);
    if prize[0] % gcd0 != 0 {return None};

    let gcd1 = fast_gcd(a[1], b[1]);
    if prize[1] % gcd1 != 0 {return None};

    let det = a[0]*b[1]-b[0]*a[1];
    let mut mest = 0i64;
    let mut nest = 0i64;
    //second, find det. if it zero, handle it here
    if det==0 {
        //this checks if a and p are multiples of each other
        if a[1]*prize[0]-prize[1]*a[0] != 0 {
            return None;
        }
        let mut n = prize[0]/b[0];
        n += 1;
        while((prize[0]-n*b[0]) % a[0]) != 0{
            if n>0 {n -= 1i64;}
            else {return None};
        }
        let m = (prize[0]-n*b[0])/a[0];
        return Some(3i64*m+n)
    }
    else{
    //if det is not zero, then solve. see if it works
        mest = b[1]*prize[0] - b[0]*prize[1];
        nest = a[0]*prize[1]-a[1]*prize[0];

        if (mest % det) != 0 {return None;}
        if (nest % det) != 0 {return None;}

        mest /= det;
        nest /= det;
    }

    println!("m is {} and n is {}",mest,nest);

    Some(3i64*mest + nest)
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut counter = 0;
    let mut sum = 0u64;
    let mut a = Vec::<i64>::from([0,0]);
    let mut b = Vec::<i64>::from([0,0]);
    let mut prize = Vec::<i64>::from([0,0]);
    for line in input.lines(){
        if line.is_empty() {continue;}

        if counter==0 {
            let toks : Vec<&str> = line.split(['+',' ',',']).into_iter().collect();

            a[0] = toks[3].parse::<i64>().unwrap();
            a[1] = toks[6].parse::<i64>().unwrap();
            counter +=1;
        }
        else if counter==1{
            let toks : Vec<&str> = line.split(['+',' ',',']).into_iter().collect();

            b[0] = toks[3].parse::<i64>().unwrap();
            b[1] = toks[6].parse::<i64>().unwrap();
            counter +=1;
        }
        else {
            let toks : Vec<&str> = line.split(['=',' ',',']).into_iter().collect();

            prize[0] = toks[2].parse::<i64>().unwrap()+10000000000000;
            prize[1] = toks[5].parse::<i64>().unwrap()+10000000000000;
            counter =0;
            //println!("a is {:?}, b is {:?}, prize is {:?}",a,b,prize);

            let ret = solve(&a, &b, &prize);
            match ret {
                Some(val)=> sum+= val as u64,
                None => println!("no solution")
            }
        }
    }
    println!("the answer is {}",sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
