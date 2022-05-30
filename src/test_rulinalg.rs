use rulinalg::vector::Vector;

fn point(x: f32, y: f32, z: f32) -> Vector<f32> {
    return Vector::new(vec![x, y, z, 1.0]);
}

fn vector(x: f32, y: f32, z: f32) -> Vector<f32> {
    return Vector::new(vec![x, y, z, 0.0]);
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

        let a: Vector<f32> = point(x, y, z);

        assert_eq!(a.into_vec(), [x, y, z, w]);
    }

    #[test]
    fn test_vector() {
        let x: f32 =  4.3;
        let y: f32 = -4.2;
        let z: f32 =  3.1;
        let w: f32 =  0.0;

        let a: Vector<f32> = vector(x, y, z);

        assert_eq!(a.into_vec(), [x, y, z, w]);
    }
}
