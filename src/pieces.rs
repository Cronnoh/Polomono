#![allow(non_snake_case)]

pub type PieceType = [Vec<(i32, i32)>; 4];

pub struct PieceList {
    pub I_type: PieceType,
    pub T_type: PieceType,
}

impl PieceList {
    pub fn new() -> Self {
        let I_type = [
            vec!((0,0), (0,1), (0,2), (0,3)),
            vec!((0,2), (1,2), (2,2), (3,2)),
            vec!((1,0), (1,1), (1,2), (1,3)),
            vec!((0,1), (1,1), (2,1), (3,1)),
        ];

        let T_type = [
            vec!((0,1), (1,0), (1,1), (1,2)),
            vec!((0,1), (1,1), (1,2), (2,1)),
            vec!((1,0), (1,1), (1,2), (2,1)),
            vec!((0,1), (1,0), (1,1), (2,1)),
        ];

        Self {
            I_type,
            T_type,
        }
    }
}
