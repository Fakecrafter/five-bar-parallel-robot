#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]


fn main() {
    let (l1, l2, l3, l4, l5) = (5.0, 5.0, 5.0, 5.0, 5.0);
    let a = [0.0, 0.0];
    let e = [l5, 0.0];
    let base1 = 32.5;
    let base4 = 32.5;
    let b = calculate_point_from_other_point(l1, base1, a);
    let d = calculate_point_from_other_point(l4, base4, e);
    let innerA = 2.0 * l3 * l4 * base4.sin() - 2.0 * l1 * l3 * base1.cos();
    let innerB = 2.0 * l3 * l5 - 2.0 * l1 * l3 * base1.sin() + 2.0 * l3 * l4 * base4.cos();
    let innerC = l1 * l1 - l2 * l2 + l3 * l3 + l4 * l4 + l5 * l5
	- l1 * l4 * base1.sin() * base4.sin()
	- 2.0 * l1 * l5 * base1.cos() + 2.0 * l1 * l5 * base4.cos()
	- 2.0 * l1 * l4 * base1.cos() * base4.cos();
    let theta3 = 2.0 * (innerA + (sqr(innerA) + sqr(innerB) + sqr(innerC).sqrt()) / (innerB - innerC));
}

fn sqr(num: f32) -> f32 {
    num * num
}
// add option to return the same thing
fn degree_to_pi(degree: f32) -> f32 {
    degree / 57.29577951
}

// input angle in pi measure
fn calculate_range_x(length: f32, angle: f32) -> f32 {
    return angle.cos() * length;
}

// input angle in pi measure
fn calculate_range_y(length: f32, angle: f32) -> f32 {
    return angle.sin() * length;
}

// add option so degree doesnt get changed if its already in pi measure
fn calculate_point_from_other_point(length: f32, angle: f32, point: [f32; 2]) -> [f32; 2] {
    let a: [f32; 2] = [
        calculate_range_x(length, degree_to_pi(angle)),
        calculate_range_y(length, degree_to_pi(angle)),
    ];
    return [point[0] + a[0], point[1] + a[1]];
}
