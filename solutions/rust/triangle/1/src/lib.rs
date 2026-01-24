pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        // reject zero-length sides (common requirement)
        if sides.iter().any(|&s| s == 0) {
            return None;
        }

        // sort so sides[2] is the largest
        sides.sort();

        // strict triangle inequality
        if sides[0] + sides[1] > sides[2] {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || a == c || b == c
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a != b && a != c && b != c
    }
}
