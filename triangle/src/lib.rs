enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
    Degenerate
}

pub struct Triangle {
    triangle_type: TriangleType
}

impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Self, &'static str> {
        if !((sides[0] > 0) && (sides[1] > 0) && (sides[2] > 0)) {
            return Err("oops")
        }

        if !((sides[0] <= (sides[1] + sides[2])) &&
            (sides[1] <= (sides[0] + sides[2])) &&
            (sides[2] <= (sides[1] + sides[0]))) {
                return Err("oops")
            }

        if (sides[0] == sides[1]) && (sides[1] == sides[2]) {
            Ok(Triangle { triangle_type: TriangleType::Equilateral })
        } else if (sides[0] == sides[1]) || (sides[2] == sides[1]) || (sides[2] == sides[0]) {
            Ok(Triangle { triangle_type: TriangleType::Isosceles })
        } else if (sides[0] == (sides[1] + sides[2])) ||
            (sides[1] == (sides[0] + sides[2])) ||
                    (sides[2] == (sides[1] + sides[0])) {
            Ok(Triangle { triangle_type: TriangleType::Degenerate })
        } else {
            Ok(Triangle { triangle_type: TriangleType::Scalene })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        match self.triangle_type {
            TriangleType::Equilateral => true,
            _                         => false
        }
    }

    pub fn is_scalene(&self) -> bool {
        match self.triangle_type {
            TriangleType::Scalene  => true,
            _                      => false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        match self.triangle_type {
            TriangleType::Isosceles  => true,
            _                        => false
        }
    }
}
