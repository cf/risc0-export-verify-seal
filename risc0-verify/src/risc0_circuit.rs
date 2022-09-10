use crate::fp::Fp;
use crate::fp4::Fp4;
use crate::poly_op::PolyOp;
use crate::read_iop::ReadIOP;
use crate::risc0_poly_ops::{RISC0_CONS, RISC0_FP4S, RISC0_POLY_OPS};
use crate::risc0_taps::RISCV_TAPS;
use crate::sha::Digest;
use crate::taps::Taps;
use crate::verify::Circuit;
use std::slice;

const OUTPUT_REGS: usize = 9;
const ACCUM_MIX_SIZE: usize = 20;

pub struct Risc0Circuit {
    po2: u32,
    globals: Vec<Fp>,
}

impl Default for Risc0Circuit {
    fn default() -> Self {
        Risc0Circuit {
            po2: 0,
            globals: vec![],
        }
    }
}

#[derive(Clone, Copy, Default)]
struct MixState {
    tot: Fp4,
    mul: Fp4,
}

impl MixState {
    pub fn assert_zero(self, val: Fp4, mix: Fp4) -> MixState {
        MixState {
            tot: self.tot + self.mul * val,
            mul: self.mul * mix,
        }
    }
    pub fn combine(self, cond: Fp4, inner: MixState) -> MixState {
        MixState {
            tot: self.tot + cond * self.mul * inner.tot,
            mul: self.mul * inner.mul,
        }
    }
}

impl Circuit for Risc0Circuit {
    fn taps(&self) -> &'static Taps {
        return &*RISCV_TAPS;
    }
    fn execute(&mut self, iop: &mut ReadIOP) {
        for _ in 0..OUTPUT_REGS {
            let mut reg: u32 = 0;
            iop.read_u32s(slice::from_mut(&mut reg));
            self.globals.push(Fp::from(reg & 0xffff));
            self.globals.push(Fp::from(reg >> 16));
        }
        iop.read_u32s(slice::from_mut(&mut self.po2));
    }
    fn accumulate(&mut self, iop: &mut ReadIOP) {
        for _ in 0..ACCUM_MIX_SIZE {
            self.globals.push(Fp::random(iop));
        }
    }
    fn po2(&self) -> u32 {
        self.po2
    }
    fn check_code(&self, root: &Digest) {}
    fn compute_polynomial(&self, u: &[Fp4], mix: Fp4) -> Fp4 {
        let mut fps = vec![Fp4::default(); RISC0_FP4S];
        let mut cons = vec![MixState::default(); RISC0_CONS];
        let mut result = MixState::default();
        for op in &*RISC0_POLY_OPS {
            match *op {
                PolyOp::Const { out, val } => fps[out] = Fp4::from(val),
                PolyOp::Get { out, idx } => fps[out] = u[idx],
                PolyOp::GetGlobal { out, idx } => fps[out] = Fp4::from(self.globals[idx]),
                PolyOp::Begin { out } => {
                    cons[out] = MixState {
                        tot: Fp4::from(0),
                        mul: Fp4::from(1),
                    }
                }
                PolyOp::AssertZero { out, orig, val } => {
                    cons[out] = cons[orig].assert_zero(fps[val], mix)
                }
                PolyOp::Combine {
                    out,
                    orig,
                    cond,
                    inner,
                } => cons[out] = cons[orig].combine(fps[cond], cons[inner]),
                PolyOp::Add { out, a, b } => fps[out] = fps[a] + fps[b],
                PolyOp::Sub { out, a, b } => fps[out] = fps[a] - fps[b],
                PolyOp::Mul { out, a, b } => fps[out] = fps[a] * fps[b],
                PolyOp::Result { val } => result = cons[val],
            }
        }
        result.tot
    }
}
