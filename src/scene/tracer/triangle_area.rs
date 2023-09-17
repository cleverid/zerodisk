use crate::primitive::Triangle;

/// Рассчитывает площадь треугольника по координатам
pub fn triangle_area(triangle: Triangle) -> f32 {
    let [p1, p2, p3] = triangle.points;
    ((p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y)).abs() as f32 / 2.0
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitive::triangle;

    #[test]
    fn right_triangle() {
        let triangle = triangle([[0, 0], [3, 0], [0, 4]]);
        let area = triangle_area(triangle);
        assert_eq!(area, 6.0);
    }

    #[test]
    fn eq_side_triangle() {
        let triangle = triangle([[0, 2], [-1, 0], [1, 0]]);
        let area = triangle_area(triangle);
        assert_eq!(area, 2.0);
    }
}
