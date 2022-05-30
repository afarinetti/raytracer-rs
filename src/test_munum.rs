use munum::{vec4, Vec4};

fn point(x: f32, y: f32, z: f32) -> Vec4 {
    return vec4(x, y, z, 1.0);
}

fn vector(x: f32, y: f32, z: f32) -> Vec4 {
    return vec4(x, y, z, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let x: f32 =  4.3;
        let y: f32 = -4.2;
        let z: f32 =  3.1;
        let w: f32 =  1.0;

        let a: Vec4 = point(x, y, z);

        assert_eq!(*a.as_ref(), [x, y, z, w]);
    }

    #[test]
    fn test_vector() {
        let x: f32 =  4.3;
        let y: f32 = -4.2;
        let z: f32 =  3.1;
        let w: f32 =  0.0;

        let a: Vec4 = vector(x, y, z);

        assert_eq!(*a.as_ref(), [x, y, z, w]);
    }
}
