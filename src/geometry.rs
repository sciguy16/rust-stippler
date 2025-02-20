use nalgebra::Matrix1x3;
use voronoi::Point as VoronoiPoint;
pub type Point = [f64; 2];
pub type Pixel = [i32; 2];
pub struct OrderedPolygon {
    pub vertices: Vec<[f64; 2]>,
}

pub struct UnorderedPolygon {
    pub vertices: Vec<[f64; 2]>,
}

pub struct Line {
    pub points: [Point; 2],
}

//return the nearest pixel as the floor of each floating point coordinate
pub fn nearest_pixel(point: &VoronoiPoint) -> Pixel {
    [point.x.round() as i32, point.y.round() as i32]
}

pub fn distance(a: &Pixel, b: &Pixel) -> f64 {
    let _x = (b[0] - a[0]).pow(2) as f64;
    let _y = (b[1] - a[1]).pow(2) as f64;

    (_x + _y).sqrt()
}

pub fn vertex_centroid(points: &Vec<[f64; 2]>) -> [f64; 2] {
    let mut x = 0.0;
    let mut y = 0.0;
    let n = points.len();
    for point in points {
        x += point[0];
        y += point[1];
    }

    x /= n as f64;
    y /= n as f64;

    [x, y]
}

impl OrderedPolygon {
    pub fn create_edges(&self) -> Vec<Line> {
        let n = self.vertices.len();
        let mut lines = Vec::new();
        for i in 0..(n - 1) {
            lines.push(Line {
                points: [self.vertices[i], self.vertices[i + 1]],
            })
        }
        lines.push(Line {
            points: [self.vertices[n - 1], self.vertices[0]],
        });

        lines
    }
}

impl UnorderedPolygon {
    pub fn from_face(face: &[VoronoiPoint]) -> Self {
        //map the ordered floats into normal floats. Probably not ideal
        let vertices = face
            .iter()
            .map(|x| [f64::try_from(x.x).unwrap(), f64::try_from(x.y).unwrap()])
            .collect();
        Self { vertices }
    }

    pub fn sort(&mut self) -> OrderedPolygon {
        let mut sorted = Vec::new();
        let centroid = vertex_centroid(&self.vertices);
        let mut x;
        let mut y;
        let mut v_b;
        let mut numerator;
        let mut denominator;
        let mut theta;
        let mut _s;
        let mut sign;
        // let sorted_verts;

        let v_a = Matrix1x3::new(centroid[0] + 300.0, centroid[1], 0.0);
        for vert in &self.vertices {
            x = vert[0];
            y = vert[1];
            v_b = Matrix1x3::new(x - centroid[0], y - centroid[1], 0.0);

            numerator = v_a.dot(&v_b);
            denominator = v_a.magnitude() * v_b.magnitude();

            theta = (numerator / denominator).acos();
            _s = v_a.cross(&v_b);
            sign = _s / _s.magnitude();
            theta *= sign[2];
            sorted.push((vert, theta));
        }
        // let mut sorted = self.vertices.clone();
        radsort::sort_by_key(&mut sorted, |k| k.1);
        let verts = sorted.iter().map(|x| *x.0).collect::<Vec<[f64; 2]>>();

        OrderedPolygon { vertices: verts }
    }
}

impl Line {
    pub fn from_nodes(nodes: &[Pixel]) -> Self {
        //assumes exactly 2 nodes
        Self {
            points: [
                [nodes[0][0] as f64, nodes[0][1] as f64],
                [nodes[1][0] as f64, nodes[1][1] as f64],
            ],
        }
    }

    pub fn line_intersection(&self, other: &Line) -> Pixel {
        let x1 = self.points[0][0];
        let y1 = self.points[0][1];
        let x2 = self.points[1][0];
        let y2 = self.points[1][1];

        let x3 = other.points[0][0];
        let y3 = other.points[0][1];
        let x4 = other.points[1][0];
        let y4 = other.points[1][1];

        let p_x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let p_x_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let p_y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        let p_y_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let p_x = p_x_num / p_x_denom;
        let p_y = p_y_num / p_y_denom;
        let intersect_point = VoronoiPoint::new(p_x, p_y);

        nearest_pixel(&intersect_point)
    }
}
