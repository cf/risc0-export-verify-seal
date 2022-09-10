#[derive(Clone, Copy)]
pub enum RegisterGroup {
    Accum = 0,
    Code = 1,
    Data = 2,
}

pub struct Register {
    pub group: RegisterGroup,
    pub offset: usize,
    pub back: Vec<usize>,
    pub combo_id: usize,
}

pub struct Combo {
    pub back: Vec<usize>,
}

pub struct Taps {
    pub registers: Vec<Register>,
    pub combos: Vec<Combo>,
}
