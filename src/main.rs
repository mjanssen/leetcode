mod solutions;

fn main() {
    let input = vec![73, 98, 9];

    let ret = solutions::_1846::Solution::maximum_element_after_decrementing_and_rearranging(input);
    println!("{:?}", ret);
}
