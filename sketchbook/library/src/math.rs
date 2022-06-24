use super::points::Point;
use nannou::prelude::*;

pub fn same_sign(a: f32, b: f32) -> bool {
    return a * b >= 0.0;
}

pub fn intersects_line(
    line1_p1: Point2,
    line1_p2: Point2,
    line2_p1: Point2,
    line2_p2: Point2,
) -> bool {
    let x1 = line1_p1.x;
    let y1 = line1_p1.y;

    let x2 = line1_p2.x;
    let y2 = line1_p2.y;

    let x3 = line2_p1.x;
    let y3 = line2_p1.y;

    let x4 = line2_p2.x;
    let y4 = line2_p2.y;

    let (mut a1, mut a2, mut b1, mut b2, mut c1, mut c2);
    let (mut r1, mut r2, mut r3, mut r4);

    // Compute a1, b1, c1, where line joining points 1 and 2
    // is "a1 x + b1 y + c1 = 0".
    a1 = y2 - y1;
    b1 = x1 - x2;
    c1 = (x2 * y1) - (x1 * y2);

    // Compute r3 and r4.
    r3 = (a1 * x3) + (b1 * y3) + c1;
    r4 = (a1 * x4) + (b1 * y4) + c1;

    //check signs of r3 and r4. If both point 3 and point 4 lie on
    //same side of line 1, the line segments do not intersect.
    if r3 != 0.0 && r4 != 0.0 && same_sign(r3, r4) {
        return false;
    }

    // Compute a2, b2, c2
    a2 = y4 - y3;
    b2 = x3 - x4;
    c2 = (x4 * y3) - (x3 * y4);

    // Compute r1 and r2
    r1 = (a2 * x1) + (b2 * y1) + c2;
    r2 = (a2 * x2) + (b2 * y2) + c2;

    // check signs of r1 and r2 to see if both point 1 and point 2 lie
    // on same side of second line segment, the line segments do not intersect
    if r1 != 0.0 && r2 != 0.0 && same_sign(r1, r2) {
        return false;
    }

    return true;
}

/*
int intersect(float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4){

  float a1, a2, b1, b2, c1, c2;
  float r1, r2 , r3, r4;
  float denom, offset, num;

  // Compute a1, b1, c1, where line joining points 1 and 2
  // is "a1 x + b1 y + c1 = 0".
  a1 = y2 - y1;
  b1 = x1 - x2;
  c1 = (x2 * y1) - (x1 * y2);

  // Compute r3 and r4.
  r3 = ((a1 * x3) + (b1 * y3) + c1);
  r4 = ((a1 * x4) + (b1 * y4) + c1);

  // Check signs of r3 and r4. If both point 3 and point 4 lie on
  // same side of line 1, the line segments do not intersect.
  if ((r3 != 0) && (r4 != 0) && same_sign(r3, r4)){
    return DONT_INTERSECT;
  }

  // Compute a2, b2, c2
  a2 = y4 - y3;
  b2 = x3 - x4;
  c2 = (x4 * y3) - (x3 * y4);

  // Compute r1 and r2
  r1 = (a2 * x1) + (b2 * y1) + c2;
  r2 = (a2 * x2) + (b2 * y2) + c2;

  // Check signs of r1 and r2. If both point 1 and point 2 lie
  // on same side of second line segment, the line segments do
  // not intersect.
  if ((r1 != 0) && (r2 != 0) && (same_sign(r1, r2))){
    return DONT_INTERSECT;
  }

  //Line segments intersect: compute intersection point.
  denom = (a1 * b2) - (a2 * b1);

  if (denom == 0) {
    return COLLINEAR;
  }

  if (denom < 0){
    offset = -denom / 2;
  }
  else {
    offset = denom / 2 ;
  }

  // The denom/2 is to get rounding instead of truncating. It
  // is added or subtracted to the numerator, depending upon the
  // sign of the numerator.
  num = (b1 * c2) - (b2 * c1);
  if (num < 0){
    x = (num - offset) / denom;
  }
  else {
    x = (num + offset) / denom;
  }

  num = (a2 * c1) - (a1 * c2);
  if (num < 0){
    y = ( num - offset) / denom;
  }
  else {
    y = (num + offset) / denom;
  }

  // lines_intersect
  return DO_INTERSECT;
}
*/

//translated from js:http://jsfiddle.net/mmansion/2hqf1kgv/6/
pub fn point_inside_triangle(pt: Point2, A: Point2, B: Point2, C: Point2) -> bool {
    let a = 0.5 * (-B.y * C.x + A.y * (-B.x + C.x) + A.x * (B.y - C.y) + B.x * C.y);
    let sign = match () {
        _ if a < 0.0 => -1.0,
        _ => 1.0,
    };
    let s = (A.y * C.x - A.x * C.y + (C.y - A.y) * pt.x + (A.x - C.x) * pt.y) * sign;
    let t = (A.x * B.y - A.y * B.x + (A.y - B.y) * pt.x + (B.x - A.x) * pt.y) * sign;

    return s > 0.0 && t > 0.0 && (s+t) < 2.0 * a * sign;
}