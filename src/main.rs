fn main() {
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
fn calculate_point_b(length: f32, angle: f32, point: [f32; 2]) -> [f32; 2] {
    let x: [f32; 2] = [
        calculate_range_x(length, degree_to_pi(angle)),
        calculate_range_y(length, degree_to_pi(angle)),
    ];
    return [point[0] + x[0], point[1] + x[1]];
}

// use std::f32::consts::PI;
