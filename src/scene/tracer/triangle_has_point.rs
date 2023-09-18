use crate::primitive::{Point, Triangle};

use super::triangle_area::triangle_area;

/// Проверяет попадание точки в треугольник
pub fn triangle_has_point(triangle: Triangle, point: Point) -> bool {
    let [p1, p2, p3] = triangle.points;
    let area = triangle_area(triangle);
    let area_1_2 = triangle_area(Triangle {
        points: [p1, p2, point],
    });
    let area_2_3 = triangle_area(Triangle {
        points: [p2, p3, point],
    });
    let area_1_3 = triangle_area(Triangle {
        points: [p1, p3, point],
    });

    area_1_2 + area_2_3 + area_1_3 - area <= 0.00001
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitive::{point, triangle};

    #[test]
    fn in_triangle() {
        let triangle = triangle([[0, 0], [3, 0], [0, 4]]);
        let point = point(1, 1);
        let result = triangle_has_point(triangle, point);
        assert_eq!(result, true);
    }

    #[test]
    fn in_point() {
        let triangle = triangle([[0, 0], [3, 0], [0, 4]]);
        let point = point(0, 0);
        let result = triangle_has_point(triangle, point);
        assert_eq!(result, true);
    }

    #[test]
    fn in_edge() {
        let triangle = triangle([[0, 0], [3, 0], [0, 4]]);
        let point = point(0, 1);
        let result = triangle_has_point(triangle, point);
        assert_eq!(result, true);
    }

    #[test]
    fn out() {
        let triangle = triangle([[0, 0], [3, 0], [0, 4]]);
        let point = point(4, 0);
        let result = triangle_has_point(triangle, point);
        assert_eq!(result, false);
    }
}
