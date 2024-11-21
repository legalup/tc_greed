/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

 

Example 1:

Input: x = 123
Output: 321

Example 2:

Input: x = -123
Output: -321

Example 3:

Input: x = 120
Output: 21

 

Constraints:

    -231 <= x <= 231 - 1

*/

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        //first, lets get the digits
        let mut dig = Vec::<i32>::new();
        let mut xx = if x>=0 {x} else {-x};
        let mut isneg = (x<0);
        while xx>0 {
            let yy = xx % 10;
            dig.push(yy);
            xx -= yy;
            xx /= 10;
        }

        let mut ret = dig.iter().fold(0i64,|ret, dd|{
            10i64*ret + (*dd as i64)
        });

        if isneg {
            ret = -ret
        }
        
        let t31 = 1i64 << 31;
        //println!("here is t31 {}",t31);
        if ret< (-t31) || ret>(t31-1i64) {
            return 0i32;
        }

        ret as i32