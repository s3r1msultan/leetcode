/*A complex number can be represented as a string on the form "real+imaginaryi" where:

real is the real part and is an integer in the range [-100, 100].
imaginary is the imaginary part and is an integer in the range [-100, 100].
i2 == -1.

Given two complex numbers num1 and num2 as strings, return a string of the complex number that represents their multiplications.



Example 1:

Input: num1 = "1+1i", num2 = "1+1i"
Output: "0+2i"
Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of 0+2i.

Example 2:

Input: num1 = "1+-1i", num2 = "1+-1i"
Output: "0+-2i"
Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.



Constraints:

num1 and num2 are valid complex numbers.

*/

pub fn complex_number_multiply(num1: String, num2: String) -> String {
    let parts_1 = num1.split("+").collect::<Vec<_>>();
    let parts_2 = num2.split("+").collect::<Vec<_>>();
    let real_1 = parts_1[0].parse::<i32>().unwrap();
    let imaginary_1 = parts_1[1].split("i").next().unwrap().parse::<i32>().unwrap();
    let real_2 = parts_2[0].parse::<i32>().unwrap();
    let imaginary_2 = parts_2[1].split("i").next().unwrap().parse::<i32>().unwrap();

    let real_part = real_1 * real_2 - imaginary_1 * imaginary_2;
    let imaginary_part = imaginary_1 * real_2 + imaginary_2 * real_1;

    format!("{}+{}", real_part, imaginary_part)
}