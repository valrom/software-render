use crate::math::vectors::{Vector2, Vector3, Vector4};

#[derive(Copy, Clone)]
pub struct Rect2 {
    start: Vector2<f32>,
    end: Vector2<f32>,
}

impl Rect2 {
    pub fn new(start: Vector2<f32>, end: Vector2<f32>) -> Option<Self> {
        let diff = end - start;

        if diff.x.abs() < 1.0 || diff.y.abs() < 1.0 {
            return None;
        }

        Some(Self { start, end })
    }

    pub fn start(&self) -> Vector2<f32> {
        self.start
    }

    pub fn end(&self) -> Vector2<f32> {
        self.end
    }
}

impl IntoIterator for Rect2 {
    type Item = Vector2<i32>;

    type IntoIter = Rect2Iter;

    fn into_iter(self) -> Self::IntoIter {
        Rect2Iter::new(self)
    }
}

#[test]
fn test_rect2() {
    let _rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)).unwrap();
    let _rect2 = Rect2::new(Vector2::new(100.0, 100.0), Vector2::new(0.0, 0.0)).unwrap();
}

#[derive(Debug)]
pub struct Rect2Iter {
    start: Vector2<i32>,
    end: Vector2<i32>,
    current: Vector2<i32>,
}

impl Rect2Iter {
    pub fn new(rect: Rect2) -> Self {
        let start = rect.start();
        let end = rect.end();

        let start = Vector2::<i32>::new(start.x as i32, start.y as i32);
        let end = Vector2::<i32>::new(end.x as i32, end.y as i32);

        let (start_x, end_x) = if start.x < end.x {
            (start.x, end.x)
        } else {
            (end.x, start.x)
        };

        let (start_y, end_y) = if start.y < end.y {
            (start.y, end.y)
        } else {
            (end.y, start.y)
        };

        let start = Vector2::new(start_x, start_y);
        let end = Vector2::new(end_x, end_y);

        Self {
            start,
            end,
            current: start,
        }
    }
}

impl Iterator for Rect2Iter {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y > self.end.y {
            return None;
        }

        let result = self.current;

        self.current.x += 1;

        if self.current.x > self.end.x {
            self.current.x = self.start.x;
            self.current.y += 1;
        }

        Some(result)
    }
}

fn _test_iter() -> Rect2Iter {
    Rect2Iter::new(Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)).unwrap())
}

#[test]
fn testing_iter() {
    let mut iter = _test_iter();

    for _ in 0..=100 {
        iter.next();
    }
    assert_eq!(iter.next().unwrap(), Vector2::new(0, 1));
}

#[test]
fn testing_iter_end() {
    let mut iter = _test_iter();

    for _ in 1..=101 * 101 - 1 {
        iter.next();
    }

    assert_eq!(iter.next(), Some(Vector2::new(100, 100)));
}

type Triplet = [Vector3<f32>; 3];
type Triplet4 = [Vector4<f32>; 3];

#[derive(Copy, Clone)]
pub struct Triangle {
    vertices: Triplet,
    segments: Triplet,
    ws: Vector3<f32>,
    rect: Rect2,
}

impl Triangle {
    pub fn new(vertices: Triplet4) -> Option<Self> {
        let ws = Vector3::new(vertices[0].w, vertices[1].w, vertices[2].w);

        let vertices: [Vector3<f32>; 3] =
            [vertices[0].into(), vertices[1].into(), vertices[2].into()];

        let segments = Self::segments(&vertices);

        let rect = Self::get_rect(&vertices)?;

        if Self::check(&vertices, &segments) {
            Some(Self {
                vertices,
                segments,
                ws,
                rect,
            })
        } else {
            None
        }
    }

    fn segments(vertices: &Triplet) -> Triplet {
        let p1 = vertices[1] - vertices[0];
        let p2 = vertices[2] - vertices[1];
        let p3 = vertices[0] - vertices[2];

        [p1, p2, p3]
    }

    fn check(vertices: &Triplet, segments: &Triplet) -> bool {
        let middle = (vertices[0] + vertices[1] + vertices[2]) / 3.0;

        let deltas = [
            middle - vertices[0],
            middle - vertices[1],
            middle - vertices[2],
        ];

        for i in 0..3 {
            if segments[i].cross(deltas[i]).z < 0.0 {
                return false;
            }
        }

        true
    }

    fn get_rect(vertices: &Triplet) -> Option<Rect2> {
        let (max_x, min_x) = get_max_min([vertices[0].x, vertices[1].x, vertices[2].x]);
        let (max_y, min_y) = get_max_min([vertices[0].y, vertices[1].y, vertices[2].y]);

        let start = Vector2::new(min_x, min_y);
        let end = Vector2::new(max_x, max_y);

        return Rect2::new(start, end);

        fn get_max_min(coords: [f32; 3]) -> (f32, f32) {
            if coords[0] > coords[1] && coords[0] > coords[2] {
                if coords[1] > coords[2] {
                    return (coords[0], coords[2]);
                }
                return (coords[0], coords[1]);
            } else if coords[1] > coords[0] && coords[1] > coords[2] {
                if coords[0] > coords[2] {
                    return (coords[1], coords[2]);
                }
                return (coords[1], coords[0]);
            } else {
                if coords[0] > coords[1] {
                    return (coords[2], coords[1]);
                }
                return (coords[2], coords[0]);
            }
        }
    }
}

impl IntoIterator for Triangle {
    type Item = Fragment;

    type IntoIter = TriangleIter;

    fn into_iter(self) -> Self::IntoIter {
        TriangleIter::new(self)
    }
}

#[test]
fn test_triangle() {
    let _triangle = Triangle::new([
        Vector3::new(0.0, 0.0, 0.0).into(),
        Vector3::new(100.0, 0.0, 0.0).into(),
        Vector3::new(0.0, 100.0, 0.0).into(),
    ])
    .unwrap();

    let _triangle = Triangle::new([
        Vector3::new(100.0, 100.0, 0.0).into(),
        Vector3::new(50.0, 50.0, 0.0).into(),
        Vector3::new(100.0, 0.0, 0.0).into(),
    ])
    .unwrap();
}

#[test]
#[should_panic]
fn test_triangle_should_failed() {
    let _ = Triangle::new([
        Vector3::new(0.0, 0.0, 0.0).into(),
        Vector3::new(0.0, 100.0, 0.0).into(),
        Vector3::new(100.0, 0.0, 0.0).into(),
    ])
    .unwrap();
}

struct LinearInterpolator {
    start: f32,
    dx: f32,
    dy: f32,
}

impl LinearInterpolator {
    fn new(start: f32, dx: f32, dy: f32) -> Self {
        Self { start, dx, dy }
    }

    fn calc(&self, dx: f32, dy: f32) -> f32 {
        self.start + self.dx * dx + self.dy * dy
    }
}

#[test]
fn test_linear_interpolator() {
    let first = Vector3::<f32>::new(100.0, 0.0, 0.0);
    let second = Vector3::new(0.0, 100.0, 0.0);

    let cross = first.cross(second).z;

    let linear = LinearInterpolator::new(cross, -first.y, first.x);

    assert_eq!(
        linear.calc(50.0, 50.0),
        first.cross(second + Vector3::new(50.0, 50.0, 0.0)).z
    );
}

pub struct TriangleIter {
    rect_iter: Rect2Iter,
    crosses: [LinearInterpolator; 3],
    start: Vector2<f32>,
    triangle: Triangle,
    zs: Vector3<f32>,
}

impl TriangleIter {
    pub fn new(triangle: Triangle) -> Self {
        let rect = triangle.rect;
        let rect_iter = Rect2Iter::new(rect);

        let start = rect.start();
        let vs = [
            start - triangle.vertices[0].into(),
            start - triangle.vertices[1].into(),
            start - triangle.vertices[2].into(),
        ];

        let crosses = [
            LinearInterpolator::new(
                triangle.segments[0].cross(vs[0].into()).z,
                -triangle.segments[0].y,
                triangle.segments[0].x,
            ),
            LinearInterpolator::new(
                triangle.segments[1].cross(vs[1].into()).z,
                -triangle.segments[1].y,
                triangle.segments[1].x,
            ),
            LinearInterpolator::new(
                triangle.segments[2].cross(vs[2].into()).z,
                -triangle.segments[2].y,
                triangle.segments[2].x,
            ),
        ];
        Self {
            rect_iter,
            crosses,
            start,
            triangle,
            zs: Vector3::new(
                triangle.vertices[0].z,
                triangle.vertices[1].z,
                triangle.vertices[2].z,
            ),
        }
    }
}

pub struct Fragment {
    pub position: Vector3<f32>,
    pub coefs: Vector3<f32>,
}

impl Iterator for TriangleIter {
    type Item = Fragment;

    fn next(&mut self) -> Option<Fragment> {
        self.rect_iter.find_map(|position| {
            let position = Vector2::<f32>::new(position.x as f32, position.y as f32);
            let delta = position - self.start;

            let cof2 = self.crosses[0].calc(delta.x, delta.y);
            let cof0 = self.crosses[1].calc(delta.x, delta.y);
            let cof1 = self.crosses[2].calc(delta.x, delta.y);

            let mut coefs = Vector3::new(cof0, cof1, cof2);

            if cof0 > 0.0 && cof1 > 0.0 && cof2 > 0.0 {
                coefs.x /= self.triangle.ws.x;
                coefs.y /= self.triangle.ws.y;
                coefs.z /= self.triangle.ws.z;

                let sum = coefs.x + coefs.y + coefs.z;

                coefs = coefs / sum;

                let z = coefs * self.zs;

                Some(Fragment {
                    position: Vector3::<f32>::new(position.x, position.y, z),
                    coefs,
                })
            } else {
                None
            }
        })
    }
}
