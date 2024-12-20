pub fn symmetric_point(p: [i32; 2], q: [i32; 2]) -> [i32; 2] {
    let x: i32 = 2*q[0] - p[0];
    let y: i32 = 2*q[1] - p[1];

    let result = [x, y];
    result
}