pub struct Pattern {
    pub name: &'static str,
    pub data: &'static [&'static [bool]],
    pub width: usize,
    pub height: usize,
}

pub const GLIDER_DATA:&[&[bool]] = &[
    &[false, true, false],
    &[false, false, true],
    &[true, true, true]
];
pub const GLIDER:Pattern = Pattern {
    name: "Glider",
    data: GLIDER_DATA,
    width: 3,
    height: 3
};

pub const LWSS_DATA:&[&[bool]] = &[
    &[false, true, true, true, true],
    &[true, false, false, false, true],
    &[false, false, false, false, true],
    &[false, true, false, true, false]
];
pub const LWSS:Pattern = Pattern {
    name: "Lightweight Spaceship",
    data: LWSS_DATA,
    width: 5,
    height: 4
};

pub const HWSS_DATA:&[&[bool]] = &[
    &[false, true, true, true, true, true],
    &[true, false, false, false, false, true],
    &[false, false, false, false, false, true],
    &[false, false, false, false, true, false],
    &[false, false, true, false, false, false]
];
pub const HWSS:Pattern = Pattern {
    name: "Heavyweight Spaceship",
    data: HWSS_DATA,
    width: 6,
    height: 5
};

pub const PUFFER_DATA:&[&[bool]] = &[
    &[true, false, false, true, false],
    &[true, true, false, true, true,],
    &[true, false, true, false, true],
    &[false, true, true, false, false]
];
pub const PUFFER:Pattern = Pattern {
    name: "Puffer",
    data: PUFFER_DATA,
    width: 5,
    height: 4
};

pub const BLOCK_DATA:&[&[bool]] = &[
    &[true, true],
    &[true, true]
];
pub const BLOCK:Pattern = Pattern {
    name: "Block",
    data: BLOCK_DATA,
    width: 2,
    height: 2
};

pub const BLINKER_DATA:&[&[bool]] = &[
    &[true, true, true],
];
pub const BLINKER:Pattern = Pattern {
    name: "Blinker",
    data: BLINKER_DATA,
    width: 3,
    height: 1
};

pub const PENTOMINO_DATA:&[&[bool]] = &[
    &[false, true, true],
    &[true, true, false],
    &[false, true, false]
];
pub const PENTOMINO:Pattern = Pattern {
    name: "Pentomino",
    data: PENTOMINO_DATA,
    width: 3,
    height: 3
};

pub const PATTERNS:&[Pattern] = &[GLIDER, LWSS, HWSS, PUFFER, BLOCK, BLINKER, PENTOMINO];