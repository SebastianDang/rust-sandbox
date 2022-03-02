use super::geometry::*;

/// Calculates the intersection point for 2 lines
pub fn collide_line_quad(line: &Line2d, quad: &Quad2d) -> bool {
    let top = collide_line_line(
        line,
        &Line2d::from_points(quad.top_left(), quad.top_right()),
    );

    let bottom = collide_line_line(
        line,
        &Line2d::from_points(quad.bottom_left(), quad.bottom_right()),
    );

    let left = collide_line_line(
        line,
        &Line2d::from_points(quad.top_left(), quad.bottom_left()),
    );

    let right = collide_line_line(
        line,
        &Line2d::from_points(quad.top_right(), quad.bottom_right()),
    );

    top.is_some() || bottom.is_some() || left.is_some() || right.is_some()
}

/// Calculates the intersection point for 2 lines
pub fn collide_line_line(line_a: &Line2d, line_b: &Line2d) -> Option<Point2d> {
    collide_segment_segment(
        line_a.p0.x,
        line_a.p0.y,
        line_a.p1.x,
        line_a.p1.y,
        line_b.p0.x,
        line_b.p0.y,
        line_b.p1.x,
        line_b.p1.y,
    )
}

/// Calculates the intersection point for 2 lines by their points
/// Input:
///   Line A: (x1, y1) to (x2, y2)
///   Line B: (x3, y3) to (x4, y4)
/// Output:
///   Point: (x, y)
pub fn collide_segment_segment(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<Point2d> {
    // calculate the distance to intersection point
    let num_a = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let den_a = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let u_a = num_a / den_a;

    let num_b = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
    let den_b = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let u_b = num_b / den_b;

    // if u_a and u_b are between 0.0 and 1.0, lines are colliding
    if u_a >= 0.0 && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0 {
        let x = x1 + (u_a * (x2 - x1));
        let y = y1 + (u_a * (y2 - y1));
        Some(Point2d::new(x, y))
    } else {
        None
    }
}
