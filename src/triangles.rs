use std::cmp::{max, min};

use crate::math::{RectI2, RectI2Iter, VecI2};

type TripletI2 = (VecI2, VecI2, VecI2);

#[derive(Clone, Copy)]
pub struct RasterTriangle {
    pub vertices: TripletI2,
    pub segments: TripletI2,
}

impl RasterTriangle {
    pub fn new(first: VecI2, second: VecI2, third: VecI2) -> Option<Self> {
        let vertices = (first, second, third);
        let segments = Self::segments_from_points(vertices);

        if Self::check_vertices(vertices, segments) {
            Some(Self { segments, vertices })
        } else {
            None
        }
    }

    fn check_vertices(vertices: TripletI2, segments: TripletI2) -> bool {
        let middle = (vertices.0 + vertices.1 + vertices.2).scale(1.0 / 3.0);
        let deltas = (
            middle - vertices.0,
            middle - vertices.1,
            middle - vertices.2,
        );

        segments.0.cross(deltas.0) > 0.0
            && segments.1.cross(deltas.1) > 0.0
            && segments.2.cross(deltas.2) > 0.0
    }

    fn segments_from_points(points: TripletI2) -> TripletI2 {
        let p1 = points.1 - points.0;
        let p2 = points.2 - points.1;
        let p3 = points.0 - points.2;

        (p1, p2, p3)
    }
}

impl TryFrom<TripletI2> for RasterTriangle {
    type Error = ();

    fn try_from(vertices: TripletI2) -> Result<Self, Self::Error> {
        Self::new(vertices.0, vertices.1, vertices.2).ok_or(())
    }
}

#[test]
fn test_triangle_creation() {
    let _: RasterTriangle = ((0, 0).into(), (100, 0).into(), (50, 50).into())
        .try_into()
        .unwrap();
}

#[test]
#[should_panic]
fn test_triangle_creation_failure() {
    let _: RasterTriangle = ((0, 0).into(), (0, 100).into(), (50, 50).into())
        .try_into()
        .unwrap();
}

pub struct Fragment {
    pub position: VecI2,
    pub attributes: (f32, f32, f32),
}

pub struct Rasterizer {
    iter: RectI2Iter,

    area: f32,
    starts: (i32, i32, i32),
    deltas: (VecI2, VecI2, VecI2),
}

impl Iterator for Rasterizer {
    type Item = Fragment;

    fn next(&mut self) -> Option<Fragment> {
        let start = self.iter.rect().start();

        self.iter.find_map(|position| {
            let delta = position - start;

            let cof2 = self.starts.0 + delta.x * self.deltas.0.x + delta.y * self.deltas.0.y;

            let cof0 = self.starts.1 + delta.x * self.deltas.1.x + delta.y * self.deltas.1.y;

            let cof1 = self.starts.2 + delta.x * self.deltas.2.x + delta.y * self.deltas.2.y;

            if cof0 >= 0 && cof1 >= 0 && cof2 >= 0 {
                Some(Fragment {
                    position,
                    attributes: (
                        cof0 as f32 / self.area,
                        cof1 as f32 / self.area,
                        cof2 as f32 / self.area,
                    ),
                })
            } else {
                None
            }
        })
    }
}

impl Rasterizer {
    #[allow(dead_code)]
    pub fn new(triangle: RasterTriangle) -> Option<Self> {
        let rect = Rasterizer::get_area(&triangle)?;

        let starts = Self::calculate_starts(&triangle, rect.start());

        let iter = rect.into_iter();

        let area = triangle.segments.0.cross(triangle.segments.1).abs();

        Some(Self {
            iter,
            starts,
            deltas: Self::calculate_deltas(&triangle),
            area,
        })
    }

    pub fn new_with_cropping(triangle: RasterTriangle, crop_area: VecI2) -> Option<Self> {
        let rect = Rasterizer::get_area(&triangle)?;

        let (start, end) = (rect.start(), rect.end());

        use std::cmp::max;

        let min_x = max(start.x, 0);
        let min_y = max(start.y, 0);

        use std::cmp::min;

        let max_x = min(end.x, crop_area.x);
        let max_y = min(end.y, crop_area.y);

        let start = (min_x, min_y).into();
        let end = (max_x, max_y).into();

        let starts = Self::calculate_starts(&triangle, start);

        let iter = RectI2::new(start, end)?.into_iter();

        let area = triangle.segments.0.cross(triangle.segments.1).abs();

        Some(Self {
            iter,
            starts,
            deltas: Self::calculate_deltas(&triangle),
            area,
        })
    }

    fn get_area(triangle: &RasterTriangle) -> Option<RectI2> {
        let first = triangle.vertices.0;
        let second = triangle.vertices.1;
        let third = triangle.vertices.2;

        let (min_x, max_x) = min_max_from_three(first.x, second.x, third.x);
        let (min_y, max_y) = min_max_from_three(first.y, second.y, third.y);

        let start = VecI2::new(min_x, min_y);
        let end = VecI2::new(max_x, max_y);

        (start, end).try_into().ok()
    }

    fn calculate_starts(triangle: &RasterTriangle, start: VecI2) -> (i32, i32, i32) {
        let vs = (
            start - triangle.vertices.0,
            start - triangle.vertices.1,
            start - triangle.vertices.2,
        );

        (
            triangle.segments.0.cross(vs.0) as i32,
            triangle.segments.1.cross(vs.1) as i32,
            triangle.segments.2.cross(vs.2) as i32,
        )
    }

    fn calculate_deltas(triangle: &RasterTriangle) -> (VecI2, VecI2, VecI2) {
        (
            (-triangle.segments.0.y, triangle.segments.0.x).into(),
            (-triangle.segments.1.y, triangle.segments.1.x).into(),
            (-triangle.segments.2.y, triangle.segments.2.x).into(),
        )
    }
}

fn min_max_from_three(first: i32, second: i32, third: i32) -> (i32, i32) {
    let min_first = min(first, second);
    let min_second = min(min_first, third);

    let max_first = max(first, second);
    let max_second = max(max_first, third);

    (min_second, max_second)
}

#[test]
fn math_teory() {
    let first = VecI2::new(10, 10);
    let second = VecI2::new(10, 0);

    let initial = first.cross(second) as i32;

    let predicted_x = initial - first.y;

    let predicted_y = initial + first.x;

    assert_eq!(first.cross(second + (1, 0).into()) as i32, predicted_x);
    assert_eq!(first.cross(second + (0, 1).into()) as i32, predicted_y);
    assert_eq!(
        first.cross(second + (1, 1).into()) as i32,
        initial + first.x - first.y
    );
}
