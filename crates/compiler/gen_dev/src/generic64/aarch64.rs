use crate::generic64::{storage::StorageManager, Assembler, CallConv, RegTrait};
use crate::Relocation;
use bumpalo::collections::Vec;
use packed_struct::prelude::*;
use roc_builtins::bitcode::FloatWidth;
use roc_error_macros::internal_error;
use roc_module::symbol::Symbol;
use roc_mono::layout::{InLayout, STLayoutInterner};

use super::{CompareOperation, RegisterWidth};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum AArch64GeneralReg {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    XR = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    IP0 = 16,
    IP1 = 17,
    PR = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    FP = 29,
    LR = 30,
    /// This can mean Zero or Stack Pointer depending on the context.
    ZRSP = 31,
}

impl RegTrait for AArch64GeneralReg {
    fn value(&self) -> u8 {
        *self as u8
    }
}
impl std::fmt::Display for AArch64GeneralReg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AArch64GeneralReg::X0 => "x0",
                AArch64GeneralReg::X1 => "x1",
                AArch64GeneralReg::X2 => "x2",
                AArch64GeneralReg::X3 => "x3",
                AArch64GeneralReg::X4 => "x4",
                AArch64GeneralReg::X5 => "x5",
                AArch64GeneralReg::X6 => "x6",
                AArch64GeneralReg::X7 => "x7",
                AArch64GeneralReg::XR => "xr",
                AArch64GeneralReg::X9 => "x9",
                AArch64GeneralReg::X10 => "x10",
                AArch64GeneralReg::X11 => "x11",
                AArch64GeneralReg::X12 => "x12",
                AArch64GeneralReg::X13 => "x13",
                AArch64GeneralReg::X14 => "x14",
                AArch64GeneralReg::X15 => "x15",
                AArch64GeneralReg::IP0 => "ip0",
                AArch64GeneralReg::IP1 => "ip1",
                AArch64GeneralReg::PR => "pr",
                AArch64GeneralReg::X19 => "x19",
                AArch64GeneralReg::X20 => "x20",
                AArch64GeneralReg::X21 => "x21",
                AArch64GeneralReg::X22 => "x22",
                AArch64GeneralReg::X23 => "x23",
                AArch64GeneralReg::X24 => "x24",
                AArch64GeneralReg::X25 => "x25",
                AArch64GeneralReg::X26 => "x26",
                AArch64GeneralReg::X27 => "x27",
                AArch64GeneralReg::X28 => "x28",
                AArch64GeneralReg::FP => "fp",
                AArch64GeneralReg::LR => "lr",
                AArch64GeneralReg::ZRSP => "zrsp",
            }
        )
    }
}

impl AArch64GeneralReg {
    #[inline(always)]
    fn id(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum AArch64FloatReg {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
    V30 = 30,
    V31 = 31,
}
impl RegTrait for AArch64FloatReg {
    fn value(&self) -> u8 {
        *self as u8
    }
}
impl std::fmt::Display for AArch64FloatReg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AArch64FloatReg::V0 => "v0",
                AArch64FloatReg::V1 => "v1",
                AArch64FloatReg::V2 => "v2",
                AArch64FloatReg::V3 => "v3",
                AArch64FloatReg::V4 => "v4",
                AArch64FloatReg::V5 => "v5",
                AArch64FloatReg::V6 => "v6",
                AArch64FloatReg::V7 => "v7",
                AArch64FloatReg::V8 => "v8",
                AArch64FloatReg::V9 => "v9",
                AArch64FloatReg::V10 => "v10",
                AArch64FloatReg::V11 => "v11",
                AArch64FloatReg::V12 => "v12",
                AArch64FloatReg::V13 => "v13",
                AArch64FloatReg::V14 => "v14",
                AArch64FloatReg::V15 => "v15",
                AArch64FloatReg::V16 => "v16",
                AArch64FloatReg::V17 => "v17",
                AArch64FloatReg::V18 => "v18",
                AArch64FloatReg::V19 => "v19",
                AArch64FloatReg::V20 => "v20",
                AArch64FloatReg::V21 => "v21",
                AArch64FloatReg::V22 => "v22",
                AArch64FloatReg::V23 => "v23",
                AArch64FloatReg::V24 => "v24",
                AArch64FloatReg::V25 => "v25",
                AArch64FloatReg::V26 => "v26",
                AArch64FloatReg::V27 => "v27",
                AArch64FloatReg::V28 => "v28",
                AArch64FloatReg::V29 => "v29",
                AArch64FloatReg::V30 => "v30",
                AArch64FloatReg::V31 => "v31",
            }
        )
    }
}

impl AArch64FloatReg {
    #[inline(always)]
    fn id(&self) -> u8 {
        *self as u8
    }
}

#[derive(Copy, Clone)]
pub struct AArch64Assembler {}

// AArch64Call may need to eventually be split by OS,
// but I think with how we use it, they may all be the same.
#[derive(Copy, Clone)]
pub struct AArch64Call {}

const STACK_ALIGNMENT: u8 = 16;

impl CallConv<AArch64GeneralReg, AArch64FloatReg, AArch64Assembler> for AArch64Call {
    const BASE_PTR_REG: AArch64GeneralReg = AArch64GeneralReg::FP;
    const STACK_PTR_REG: AArch64GeneralReg = AArch64GeneralReg::ZRSP;

    const GENERAL_PARAM_REGS: &'static [AArch64GeneralReg] = &[
        AArch64GeneralReg::X0,
        AArch64GeneralReg::X1,
        AArch64GeneralReg::X2,
        AArch64GeneralReg::X3,
        AArch64GeneralReg::X4,
        AArch64GeneralReg::X5,
        AArch64GeneralReg::X6,
        AArch64GeneralReg::X7,
    ];
    const GENERAL_RETURN_REGS: &'static [AArch64GeneralReg] = Self::GENERAL_PARAM_REGS;
    const GENERAL_DEFAULT_FREE_REGS: &'static [AArch64GeneralReg] = &[
        // The regs we want to use first should be at the end of this vec.
        // We will use pop to get which reg to use next

        // Don't use frame pointer: AArch64GeneralReg::FP,
        // Don't user indirect result location: AArch64GeneralReg::XR,
        // Don't use platform register: AArch64GeneralReg::PR,
        // Don't use link register: AArch64GeneralReg::LR,
        // Don't use zero register/stack pointer: AArch64GeneralReg::ZRSP,

        // Use callee saved regs last.
        AArch64GeneralReg::X19,
        AArch64GeneralReg::X20,
        AArch64GeneralReg::X21,
        AArch64GeneralReg::X22,
        AArch64GeneralReg::X23,
        AArch64GeneralReg::X24,
        AArch64GeneralReg::X25,
        AArch64GeneralReg::X26,
        AArch64GeneralReg::X27,
        AArch64GeneralReg::X28,
        // Use caller saved regs first.
        AArch64GeneralReg::X0,
        AArch64GeneralReg::X1,
        AArch64GeneralReg::X2,
        AArch64GeneralReg::X3,
        AArch64GeneralReg::X4,
        AArch64GeneralReg::X5,
        AArch64GeneralReg::X6,
        AArch64GeneralReg::X7,
        AArch64GeneralReg::X9,
        AArch64GeneralReg::X10,
        AArch64GeneralReg::X11,
        AArch64GeneralReg::X12,
        AArch64GeneralReg::X13,
        AArch64GeneralReg::X14,
        AArch64GeneralReg::X15,
        AArch64GeneralReg::IP0,
        AArch64GeneralReg::IP1,
    ];
    const FLOAT_PARAM_REGS: &'static [AArch64FloatReg] = &[];
    const FLOAT_RETURN_REGS: &'static [AArch64FloatReg] = Self::FLOAT_PARAM_REGS;
    const FLOAT_DEFAULT_FREE_REGS: &'static [AArch64FloatReg] = &[];

    const SHADOW_SPACE_SIZE: u8 = 0;

    #[inline(always)]
    fn general_callee_saved(reg: &AArch64GeneralReg) -> bool {
        matches!(
            reg,
            AArch64GeneralReg::X19
                | AArch64GeneralReg::X20
                | AArch64GeneralReg::X21
                | AArch64GeneralReg::X22
                | AArch64GeneralReg::X23
                | AArch64GeneralReg::X24
                | AArch64GeneralReg::X25
                | AArch64GeneralReg::X26
                | AArch64GeneralReg::X27
                | AArch64GeneralReg::X28
        )
    }
    #[inline(always)]
    fn float_callee_saved(_reg: &AArch64FloatReg) -> bool {
        todo!("AArch64 FloatRegs");
    }

    #[inline(always)]
    fn setup_stack(
        buf: &mut Vec<'_, u8>,
        saved_general_regs: &[AArch64GeneralReg],
        saved_float_regs: &[AArch64FloatReg],
        requested_stack_size: i32,
        fn_call_stack_size: i32,
    ) -> i32 {
        // Full size is upcast to i64 to make sure we don't overflow here.
        let full_stack_size = match requested_stack_size
            .checked_add(8 * (saved_general_regs.len() + saved_float_regs.len()) as i32 + 8) // The extra 8 is space to store the frame pointer.
            .and_then(|size| size.checked_add(fn_call_stack_size))
        {
            Some(size) => size,
            _ => internal_error!("Ran out of stack space"),
        };
        let alignment = if full_stack_size <= 0 {
            0
        } else {
            full_stack_size % STACK_ALIGNMENT as i32
        };
        let offset = if alignment == 0 {
            0
        } else {
            STACK_ALIGNMENT - alignment as u8
        };
        if let Some(aligned_stack_size) = full_stack_size.checked_add(offset as i32) {
            if aligned_stack_size > 0 {
                AArch64Assembler::mov_reg64_reg64(
                    buf,
                    AArch64GeneralReg::FP,
                    AArch64GeneralReg::ZRSP,
                );
                AArch64Assembler::sub_reg64_reg64_imm32(
                    buf,
                    AArch64GeneralReg::ZRSP,
                    AArch64GeneralReg::ZRSP,
                    aligned_stack_size,
                );

                // All the following stores could be optimized by using `STP` to store pairs.
                let mut offset = aligned_stack_size;
                offset -= 8;
                AArch64Assembler::mov_stack32_reg64(buf, offset, AArch64GeneralReg::LR);
                offset -= 8;
                AArch64Assembler::mov_stack32_reg64(buf, offset, AArch64GeneralReg::FP);

                offset = aligned_stack_size - fn_call_stack_size;
                for reg in saved_general_regs {
                    offset -= 8;
                    AArch64Assembler::mov_base32_reg64(buf, offset, *reg);
                }
                for reg in saved_float_regs {
                    offset -= 8;
                    AArch64Assembler::mov_base32_freg64(buf, offset, *reg);
                }
                aligned_stack_size
            } else {
                0
            }
        } else {
            internal_error!("Ran out of stack space");
        }
    }

    #[inline(always)]
    fn cleanup_stack(
        buf: &mut Vec<'_, u8>,
        saved_general_regs: &[AArch64GeneralReg],
        saved_float_regs: &[AArch64FloatReg],
        aligned_stack_size: i32,
        fn_call_stack_size: i32,
    ) {
        if aligned_stack_size > 0 {
            // All the following stores could be optimized by using `STP` to store pairs.
            let mut offset = aligned_stack_size;
            offset -= 8;
            AArch64Assembler::mov_reg64_stack32(buf, AArch64GeneralReg::LR, offset);
            offset -= 8;
            AArch64Assembler::mov_reg64_stack32(buf, AArch64GeneralReg::FP, offset);

            offset = aligned_stack_size - fn_call_stack_size;
            for reg in saved_general_regs {
                offset -= 8;
                AArch64Assembler::mov_reg64_base32(buf, *reg, offset);
            }
            for reg in saved_float_regs {
                offset -= 8;
                AArch64Assembler::mov_freg64_base32(buf, *reg, offset);
            }
            AArch64Assembler::add_reg64_reg64_imm32(
                buf,
                AArch64GeneralReg::ZRSP,
                AArch64GeneralReg::ZRSP,
                aligned_stack_size,
            );
        }
    }

    #[inline(always)]
    fn load_args<'a>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<
            'a,
            '_,
            AArch64GeneralReg,
            AArch64FloatReg,
            AArch64Assembler,
            AArch64Call,
        >,
        _layout_interner: &mut STLayoutInterner<'a>,
        _args: &'a [(InLayout<'a>, Symbol)],
        _ret_layout: &InLayout<'a>,
    ) {
        todo!("Loading args for AArch64");
    }

    #[inline(always)]
    fn store_args<'a>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<
            'a,
            '_,
            AArch64GeneralReg,
            AArch64FloatReg,
            AArch64Assembler,
            AArch64Call,
        >,
        _layout_interner: &mut STLayoutInterner<'a>,
        _dst: &Symbol,
        _args: &[Symbol],
        _arg_layouts: &[InLayout<'a>],
        _ret_layout: &InLayout<'a>,
    ) {
        todo!("Storing args for AArch64");
    }

    fn return_complex_symbol<'a>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<
            'a,
            '_,
            AArch64GeneralReg,
            AArch64FloatReg,
            AArch64Assembler,
            AArch64Call,
        >,
        _layout_interner: &mut STLayoutInterner<'a>,
        _sym: &Symbol,
        _layout: &InLayout<'a>,
    ) {
        todo!("Returning complex symbols for AArch64");
    }

    fn load_returned_complex_symbol<'a>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<
            'a,
            '_,
            AArch64GeneralReg,
            AArch64FloatReg,
            AArch64Assembler,
            AArch64Call,
        >,
        _layout_interner: &mut STLayoutInterner<'a>,
        _sym: &Symbol,
        _layout: &InLayout<'a>,
    ) {
        todo!("Loading returned complex symbols for AArch64");
    }
}

impl Assembler<AArch64GeneralReg, AArch64FloatReg> for AArch64Assembler {
    #[inline(always)]
    fn base_pointer() -> AArch64GeneralReg {
        AArch64GeneralReg::FP
    }

    #[inline(always)]
    fn abs_reg64_reg64(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, src: AArch64GeneralReg) {
        cmp_reg64_imm12(buf, src, 0);
        cneg_reg64_reg64_cond(buf, dst, src, ConditionCode::MI);
    }

    #[inline(always)]
    fn abs_freg64_freg64(
        buf: &mut Vec<'_, u8>,
        _relocs: &mut Vec<'_, Relocation>,
        dst: AArch64FloatReg,
        src: AArch64FloatReg,
    ) {
        fabs_freg_freg(buf, FloatWidth::F64, dst, src);
    }

    #[inline(always)]
    fn add_reg64_reg64_imm32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src: AArch64GeneralReg,
        imm32: i32,
    ) {
        if imm32 < 0 {
            todo!("immediate addition with values less than 0");
        } else if imm32 < 0xFFF {
            add_reg64_reg64_imm12(buf, dst, src, imm32 as u16);
        } else {
            todo!("immediate additions with values greater than 12bits");
        }
    }
    #[inline(always)]
    fn add_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        add_reg64_reg64_reg64(buf, dst, src1, src2);
    }
    #[inline(always)]
    fn add_freg32_freg32_freg32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fadd_freg_freg_freg(buf, FloatWidth::F32, dst, src1, src2);
    }
    #[inline(always)]
    fn add_freg64_freg64_freg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fadd_freg_freg_freg(buf, FloatWidth::F64, dst, src1, src2);
    }

    #[inline(always)]
    fn call(_buf: &mut Vec<'_, u8>, _relocs: &mut Vec<'_, Relocation>, _fn_name: String) {
        todo!("calling functions literal for AArch64");
    }

    #[inline(always)]
    fn function_pointer(
        _buf: &mut Vec<'_, u8>,
        _relocs: &mut Vec<'_, Relocation>,
        _fn_name: String,
        _dst: AArch64GeneralReg,
    ) {
        todo!("calling functions literal for AArch64");
    }

    #[inline(always)]
    fn imul_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        mul_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn umul_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        mul_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn idiv_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        sdiv_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn udiv_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        udiv_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn irem_reg64_reg64_reg64<'a, ASM, CC>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        _dst: AArch64GeneralReg,
        _src1: AArch64GeneralReg,
        _src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        todo!()
    }

    fn urem_reg64_reg64_reg64<'a, ASM, CC>(
        _buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        _dst: AArch64GeneralReg,
        _src1: AArch64GeneralReg,
        _src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        todo!()
    }

    #[inline(always)]
    fn mul_freg32_freg32_freg32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fmul_freg_freg_freg(buf, FloatWidth::F32, dst, src1, src2);
    }
    #[inline(always)]
    fn mul_freg64_freg64_freg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fmul_freg_freg_freg(buf, FloatWidth::F64, dst, src1, src2);
    }

    #[inline(always)]
    fn div_freg32_freg32_freg32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fdiv_freg_freg_freg(buf, FloatWidth::F32, dst, src1, src2);
    }
    #[inline(always)]
    fn div_freg64_freg64_freg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64FloatReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
    ) {
        fdiv_freg_freg_freg(buf, FloatWidth::F64, dst, src1, src2);
    }

    #[inline(always)]
    fn jmp_imm32(buf: &mut Vec<'_, u8>, offset: i32) -> usize {
        if (-(1 << 27)..(1 << 27)).contains(&offset) {
            b_imm26(buf, offset);
        } else {
            todo!("jump offsets over 27 bits for AArch64: {:#x}", offset);
        }

        buf.len()
    }

    #[inline(always)]
    fn tail_call(buf: &mut Vec<'_, u8>) -> u64 {
        Self::jmp_imm32(buf, 0);
        buf.len() as u64 - 4 // TODO is 4 the correct offset in ARM?
    }

    #[inline(always)]
    fn jne_reg64_imm64_imm32(
        buf: &mut Vec<'_, u8>,
        reg: AArch64GeneralReg,
        imm: u64,
        offset: i32,
    ) -> usize {
        if imm < (1 << 12) {
            cmp_reg64_imm12(buf, reg, imm as u16);
        } else {
            todo!(
                "cmp immediate with value over 12 bits for AArch64: {:#x}",
                imm
            );
        }

        if (-(1 << 20)..(1 << 20)).contains(&offset) {
            b_cond_imm19(buf, ConditionCode::NE, offset);
        } else {
            todo!("jump offsets over 20 bits for AArch64: {:#x}", offset);
        }

        buf.len()
    }

    #[inline(always)]
    fn mov_freg32_imm32(
        buf: &mut Vec<'_, u8>,
        _relocs: &mut Vec<'_, Relocation>,
        dst: AArch64FloatReg,
        imm: f32,
    ) {
        // See https://stackoverflow.com/a/64608524
        if imm == 0.0 && !imm.is_sign_negative() {
            movi_freg_zero(buf, dst);
            return;
        }
        match encode_f32_to_imm8(imm) {
            Some(imm8) => {
                fmov_freg_imm8(buf, FloatWidth::F32, dst, imm8);
            }
            None => {
                todo!("loading f32 literal over 8 bits for AArch64");
            }
        }
    }
    #[inline(always)]
    fn mov_freg64_imm64(
        buf: &mut Vec<'_, u8>,
        _relocs: &mut Vec<'_, Relocation>,
        dst: AArch64FloatReg,
        imm: f64,
    ) {
        // See https://stackoverflow.com/a/64608524
        if imm == 0.0 && !imm.is_sign_negative() {
            movi_freg_zero(buf, dst);
            return;
        }
        match encode_f64_to_imm8(imm) {
            Some(imm8) => {
                fmov_freg_imm8(buf, FloatWidth::F64, dst, imm8);
            }
            None => {
                todo!("loading f64 literal over 8 bits for AArch64");
            }
        }
    }
    #[inline(always)]
    fn mov_reg64_imm64(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, imm: i64) {
        let mut remaining = imm as u64;
        movz_reg64_imm16(buf, dst, remaining as u16, 0);
        remaining >>= 16;
        if remaining > 0 {
            movk_reg64_imm16(buf, dst, remaining as u16, 1);
        }
        remaining >>= 16;
        if remaining > 0 {
            movk_reg64_imm16(buf, dst, remaining as u16, 2);
        }
        remaining >>= 16;
        if remaining > 0 {
            movk_reg64_imm16(buf, dst, remaining as u16, 3);
        }
    }
    #[inline(always)]
    fn mov_freg64_freg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
        fmov_freg_freg(buf, FloatWidth::F64, dst, src);
    }

    #[inline(always)]
    fn mov_reg32_freg32(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg, _src: AArch64FloatReg) {
        unimplemented!();
    }
    #[inline(always)]
    fn mov_reg64_freg64(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg, _src: AArch64FloatReg) {
        unimplemented!();
    }

    #[inline(always)]
    fn mov_reg_reg(
        buf: &mut Vec<'_, u8>,
        register_width: RegisterWidth,
        dst: AArch64GeneralReg,
        src: AArch64GeneralReg,
    ) {
        match register_width {
            RegisterWidth::W8 => todo!(),
            RegisterWidth::W16 => todo!(),
            RegisterWidth::W32 => todo!(),
            RegisterWidth::W64 => mov_reg64_reg64(buf, dst, src),
        }
    }

    #[inline(always)]
    fn movsx_reg_reg(
        _buf: &mut Vec<'_, u8>,
        _input_width: RegisterWidth,
        _dst: AArch64GeneralReg,
        _src: AArch64GeneralReg,
    ) {
        todo!("move with sign extension");
    }

    #[inline(always)]
    fn mov_freg64_base32(_buf: &mut Vec<'_, u8>, _dst: AArch64FloatReg, _offset: i32) {
        todo!("loading floating point reg from base offset for AArch64");
    }
    #[inline(always)]
    fn mov_reg64_base32(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, offset: i32) {
        if offset < 0 {
            todo!("negative base offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            ldr_reg64_reg64_imm12(buf, dst, AArch64GeneralReg::FP, (offset as u16) >> 3);
        } else {
            todo!("base offsets over 32k for AArch64");
        }
    }
    #[inline(always)]
    fn mov_reg32_base32(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg, _offset: i32) {
        todo!()
    }
    #[inline(always)]
    fn mov_reg16_base32(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg, _offset: i32) {
        todo!()
    }
    #[inline(always)]
    fn mov_reg8_base32(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg, _offset: i32) {
        todo!()
    }
    #[inline(always)]
    fn mov_base32_freg64(_buf: &mut Vec<'_, u8>, _offset: i32, _src: AArch64FloatReg) {
        todo!("saving floating point reg to base offset for AArch64");
    }
    #[inline(always)]
    fn movesd_mem64_offset32_freg64(
        _buf: &mut Vec<'_, u8>,
        _ptr: AArch64GeneralReg,
        _offset: i32,
        _src: AArch64FloatReg,
    ) {
        todo!()
    }

    #[inline(always)]
    fn mov_base32_reg64(buf: &mut Vec<'_, u8>, offset: i32, src: AArch64GeneralReg) {
        if offset < 0 {
            todo!("negative base offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            str_reg64_reg64_imm12(buf, src, AArch64GeneralReg::FP, (offset as u16) >> 3);
        } else {
            todo!("base offsets over 32k for AArch64");
        }
    }

    #[inline(always)]
    fn mov_base32_reg32(_buf: &mut Vec<'_, u8>, _offset: i32, _src: AArch64GeneralReg) {
        todo!()
    }
    #[inline(always)]
    fn mov_base32_reg16(_buf: &mut Vec<'_, u8>, _offset: i32, _src: AArch64GeneralReg) {
        todo!()
    }
    #[inline(always)]
    fn mov_base32_reg8(_buf: &mut Vec<'_, u8>, _offset: i32, _src: AArch64GeneralReg) {
        todo!()
    }

    #[inline(always)]
    fn mov_reg64_mem64_offset32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src: AArch64GeneralReg,
        offset: i32,
    ) {
        if offset < 0 {
            todo!("negative mem offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            ldr_reg64_reg64_imm12(buf, dst, src, (offset as u16) >> 3);
        } else {
            todo!("mem offsets over 32k for AArch64");
        }
    }
    #[inline(always)]
    fn mov_reg32_mem32_offset32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src: AArch64GeneralReg,
        offset: i32,
    ) {
        if offset < 0 {
            todo!("negative mem offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            ldr_reg64_reg64_imm12(buf, dst, src, (offset as u16) >> 3);
        } else {
            todo!("mem offsets over 32k for AArch64");
        }
    }
    #[inline(always)]
    fn mov_reg16_mem16_offset32(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64GeneralReg,
        _src: AArch64GeneralReg,
        _offset: i32,
    ) {
        todo!()
    }
    #[inline(always)]
    fn mov_reg8_mem8_offset32(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64GeneralReg,
        _src: AArch64GeneralReg,
        _offset: i32,
    ) {
        todo!()
    }

    #[inline(always)]
    fn mov_mem64_offset32_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        offset: i32,
        src: AArch64GeneralReg,
    ) {
        if offset < 0 {
            todo!("negative mem offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            str_reg64_reg64_imm12(buf, src, dst, (offset as u16) >> 3);
        } else {
            todo!("mem offsets over 32k for AArch64");
        }
    }

    #[inline(always)]
    fn mov_mem32_offset32_reg32(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64GeneralReg,
        _offset: i32,
        _src: AArch64GeneralReg,
    ) {
        todo!()
    }

    #[inline(always)]
    fn mov_mem16_offset32_reg16(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64GeneralReg,
        _offset: i32,
        _src: AArch64GeneralReg,
    ) {
        todo!()
    }

    #[inline(always)]
    fn mov_mem8_offset32_reg8(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64GeneralReg,
        _offset: i32,
        _src: AArch64GeneralReg,
    ) {
        todo!()
    }

    #[inline(always)]
    fn movsx_reg64_base32(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, offset: i32, size: u8) {
        debug_assert!(size <= 8);
        if size == 8 {
            Self::mov_reg64_base32(buf, dst, offset);
        } else if size == 4 {
            todo!("sign extending 4 byte values");
        } else if size == 2 {
            todo!("sign extending 2 byte values");
        } else if size == 1 {
            todo!("sign extending 1 byte values");
        } else {
            internal_error!("Invalid size for sign extension: {}", size);
        }
    }
    #[inline(always)]
    fn movzx_reg64_base32(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, offset: i32, size: u8) {
        debug_assert!(size <= 8);
        if size == 8 {
            Self::mov_reg64_base32(buf, dst, offset);
        } else if size == 4 {
            todo!("zero extending 4 byte values");
        } else if size == 2 {
            todo!("zero extending 2 byte values");
        } else if size == 1 {
            todo!("zero extending 1 byte values");
        } else {
            internal_error!("Invalid size for zero extension: {}", size);
        }
    }

    #[inline(always)]
    fn mov_freg64_stack32(_buf: &mut Vec<'_, u8>, _dst: AArch64FloatReg, _offset: i32) {
        todo!("loading floating point reg from stack for AArch64");
    }
    #[inline(always)]
    fn mov_reg64_stack32(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, offset: i32) {
        if offset < 0 {
            todo!("negative stack offsets for AArch64");
        } else if offset < (0xFFF << 8) {
            debug_assert!(offset % 8 == 0);
            ldr_reg64_reg64_imm12(buf, dst, AArch64GeneralReg::ZRSP, (offset as u16) >> 3);
        } else {
            todo!("stack offsets over 32k for AArch64");
        }
    }
    #[inline(always)]
    fn mov_stack32_freg64(_buf: &mut Vec<'_, u8>, _offset: i32, _src: AArch64FloatReg) {
        todo!("saving floating point reg to stack for AArch64");
    }
    #[inline(always)]
    fn mov_stack32_reg(
        buf: &mut Vec<'_, u8>,
        register_width: RegisterWidth,
        offset: i32,
        src: AArch64GeneralReg,
    ) {
        match register_width {
            RegisterWidth::W8 => todo!(),
            RegisterWidth::W16 => todo!(),
            RegisterWidth::W32 => todo!(),
            RegisterWidth::W64 => {
                if offset < 0 {
                    todo!("negative stack offsets for AArch64");
                } else if offset < (0xFFF << 8) {
                    debug_assert!(offset % 8 == 0);
                    str_reg64_reg64_imm12(buf, src, AArch64GeneralReg::ZRSP, (offset as u16) >> 3);
                } else {
                    todo!("stack offsets over 32k for AArch64");
                }
            }
        }
    }
    #[inline(always)]
    fn neg_reg64_reg64(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, src: AArch64GeneralReg) {
        neg_reg64_reg64(buf, dst, src);
    }

    #[inline(always)]
    fn sub_reg64_reg64_imm32(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src: AArch64GeneralReg,
        imm32: i32,
    ) {
        if imm32 < 0 {
            todo!("immediate subtractions with values less than 0");
        } else if imm32 < 0xFFF {
            sub_reg64_reg64_imm12(buf, dst, src, imm32 as u16);
        } else {
            todo!("immediate subtractions with values greater than 12bits");
        }
    }
    #[inline(always)]
    fn sub_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        sub_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    #[inline(always)]
    fn eq_reg_reg_reg(
        buf: &mut Vec<'_, u8>,
        _register_width: RegisterWidth,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        cmp_reg64_reg64(buf, src1, src2);
        cset_reg64_cond(buf, dst, ConditionCode::EQ);
    }

    #[inline(always)]
    fn neq_reg_reg_reg(
        buf: &mut Vec<'_, u8>,
        _register_width: RegisterWidth,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        cmp_reg64_reg64(buf, src1, src2);
        cset_reg64_cond(buf, dst, ConditionCode::NE);
    }

    #[inline(always)]
    fn cmp_freg_freg_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64FloatReg,
        src2: AArch64FloatReg,
        width: FloatWidth,
        operation: CompareOperation,
    ) {
        fcmp_freg_freg(buf, width, src1, src2);

        let cond = match operation {
            CompareOperation::LessThan => ConditionCode::MI,
            CompareOperation::LessThanOrEqual => ConditionCode::LS,
            CompareOperation::GreaterThan => ConditionCode::GT,
            CompareOperation::GreaterThanOrEqual => ConditionCode::GE,
        };
        cset_reg64_cond(buf, dst, cond);
    }

    #[inline(always)]
    fn is_nan_freg_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src: AArch64FloatReg,
        width: FloatWidth,
    ) {
        fcmp_freg_freg(buf, width, src, src);
        cset_reg64_cond(buf, dst, ConditionCode::NE);
    }

    #[inline(always)]
    fn to_float_freg64_reg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64GeneralReg) {
        scvtf_freg_reg64(buf, FloatWidth::F64, dst, src);
    }

    #[inline(always)]
    fn to_float_freg32_reg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64GeneralReg) {
        scvtf_freg_reg64(buf, FloatWidth::F32, dst, src);
    }

    #[inline(always)]
    fn to_float_freg32_freg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
        fcvt_freg32_freg64(buf, dst, src);
    }

    #[inline(always)]
    fn to_float_freg64_freg32(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
        fcvt_freg64_freg32(buf, dst, src);
    }

    fn set_if_overflow(_buf: &mut Vec<'_, u8>, _dst: AArch64GeneralReg) {
        todo!("set if overflow for AArch64");
    }

    #[inline(always)]
    fn ret(buf: &mut Vec<'_, u8>) {
        ret_reg64(buf, AArch64GeneralReg::LR)
    }

    fn and_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        and_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn or_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        orr_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn xor_reg64_reg64_reg64(
        buf: &mut Vec<'_, u8>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        eor_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn shl_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        lsl_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn shr_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        lsr_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn sar_reg64_reg64_reg64<'a, ASM, CC>(
        buf: &mut Vec<'a, u8>,
        _storage_manager: &mut StorageManager<'a, '_, AArch64GeneralReg, AArch64FloatReg, ASM, CC>,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) where
        ASM: Assembler<AArch64GeneralReg, AArch64FloatReg>,
        CC: CallConv<AArch64GeneralReg, AArch64FloatReg, ASM>,
    {
        asr_reg64_reg64_reg64(buf, dst, src1, src2);
    }

    fn sqrt_freg64_freg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
        fsqrt_freg_freg(buf, FloatWidth::F64, dst, src);
    }

    fn sqrt_freg32_freg32(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
        fsqrt_freg_freg(buf, FloatWidth::F32, dst, src);
    }

    fn signed_compare_reg64(
        buf: &mut Vec<'_, u8>,
        _register_width: RegisterWidth,
        operation: CompareOperation,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        cmp_reg64_reg64(buf, src1, src2);
        let cond = match operation {
            CompareOperation::LessThan => ConditionCode::LT,
            CompareOperation::LessThanOrEqual => ConditionCode::LE,
            CompareOperation::GreaterThan => ConditionCode::GT,
            CompareOperation::GreaterThanOrEqual => ConditionCode::GE,
        };
        cset_reg64_cond(buf, dst, cond);
    }

    fn unsigned_compare_reg64(
        buf: &mut Vec<'_, u8>,
        _register_width: RegisterWidth,
        operation: CompareOperation,
        dst: AArch64GeneralReg,
        src1: AArch64GeneralReg,
        src2: AArch64GeneralReg,
    ) {
        cmp_reg64_reg64(buf, src1, src2);
        let cond = match operation {
            CompareOperation::LessThan => ConditionCode::CCLO,
            CompareOperation::LessThanOrEqual => ConditionCode::LS,
            CompareOperation::GreaterThan => ConditionCode::HI,
            CompareOperation::GreaterThanOrEqual => ConditionCode::CSHS,
        };
        cset_reg64_cond(buf, dst, cond);
    }

    fn mov_freg64_mem64_offset32(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64FloatReg,
        _src: AArch64GeneralReg,
        _offset: i32,
    ) {
        todo!()
    }

    fn mov_freg32_mem32_offset32(
        _buf: &mut Vec<'_, u8>,
        _dst: AArch64FloatReg,
        _src: AArch64GeneralReg,
        _offset: i32,
    ) {
        todo!()
    }
}

impl AArch64Assembler {}

// Instructions
// ARM manual section C3
// https://developer.arm.com/documentation/ddi0487/ga
// Map all instructions to a packed struct.

trait Aarch64Bytes: PackedStruct {
    #[inline(always)]
    fn bytes(&self) -> [u8; 4] {
        let mut bytes: [u8; 4] = [0, 0, 0, 0];

        self.pack_to_slice(&mut bytes).unwrap();

        bytes.reverse();

        bytes
    }
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct MoveWideImmediate {
    sf: bool,
    opc: Integer<u8, packed_bits::Bits<2>>,
    fixed: Integer<u8, packed_bits::Bits<6>>, // = 0b100101,
    hw: Integer<u8, packed_bits::Bits<2>>,
    imm16: u16,
    reg_d: Integer<u8, packed_bits::Bits<5>>, // AArch64GeneralReg
}

impl Aarch64Bytes for MoveWideImmediate {}

pub struct MoveWideImmediateParams {
    opc: u8,
    rd: AArch64GeneralReg,
    imm16: u16,
    hw: u8,
    sf: bool,
}

impl MoveWideImmediate {
    #[inline(always)]
    fn new(
        MoveWideImmediateParams {
            opc,
            rd,
            imm16,
            hw,
            sf,
        }: MoveWideImmediateParams,
    ) -> Self {
        // TODO: revisit this is we change where we want to check the shift
        // currently this is done in the assembler above
        // assert!(shift % 16 == 0 && shift <= 48);
        debug_assert!(hw <= 0b11);
        debug_assert!(opc <= 0b11);

        Self {
            reg_d: rd.id().into(),
            imm16,
            hw: hw.into(),
            opc: opc.into(),
            sf,
            fixed: 0b100101.into(),
        }
    }
}

#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct ArithmeticImmediate {
    sf: bool,
    op: bool, // add or subtract
    s: bool,
    fixed: Integer<u8, packed_bits::Bits<6>>, // = 0b100010,
    sh: bool,                                 // shift
    imm12: Integer<u16, packed_bits::Bits<12>>,
    reg_n: Integer<u8, packed_bits::Bits<5>>,
    reg_d: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for ArithmeticImmediate {}

pub struct ArithmeticImmediateParams {
    op: bool,
    s: bool,
    rd: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    imm12: u16,
    sh: bool,
}

impl ArithmeticImmediate {
    #[inline(always)]
    fn new(
        ArithmeticImmediateParams {
            op,
            s,
            rd,
            rn,
            imm12,
            sh,
        }: ArithmeticImmediateParams,
    ) -> Self {
        debug_assert!(imm12 <= 0xFFF);

        Self {
            reg_d: rd.id().into(),
            reg_n: rn.id().into(),
            imm12: imm12.into(),
            sh,
            s,
            op,
            // true for 64 bit addition
            // false for 32 bit addition
            sf: true,
            fixed: 0b100010.into(),
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum ShiftType {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
}

impl ShiftType {
    #[inline(always)]
    fn id(&self) -> u8 {
        *self as u8
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct ArithmeticShifted {
    sf: bool,
    op: bool, // add or subtract
    s: bool,
    fixed: Integer<u8, packed_bits::Bits<5>>, // = 0b01011,
    shift: Integer<u8, packed_bits::Bits<2>>, // shift
    fixed2: bool,                             // = 0b0,
    reg_m: Integer<u8, packed_bits::Bits<5>>,
    imm6: Integer<u8, packed_bits::Bits<6>>,
    reg_n: Integer<u8, packed_bits::Bits<5>>,
    reg_d: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for ArithmeticShifted {}

pub struct ArithmeticShiftedParams {
    op: bool,
    s: bool,
    shift: ShiftType,
    imm6: u8,
    rm: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    rd: AArch64GeneralReg,
}

impl ArithmeticShifted {
    #[inline(always)]
    fn new(
        ArithmeticShiftedParams {
            op,
            s,
            shift,
            imm6,
            rm,
            rn,
            rd,
        }: ArithmeticShiftedParams,
    ) -> Self {
        debug_assert!(imm6 <= 0b111111);

        Self {
            reg_d: rd.id().into(),
            reg_n: rn.id().into(),
            imm6: imm6.into(),
            reg_m: rm.id().into(),
            fixed2: false,
            shift: shift.id().into(),
            fixed: 0b01011.into(),
            s,
            op,
            // true for 64 bit addition
            // false for 32 bit addition
            sf: true,
        }
    }
}

// ARM manual section C1.2.4
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum ConditionCode {
    /// Equal
    EQ = 0b0000,
    /// Not equal
    NE = 0b0001,
    /// CS or HS: Carry set
    CSHS = 0b0010,
    /// CC or LO: Carry clear
    CCLO = 0b0011,
    /// Minus, negative
    MI = 0b0100,
    /// Plus, positive or zero
    PL = 0b0101,
    /// Overflow
    VS = 0b0110,
    /// No overflow
    VC = 0b0111,
    /// Unsigned higher
    HI = 0b1000,
    /// Unsigned lower or same
    LS = 0b1001,
    /// Signed greater than or equal
    GE = 0b1010,
    /// Signed less than
    LT = 0b1011,
    /// Signed greater than
    GT = 0b1100,
    /// Signed less than or equal
    LE = 0b1101,
    /// Always
    AL = 0b1110,
}

impl ConditionCode {
    #[inline(always)]
    fn id(&self) -> u8 {
        *self as u8
    }

    /// The inverse of the condition code. For example, EQ becomes NE.
    fn invert(self) -> Self {
        match self {
            ConditionCode::EQ => ConditionCode::NE,
            ConditionCode::NE => ConditionCode::EQ,
            ConditionCode::CSHS => ConditionCode::CCLO,
            ConditionCode::CCLO => ConditionCode::CSHS,
            ConditionCode::MI => ConditionCode::PL,
            ConditionCode::PL => ConditionCode::MI,
            ConditionCode::VS => ConditionCode::VC,
            ConditionCode::VC => ConditionCode::VS,
            ConditionCode::HI => ConditionCode::LS,
            ConditionCode::LS => ConditionCode::HI,
            ConditionCode::GE => ConditionCode::LT,
            ConditionCode::LT => ConditionCode::GE,
            ConditionCode::GT => ConditionCode::LE,
            ConditionCode::LE => ConditionCode::GT,
            ConditionCode::AL => ConditionCode::AL,
        }
    }
}

impl std::fmt::Display for ConditionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConditionCode::EQ => "eq",
                ConditionCode::NE => "ne",
                ConditionCode::CSHS => "hs",
                ConditionCode::CCLO => "lo",
                ConditionCode::MI => "mi",
                ConditionCode::PL => "pl",
                ConditionCode::VS => "vs",
                ConditionCode::VC => "vc",
                ConditionCode::HI => "hi",
                ConditionCode::LS => "ls",
                ConditionCode::GE => "ge",
                ConditionCode::LT => "lt",
                ConditionCode::GT => "gt",
                ConditionCode::LE => "le",
                ConditionCode::AL => "al",
            }
        )
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct ConditionalBranchImmediate {
    fixed: Integer<u8, packed_bits::Bits<7>>,
    o1: bool,
    imm19: Integer<u32, packed_bits::Bits<19>>,
    o0: bool,
    cond: Integer<u8, packed_bits::Bits<4>>,
}

impl Aarch64Bytes for ConditionalBranchImmediate {}

pub struct ConditionalBranchImmediateParams {
    cond: ConditionCode,
    imm19: u32,
}

impl ConditionalBranchImmediate {
    #[inline(always)]
    fn new(
        ConditionalBranchImmediateParams { cond, imm19 }: ConditionalBranchImmediateParams,
    ) -> Self {
        debug_assert!(imm19 >> 19 == 0);

        Self {
            cond: cond.id().into(),
            o0: false,
            imm19: imm19.into(),
            o1: false,
            fixed: 0b0101010.into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct ConditionalSelect {
    sf: bool,
    op: bool,
    s: bool,
    fixed: Integer<u8, packed_bits::Bits<8>>,
    reg_m: Integer<u8, packed_bits::Bits<5>>,
    cond: Integer<u8, packed_bits::Bits<4>>,
    op2: Integer<u8, packed_bits::Bits<2>>,
    reg_n: Integer<u8, packed_bits::Bits<5>>,
    reg_d: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for ConditionalSelect {}

pub struct ConditionalSelectParams {
    op: bool,
    s: bool,
    cond: ConditionCode,
    op2: u8,
    rm: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    rd: AArch64GeneralReg,
}

impl ConditionalSelect {
    #[inline(always)]
    fn new(
        ConditionalSelectParams {
            op,
            s,
            cond,
            op2,
            rm,
            rn,
            rd,
        }: ConditionalSelectParams,
    ) -> Self {
        debug_assert!(op2 <= 0b11);

        Self {
            reg_d: rd.id().into(),
            reg_n: rn.id().into(),
            op2: op2.into(),
            cond: cond.id().into(),
            reg_m: rm.id().into(),
            fixed: 0b11010100.into(),
            s,
            op,
            // true for 64 bit addition
            // false for 32 bit addition
            sf: true,
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct DataProcessingTwoSource {
    sf: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<8>>,
    reg_m: Integer<u8, packed_bits::Bits<5>>,
    op: Integer<u8, packed_bits::Bits<6>>,
    reg_n: Integer<u8, packed_bits::Bits<5>>,
    reg_d: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for DataProcessingTwoSource {}

pub struct DataProcessingTwoSourceParams {
    op: u8,
    rm: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    rd: AArch64GeneralReg,
}

impl DataProcessingTwoSource {
    #[inline(always)]
    fn new(
        DataProcessingTwoSourceParams { op, rm, rn, rd }: DataProcessingTwoSourceParams,
    ) -> Self {
        debug_assert!(op <= 0b111111);

        Self {
            sf: true,
            fixed: false,
            s: false,
            fixed2: 0b11010110.into(),
            reg_m: rm.id().into(),
            op: op.into(),
            reg_n: rn.id().into(),
            reg_d: rd.id().into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct DataProcessingThreeSource {
    sf: bool,
    op54: Integer<u8, packed_bits::Bits<2>>,
    fixed: Integer<u8, packed_bits::Bits<5>>,
    op31: Integer<u8, packed_bits::Bits<3>>,
    rm: Integer<u8, packed_bits::Bits<5>>,
    o0: bool,
    ra: Integer<u8, packed_bits::Bits<5>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for DataProcessingThreeSource {}

pub struct DataProcessingThreeSourceParams {
    op31: u8,
    rm: AArch64GeneralReg,
    ra: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    rd: AArch64GeneralReg,
}

impl DataProcessingThreeSource {
    #[inline(always)]
    fn new(
        DataProcessingThreeSourceParams {
            op31,
            rm,
            ra,
            rn,
            rd,
        }: DataProcessingThreeSourceParams,
    ) -> Self {
        debug_assert!(op31 <= 0b111);

        Self {
            sf: true,
            op54: 0b00.into(),
            fixed: 0b011011.into(),
            op31: op31.into(),
            rm: rm.id().into(),
            o0: false,
            ra: ra.id().into(),
            rn: rn.id().into(),
            rd: rd.id().into(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum LogicalOp {
    AND,
    BIC,
    ORR,
    ORN,
    EOR,
    EON,
    ANDS,
    BICS,
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct LogicalShiftedRegister {
    sf: bool,
    op: Integer<u8, packed_bits::Bits<2>>,
    fixed: Integer<u8, packed_bits::Bits<5>>, // = 0b01010,
    shift: Integer<u8, packed_bits::Bits<2>>, // shift
    n: bool,
    reg_m: Integer<u8, packed_bits::Bits<5>>,
    imm6: Integer<u8, packed_bits::Bits<6>>,
    reg_n: Integer<u8, packed_bits::Bits<5>>,
    reg_d: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for LogicalShiftedRegister {}

pub struct LogicalShiftedRegisterParams {
    op: LogicalOp,
    shift: ShiftType,
    imm6: u8,
    rm: AArch64GeneralReg,
    rn: AArch64GeneralReg,
    rd: AArch64GeneralReg,
}

impl LogicalShiftedRegister {
    #[inline(always)]
    fn new(
        LogicalShiftedRegisterParams {
            op,
            shift,
            imm6,
            rm,
            rn,
            rd,
        }: LogicalShiftedRegisterParams,
    ) -> Self {
        debug_assert!(imm6 <= 0b111111);

        let (op, n) = match op {
            LogicalOp::AND => (0b00, false),
            LogicalOp::BIC => (0b00, true),
            LogicalOp::ORR => (0b01, false),
            LogicalOp::ORN => (0b01, true),
            LogicalOp::EOR => (0b10, false),
            LogicalOp::EON => (0b10, true),
            LogicalOp::ANDS => (0b11, false),
            LogicalOp::BICS => (0b11, true),
        };

        Self {
            reg_d: rd.id().into(),
            reg_n: rn.id().into(),
            imm6: imm6.into(),
            reg_m: rm.id().into(),
            n,
            shift: shift.id().into(),
            fixed: 0b01010.into(),
            op: op.into(),
            // true for 64 bit addition
            // false for 32 bit addition
            sf: true,
        }
    }
}

#[derive(PackedStruct)]
pub struct UnconditionalBranchRegister {
    fixed: Integer<u8, packed_bits::Bits<7>>,
    z: bool,
    fixed2: bool,
    op: Integer<u8, packed_bits::Bits<2>>,
    fixed3: Integer<u8, packed_bits::Bits<5>>,
    fixed4: Integer<u8, packed_bits::Bits<4>>,
    a: bool,
    m: bool,
    rn: Integer<u8, packed_bits::Bits<5>>,
    fixed5: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for UnconditionalBranchRegister {}

pub struct UnconditionalBranchRegisterParams {
    op: u8,
    rn: AArch64GeneralReg,
}

impl UnconditionalBranchRegister {
    #[inline(always)]
    fn new(
        UnconditionalBranchRegisterParams { op, rn }: UnconditionalBranchRegisterParams,
    ) -> Self {
        debug_assert!(op <= 0b11);

        Self {
            fixed5: 0b00000.into(),
            rn: rn.id().into(),
            m: false,
            a: false,
            fixed4: 0b0000.into(),
            fixed3: 0b11111.into(),
            op: op.into(),
            fixed2: false,
            z: false,
            fixed: 0b1101011.into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct UnconditionalBranchImmediate {
    op: bool, // false=B, true=BL
    fixed: Integer<u8, packed_bits::Bits<5>>,
    imm26: Integer<u32, packed_bits::Bits<26>>,
}

impl Aarch64Bytes for UnconditionalBranchImmediate {}

pub struct UnconditionalBranchImmediateParams {
    op: bool,
    imm26: u32,
}

impl UnconditionalBranchImmediate {
    #[inline(always)]
    fn new(
        UnconditionalBranchImmediateParams { op, imm26 }: UnconditionalBranchImmediateParams,
    ) -> Self {
        debug_assert!(imm26 <= 0b11_1111_1111_1111_1111_1111_1111);
        Self {
            op,
            fixed: 0b00101.into(),
            imm26: imm26.into(),
        }
    }
}

// Uses unsigned Offset
// opc = 0b01 means load
// opc = 0b00 means store
#[derive(PackedStruct, Debug)]
#[packed_struct(endian = "msb")]
pub struct LoadStoreRegisterImmediate {
    size: Integer<u8, packed_bits::Bits<2>>,
    fixed: Integer<u8, packed_bits::Bits<3>>, // = 0b111,
    fixed2: bool,
    fixed3: Integer<u8, packed_bits::Bits<2>>,
    opc: Integer<u8, packed_bits::Bits<2>>,
    imm12: Integer<u16, packed_bits::Bits<12>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    rt: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for LoadStoreRegisterImmediate {}

pub struct LoadStoreRegisterImmediateParams {
    size: u8,
    imm12: u16,
    rn: AArch64GeneralReg,
    rt: AArch64GeneralReg,
}

impl LoadStoreRegisterImmediate {
    #[inline(always)]
    fn new(
        opc: u8,
        LoadStoreRegisterImmediateParams {
            size,
            imm12,
            rn,
            rt,
        }: LoadStoreRegisterImmediateParams,
    ) -> Self {
        debug_assert!(size <= 0b11);
        debug_assert!(imm12 <= 0xFFF);

        Self {
            rt: rt.id().into(),
            rn: rn.id().into(),
            imm12: imm12.into(),
            opc: opc.into(),
            fixed3: 0b01.into(),
            fixed2: false,
            fixed: 0b111.into(),
            size: size.into(),
        }
    }

    #[inline(always)]
    fn new_load(params: LoadStoreRegisterImmediateParams) -> Self {
        Self::new(0b01, params)
    }

    #[inline(always)]
    fn new_store(params: LoadStoreRegisterImmediateParams) -> Self {
        Self::new(0b00, params)
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct AdvancedSimdModifiedImmediate {
    fixed: bool,
    q: bool,
    op: bool,
    fixed2: Integer<u16, packed_bits::Bits<10>>,
    a: bool,
    b: bool,
    c: bool,
    cmode: Integer<u8, packed_bits::Bits<4>>,
    o2: bool,
    fixed3: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for AdvancedSimdModifiedImmediate {}

impl AdvancedSimdModifiedImmediate {
    #[inline(always)]
    fn new(rd: AArch64FloatReg) -> Self {
        Self {
            fixed: false,
            q: false,
            op: true,
            fixed2: 0b0111100000.into(),
            a: false,
            b: false,
            c: false,
            cmode: 0b1110.into(),
            o2: false,
            fixed3: true,
            d: false,
            e: false,
            f: false,
            g: false,
            h: false,
            rd: rd.id().into(),
        }
    }
}

fn encode_float_width(width: FloatWidth) -> u8 {
    match width {
        FloatWidth::F32 => 0b00,
        FloatWidth::F64 => 0b01,
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct ConversionBetweenFloatingPointAndInteger {
    sf: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<5>>,
    ptype: Integer<u8, packed_bits::Bits<2>>,
    fixed3: bool,
    rmode: Integer<u8, packed_bits::Bits<2>>,
    opcode: Integer<u8, packed_bits::Bits<3>>,
    fixed4: Integer<u8, packed_bits::Bits<6>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for ConversionBetweenFloatingPointAndInteger {}

pub struct ConversionBetweenFloatingPointAndIntegerParams {
    ptype: FloatWidth,
    rmode: u8,
    opcode: u8,
    rn: AArch64GeneralReg,
    rd: AArch64FloatReg,
}

impl ConversionBetweenFloatingPointAndInteger {
    #[inline(always)]
    fn new(
        ConversionBetweenFloatingPointAndIntegerParams {
            ptype,
            rmode,
            opcode,
            rn,
            rd,
        }: ConversionBetweenFloatingPointAndIntegerParams,
    ) -> Self {
        debug_assert!(rmode <= 0b11);
        debug_assert!(opcode <= 0b111);

        Self {
            sf: true,
            fixed: false,
            s: false,
            fixed2: 0b11110.into(),
            ptype: encode_float_width(ptype).into(),
            fixed3: true,
            rmode: rmode.into(),
            opcode: opcode.into(),
            fixed4: 0b000000.into(),
            rn: rn.id().into(),
            rd: rd.id().into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct FloatingPointDataProcessingOneSource {
    m: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<5>>,
    ptype: Integer<u8, packed_bits::Bits<2>>,
    fixed3: bool,
    opcode: Integer<u8, packed_bits::Bits<6>>,
    fixed4: Integer<u8, packed_bits::Bits<5>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for FloatingPointDataProcessingOneSource {}

pub struct FloatingPointDataProcessingOneSourceParams {
    ptype: FloatWidth,
    opcode: u8,
    rn: AArch64FloatReg,
    rd: AArch64FloatReg,
}

impl FloatingPointDataProcessingOneSource {
    #[inline(always)]
    fn new(
        FloatingPointDataProcessingOneSourceParams {
            ptype,
            opcode,
            rn,
            rd,
        }: FloatingPointDataProcessingOneSourceParams,
    ) -> Self {
        debug_assert!(opcode <= 0b111111);

        Self {
            m: false,
            fixed: false,
            s: false,
            fixed2: 0b11110.into(),
            ptype: encode_float_width(ptype).into(),
            fixed3: true,
            opcode: opcode.into(),
            fixed4: 0b10000.into(),
            rn: rn.id().into(),
            rd: rd.id().into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct FloatingPointCompare {
    m: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<5>>,
    ptype: Integer<u8, packed_bits::Bits<2>>,
    fixed3: bool,
    rm: Integer<u8, packed_bits::Bits<5>>,
    op: Integer<u8, packed_bits::Bits<2>>,
    fixed4: Integer<u8, packed_bits::Bits<4>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    opcode2: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for FloatingPointCompare {}

pub struct FloatingPointCompareParams {
    ptype: FloatWidth,
    rm: AArch64FloatReg,
    rn: AArch64FloatReg,
    opcode2: u8,
}

impl FloatingPointCompare {
    #[inline(always)]
    fn new(
        FloatingPointCompareParams {
            ptype,
            rm,
            rn,
            opcode2,
        }: FloatingPointCompareParams,
    ) -> Self {
        debug_assert!(opcode2 <= 0b11111);

        Self {
            m: false,
            fixed: false,
            s: false,
            fixed2: 0b11110.into(),
            ptype: encode_float_width(ptype).into(),
            fixed3: true,
            rm: rm.id().into(),
            op: 0b00.into(),
            fixed4: 0b1000.into(),
            rn: rn.id().into(),
            opcode2: opcode2.into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct FloatingPointDataProcessingTwoSource {
    m: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<5>>,
    ptype: Integer<u8, packed_bits::Bits<2>>,
    fixed3: bool,
    rm: Integer<u8, packed_bits::Bits<5>>,
    opcode: Integer<u8, packed_bits::Bits<4>>,
    fixed4: Integer<u8, packed_bits::Bits<2>>,
    rn: Integer<u8, packed_bits::Bits<5>>,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for FloatingPointDataProcessingTwoSource {}

pub struct FloatingPointDataProcessingTwoSourceParams {
    ptype: FloatWidth,
    rm: AArch64FloatReg,
    opcode: u8,
    rn: AArch64FloatReg,
    rd: AArch64FloatReg,
}

impl FloatingPointDataProcessingTwoSource {
    #[inline(always)]
    fn new(
        FloatingPointDataProcessingTwoSourceParams {
            ptype,
            rm,
            opcode,
            rn,
            rd,
        }: FloatingPointDataProcessingTwoSourceParams,
    ) -> Self {
        debug_assert!(opcode <= 0b1111);

        Self {
            m: false,
            fixed: false,
            s: false,
            fixed2: 0b11110.into(),
            ptype: encode_float_width(ptype).into(),
            fixed3: true,
            rm: rm.id().into(),
            opcode: opcode.into(),
            fixed4: 0b10.into(),
            rn: rn.id().into(),
            rd: rd.id().into(),
        }
    }
}

#[derive(PackedStruct)]
#[packed_struct(endian = "msb")]
pub struct FloatingPointImmediate {
    m: bool,
    fixed: bool,
    s: bool,
    fixed2: Integer<u8, packed_bits::Bits<5>>,
    ptype: Integer<u8, packed_bits::Bits<2>>,
    fixed3: bool,
    imm8: u8,
    fixed4: Integer<u8, packed_bits::Bits<3>>,
    imm5: Integer<u8, packed_bits::Bits<5>>,
    rd: Integer<u8, packed_bits::Bits<5>>,
}

impl Aarch64Bytes for FloatingPointImmediate {}

pub struct FloatingPointImmediateParams {
    ptype: FloatWidth,
    imm8: u8,
    rd: AArch64FloatReg,
}

impl FloatingPointImmediate {
    #[inline(always)]
    fn new(FloatingPointImmediateParams { ptype, imm8, rd }: FloatingPointImmediateParams) -> Self {
        Self {
            m: false,
            fixed: false,
            s: false,
            fixed2: 0b11110.into(),
            ptype: encode_float_width(ptype).into(),
            fixed3: true,
            imm8,
            fixed4: 0b100.into(),
            imm5: 0b00000.into(),
            rd: rd.id().into(),
        }
    }
}

// Below here are the functions for all of the base assembly instructions.
// Their names are based on the instruction and operators combined.
// You should call `buf.reserve()` if you push or extend more than once.
// Unit tests are added at the bottom of the file to ensure correct asm generation.
// Please keep these in alphanumeric order.
// Floating-point (and advanced SIMD) instructions are at the bottom.

// ARM manual section C6

/// `ADD Xd, Xn, imm12` -> Add Xn and imm12 and place the result into Xd.
#[inline(always)]
fn add_reg64_reg64_imm12(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src: AArch64GeneralReg,
    imm12: u16,
) {
    let inst = ArithmeticImmediate::new(ArithmeticImmediateParams {
        op: false,
        s: false,
        rd: dst,
        rn: src,
        imm12,
        sh: false,
    });

    buf.extend(inst.bytes());
}

/// `ADD Xd, Xm, Xn` -> Add Xm and Xn and place the result into Xd.
#[inline(always)]
fn add_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = ArithmeticShifted::new(ArithmeticShiftedParams {
        op: false,
        s: false,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `AND Xd, Xn, Xm` -> Bitwise AND Xn and Xm and place the result into Xd.
#[inline(always)]
fn and_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = LogicalShiftedRegister::new(LogicalShiftedRegisterParams {
        op: LogicalOp::AND,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `ASR Xd, Xn, Xn` -> Arithmetic shift right Xn by Xm and place the result into Xd.
#[inline(always)]
fn asr_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = DataProcessingTwoSource::new(DataProcessingTwoSourceParams {
        op: 0b001010,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `B.cond imm19` -> Jump to PC + imm19 if cond is met.
#[inline(always)]
fn b_cond_imm19(buf: &mut Vec<'_, u8>, cond: ConditionCode, imm19: i32) {
    // Since instructions are 4 bytes, the branch instructions assume the last 2 bits are 0
    debug_assert!(imm19 & 0b11 == 0, "branch location must be 4-byte aligned");
    let shifted = imm19 >> 2;
    let unsigned = shifted as u32;
    // Our offset is only 19 bits, so we need to remove the first 13 bits
    let left_removed = (unsigned << 13) >> 13;
    // Check that imm19 wasn't too big
    if imm19 >= 0 {
        // Removing the first 13 bits should not have changed the value
        debug_assert!(left_removed == unsigned);
    } else {
        // If imm19 was negative, left_removed will be sign-extended by the instruction
        debug_assert!(left_removed | 0b1111_1111_1111_1100_0000_0000_0000_0000 == unsigned);
    }

    let inst = ConditionalBranchImmediate::new(ConditionalBranchImmediateParams {
        cond,
        imm19: left_removed,
    });

    buf.extend(inst.bytes());
}

/// `B imm26` -> Jump to PC + imm26.
#[inline(always)]
fn b_imm26(buf: &mut Vec<'_, u8>, imm26: i32) {
    // Since instructions are 4 bytes, the branch instructions assume the last 2 bits are 0
    debug_assert!(imm26 & 0b11 == 0, "branch location must be 4-byte aligned");
    let shifted = imm26 >> 2;
    let unsigned = shifted as u32;
    // Our offset is only 26 bits, so we need to remove the first 6 bits
    let left_removed = (unsigned << 6) >> 6;
    // Check that imm26 wasn't too big
    if imm26 >= 0 {
        // Removing the first 6 bits should not have changed the value
        debug_assert!(left_removed == unsigned);
    } else {
        // If imm26 was negative, left_removed will be sign-extended by the instruction
        debug_assert!(left_removed | 0b1111_1110_0000_0000_0000_0000_0000_0000 == unsigned);
    }

    let inst = UnconditionalBranchImmediate::new(UnconditionalBranchImmediateParams {
        op: false,
        imm26: left_removed,
    });

    buf.extend(inst.bytes());
}

/// `CMP Xn, imm12` -> Compare Xn and imm12, setting condition flags.
#[inline(always)]
fn cmp_reg64_imm12(buf: &mut Vec<'_, u8>, src: AArch64GeneralReg, imm12: u16) {
    subs_reg64_reg64_imm12(buf, AArch64GeneralReg::ZRSP, src, imm12);
}

/// `CMP Xn, Xm` -> Compare Xn and Xm, setting condition flags.
#[inline(always)]
fn cmp_reg64_reg64(buf: &mut Vec<'_, u8>, src1: AArch64GeneralReg, src2: AArch64GeneralReg) {
    subs_reg64_reg64_reg64(buf, AArch64GeneralReg::ZRSP, src1, src2);
}

/// `CNEG Xd, Xn, cond` -> If cond is true, then Xd = -Xn, else Xd = Xn.
#[inline(always)]
fn cneg_reg64_reg64_cond(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src: AArch64GeneralReg,
    cond: ConditionCode,
) {
    csneg_reg64_reg64_reg64_cond(buf, dst, src, src, cond.invert());
}

/// `CSET Xd, cond` -> If cond is true, then Xd = 1, else Xd = 0.
#[inline(always)]
fn cset_reg64_cond(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, cond: ConditionCode) {
    csinc_reg64_reg64_reg64_cond(
        buf,
        dst,
        AArch64GeneralReg::ZRSP,
        AArch64GeneralReg::ZRSP,
        cond.invert(),
    );
}

/// `CSINC Xd, Xn, Xm, cond` -> If cond is true, then Xd = Xn, else Xd = Xm + 1.
#[inline(always)]
fn csinc_reg64_reg64_reg64_cond(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
    cond: ConditionCode,
) {
    let inst = ConditionalSelect::new(ConditionalSelectParams {
        op: false,
        s: false,
        cond,
        op2: 0b01,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `CSNEG Xd, Xn, Xm, cond` -> If cond is true, then Xd = Xn, else Xd = -Xm.
#[inline(always)]
fn csneg_reg64_reg64_reg64_cond(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
    cond: ConditionCode,
) {
    let inst = ConditionalSelect::new(ConditionalSelectParams {
        op: true,
        s: false,
        cond,
        op2: 0b01,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `EOR Xd, Xn, Xm` -> Bitwise XOR Xn and Xm and place the result into Xd.
#[inline(always)]
fn eor_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = LogicalShiftedRegister::new(LogicalShiftedRegisterParams {
        op: LogicalOp::EOR,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `LDR Xt, [Xn, #offset]` -> Load Xn + Offset Xt. ZRSP is SP.
/// Note: imm12 is the offest divided by 8.
#[inline(always)]
fn ldr_reg64_reg64_imm12(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    base: AArch64GeneralReg,
    imm12: u16,
) {
    let inst = LoadStoreRegisterImmediate::new_load(LoadStoreRegisterImmediateParams {
        size: 0b11,
        imm12,
        rn: base,
        rt: dst,
    });

    buf.extend(inst.bytes());
}

/// `LSL Xd, Xn, Xm` -> Logical shift Xn left by Xm and place the result into Xd.
#[inline(always)]
fn lsl_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = DataProcessingTwoSource::new(DataProcessingTwoSourceParams {
        op: 0b001000,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `LSR Xd, Xn, Xm` -> Logical shift Xn right by Xm and place the result into Xd.
#[inline(always)]
fn lsr_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = DataProcessingTwoSource::new(DataProcessingTwoSourceParams {
        op: 0b001001,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `MADD Xd, Xn, Xm, Xa` -> Multiply Xn and Xm, add Xa, and place the result into Xd.
#[inline(always)]
fn madd_reg64_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
    src3: AArch64GeneralReg,
) {
    let inst = DataProcessingThreeSource::new(DataProcessingThreeSourceParams {
        op31: 0b000000,
        rm: src2,
        ra: src3,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `MOV Xd, Xm` -> Move Xm to Xd.
#[inline(always)]
fn mov_reg64_reg64(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, src: AArch64GeneralReg) {
    // MOV is equivalent to `ORR Xd, XZR, Xm` in AARCH64.
    orr_reg64_reg64_reg64(buf, dst, AArch64GeneralReg::ZRSP, src);
}

/// `MOVK Xd, imm16` -> Keeps Xd and moves an optionally shifted imm16 to Xd.
#[inline(always)]
fn movk_reg64_imm16(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, imm16: u16, hw: u8) {
    let inst = MoveWideImmediate::new(MoveWideImmediateParams {
        opc: 0b11,
        rd: dst,
        imm16,
        hw,
        sf: true,
    });

    buf.extend(inst.bytes());
}

/// `MOVZ Xd, imm16` -> Zeros Xd and moves an optionally shifted imm16 to Xd.
#[inline(always)]
fn movz_reg64_imm16(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, imm16: u16, hw: u8) {
    let inst = MoveWideImmediate::new(MoveWideImmediateParams {
        opc: 0b10,
        rd: dst,
        imm16,
        hw,
        sf: true,
    });

    buf.extend(inst.bytes());
}

/// `MUL Xd, Xn, Xm` -> Multiply Xn and Xm and place the result into Xd.
#[inline(always)]
fn mul_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    madd_reg64_reg64_reg64_reg64(buf, dst, src1, src2, AArch64GeneralReg::ZRSP);
}

/// `NEG Xd, Xm` -> Negate Xm and place the result into Xd.
#[inline(always)]
fn neg_reg64_reg64(buf: &mut Vec<'_, u8>, dst: AArch64GeneralReg, src: AArch64GeneralReg) {
    sub_reg64_reg64_reg64(buf, dst, AArch64GeneralReg::ZRSP, src);
}

/// `ORR Xd, Xn, Xm` -> Bitwise OR Xn and Xm and place the result into Xd.
#[inline(always)]
fn orr_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = LogicalShiftedRegister::new(LogicalShiftedRegisterParams {
        op: LogicalOp::ORR,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `SDIV Xd, Xn, Xm` -> Divide Xn by Xm and place the result into Xd.
/// Xn, Xm, and Xd are signed integers.
#[inline(always)]
fn sdiv_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = DataProcessingTwoSource::new(DataProcessingTwoSourceParams {
        op: 0b000011,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `STR Xt, [Xn, #offset]` -> Store Xt to Xn + Offset. ZRSP is SP.
/// Note: imm12 is the offest divided by 8.
#[inline(always)]
fn str_reg64_reg64_imm12(
    buf: &mut Vec<'_, u8>,
    src: AArch64GeneralReg,
    base: AArch64GeneralReg,
    imm12: u16,
) {
    let inst = LoadStoreRegisterImmediate::new_store(LoadStoreRegisterImmediateParams {
        size: 0b11,
        imm12,
        rn: base,
        rt: src,
    });

    buf.extend(inst.bytes());
}

/// `SUB Xd, Xn, imm12` -> Subtract Xn and imm12 and place the result into Xd.
#[inline(always)]
fn sub_reg64_reg64_imm12(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src: AArch64GeneralReg,
    imm12: u16,
) {
    let inst = ArithmeticImmediate::new(ArithmeticImmediateParams {
        op: true,
        s: false,
        rd: dst,
        rn: src,
        imm12,
        sh: false,
    });

    buf.extend(inst.bytes());
}

/// `SUB Xd, Xm, Xn` -> Subtract Xm and Xn and place the result into Xd.
#[inline(always)]
fn sub_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = ArithmeticShifted::new(ArithmeticShiftedParams {
        op: true,
        s: false,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `SUBS Xd, Xn, imm12` -> Subtract Xn and imm12 and place the result into Xd. Set condition flags.
#[inline(always)]
fn subs_reg64_reg64_imm12(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src: AArch64GeneralReg,
    imm12: u16,
) {
    let inst = ArithmeticImmediate::new(ArithmeticImmediateParams {
        op: true,
        s: true,
        rd: dst,
        rn: src,
        imm12,
        sh: false,
    });

    buf.extend(inst.bytes());
}

/// `SUBS Xd, Xn, Xm` -> Subtract Xn and Xm and place the result into Xd. Set condition flags.
#[inline(always)]
fn subs_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = ArithmeticShifted::new(ArithmeticShiftedParams {
        op: true,
        s: true,
        shift: ShiftType::LSL,
        imm6: 0,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

/// `RET Xn` -> Return to the address stored in Xn.
#[inline(always)]
fn ret_reg64(buf: &mut Vec<'_, u8>, xn: AArch64GeneralReg) {
    let inst =
        UnconditionalBranchRegister::new(UnconditionalBranchRegisterParams { op: 0b10, rn: xn });

    buf.extend(inst.bytes());
}

/// `UDIV Xd, Xn, Xm` -> Divide Xn by Xm and place the result into Xd.
/// Xn, Xm, and Xd are unsigned integers.
#[inline(always)]
fn udiv_reg64_reg64_reg64(
    buf: &mut Vec<'_, u8>,
    dst: AArch64GeneralReg,
    src1: AArch64GeneralReg,
    src2: AArch64GeneralReg,
) {
    let inst = DataProcessingTwoSource::new(DataProcessingTwoSourceParams {
        op: 0b000010,
        rm: src2,
        rn: src1,
        rd: dst,
    });

    buf.extend(inst.bytes());
}

// Floating point (and advanced SIMD) instructions
// ARM manual section C7

/// `FABS Sd/Dd, Sn/Dn` -> Take the absolute value of Sn/Dn and place the result into Sd/Dd.
#[inline(always)]
fn fabs_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingOneSource::new(FloatingPointDataProcessingOneSourceParams {
            opcode: 0b000001,
            ptype: ftype,
            rd: dst,
            rn: src,
        });

    buf.extend(inst.bytes());
}

/// `FADD Sd/Dd, Sn/Dn, Sm/Dm` -> Add Sn/Dn and Sm/Dm and place the result into Sd/Dd.
#[inline(always)]
fn fadd_freg_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src1: AArch64FloatReg,
    src2: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingTwoSource::new(FloatingPointDataProcessingTwoSourceParams {
            opcode: 0b0010,
            ptype: ftype,
            rd: dst,
            rn: src1,
            rm: src2,
        });

    buf.extend(inst.bytes());
}

/// `FCMP Sn/Dn, Sm/Dm` -> Compare Sn/Dn and Sm/Dm, setting condition flags.
#[inline(always)]
fn fcmp_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    src1: AArch64FloatReg,
    src2: AArch64FloatReg,
) {
    let inst = FloatingPointCompare::new(FloatingPointCompareParams {
        ptype: ftype,
        rn: src1,
        rm: src2,
        opcode2: 0b00000,
    });

    buf.extend(inst.bytes());
}

/// `FCVT Sd, Dn` -> Convert 64-bit float Dn to 32-bit float Sd.
#[inline(always)]
fn fcvt_freg32_freg64(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
    let inst =
        FloatingPointDataProcessingOneSource::new(FloatingPointDataProcessingOneSourceParams {
            opcode: 0b000100,
            ptype: FloatWidth::F64,
            rd: dst,
            rn: src,
        });

    buf.extend(inst.bytes());
}

/// `FCVT Dd, Sn` -> Convert 32-bit float Sn to 64-bit float Dd.
#[inline(always)]
fn fcvt_freg64_freg32(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg, src: AArch64FloatReg) {
    let inst =
        FloatingPointDataProcessingOneSource::new(FloatingPointDataProcessingOneSourceParams {
            opcode: 0b000101,
            ptype: FloatWidth::F32,
            rd: dst,
            rn: src,
        });

    buf.extend(inst.bytes());
}

/// `FDIV Sd/Dd, Sn/Dn, Sm/Dm` -> Divide Sn/Dn by Sm/Dm and place the result into Sd/Dd.
#[inline(always)]
fn fdiv_freg_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src1: AArch64FloatReg,
    src2: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingTwoSource::new(FloatingPointDataProcessingTwoSourceParams {
            opcode: 0b0001,
            ptype: ftype,
            rd: dst,
            rn: src1,
            rm: src2,
        });

    buf.extend(inst.bytes());
}

/// `FMOV Sd/Dd, Sn/Dn` -> Move Sn/Dn to Sd/Dd.
#[inline(always)]
fn fmov_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingOneSource::new(FloatingPointDataProcessingOneSourceParams {
            opcode: 0b000000,
            ptype: ftype,
            rd: dst,
            rn: src,
        });

    buf.extend(inst.bytes());
}

/// Encode a 32-bit float into an 8-bit immediate for FMOV.
/// See Table C2-1 in the ARM manual for a table of every float that can be encoded in 8 bits.
/// If the float cannot be encoded, return None.
/// This operation is the inverse of VFPExpandImm in the ARM manual.
#[inline(always)]
fn encode_f32_to_imm8(imm: f32) -> Option<u8> {
    let n = 32;
    let e = 8; // number of exponent bits in a 32-bit float
    let f = n - e - 1; // 23: number of fraction bits in a 32-bit float

    let bits = imm.to_bits();

    let sign = (bits >> (n - 1)) & 1; // bits<31>
    let exp = (bits >> f) & ((1 << e) - 1); // bits<30:23>
    let frac = bits & ((1 << f) - 1); // bits<22:0>

    let exp_first = (exp >> (e - 1)) & 1; // exp<7>
    let exp_middle = (exp >> 2) & ((1 << (e - 3)) - 1); // exp<6:2>
    let exp_last = exp & 0b11; // exp<1:0>
    if exp_first == 0 && exp_middle != ((1 << (e - 3)) - 1) {
        // If exp_first is 0, exp_middle must be all 1s.
        return None;
    }
    if exp_first == 1 && exp_middle != 0 {
        // If exp_first is 1, exp_middle must be all 0s.
        return None;
    }

    let frac_begin = frac >> (f - 4); // frac<22:19>
    let frac_end = frac & ((1 << (f - 4)) - 1); // frac<18:0>
    if frac_end != 0 {
        // frac_end must be all 0s.
        return None;
    }

    // The sign is the same.
    let ret_sign = sign << 7;
    // The first bit of the exponent is inverted.
    let ret_exp_first = (exp_first ^ 1) << 6;
    // The rest of the exponent is the same as the last 2 bits of the original exponent.
    let ret_exp_last = exp_last << 4;
    // The fraction is the same as the first 4 bits of the original fraction.
    let ret_frac = frac_begin;

    Some((ret_sign | ret_exp_first | ret_exp_last | ret_frac) as u8)
}

/// Encode a 64-bit float into an 8-bit immediate for FMOV.
/// See Table C2-1 in the ARM manual for a table of every float that can be encoded in 8 bits.
/// If the float cannot be encoded, return None.
/// This operation is the inverse of VFPExpandImm in the ARM manual.
#[inline(always)]
fn encode_f64_to_imm8(imm: f64) -> Option<u8> {
    let n = 64;
    let e = 11; // number of exponent bits in a 64-bit float
    let f = n - e - 1; // 52: number of fraction bits in a 64-bit float

    let bits = imm.to_bits();

    let sign = (bits >> (n - 1)) & 1; // bits<63>
    let exp = (bits >> f) & ((1 << e) - 1); // bits<62:52>
    let frac = bits & ((1 << f) - 1); // bits<51:0>

    let exp_first = (exp >> (e - 1)) & 1; // exp<10>
    let exp_middle = (exp >> 2) & ((1 << (e - 3)) - 1); // exp<9:2>
    let exp_last = exp & 0b11; // exp<0:1>
    if exp_first == 0 && exp_middle != ((1 << (e - 3)) - 1) {
        // If exp_first is 0, exp_middle must be all 1s.
        return None;
    }
    if exp_first == 1 && exp_middle != 0 {
        // If exp_first is 1, exp_middle must be all 0s.
        return None;
    }

    let frac_begin = frac >> (f - 4); // frac<51:48>
    let frac_end = frac & ((1 << (f - 4)) - 1); // frac<47:0>
    if frac_end != 0 {
        // frac_end must be all 0s.
        return None;
    }

    // The sign is the same.
    let ret_sign = sign << 7;
    // The first bit of the exponent is inverted.
    let ret_exp_first = (exp_first ^ 1) << 6;
    // The rest of the exponent is the same as the last 2 bits of the original exponent.
    let ret_exp_last = exp_last << 4;
    // The fraction is the same as the first 4 bits of the original fraction.
    let ret_frac = frac_begin;

    Some((ret_sign | ret_exp_first | ret_exp_last | ret_frac) as u8)
}

/// `FMOV Sd/Dd, imm8` -> Move imm8 to a float register.
/// imm8 is a float encoded using encode_f32_to_imm8 or encode_f64_to_imm8.
#[inline(always)]
fn fmov_freg_imm8(buf: &mut Vec<'_, u8>, ftype: FloatWidth, dst: AArch64FloatReg, imm8: u8) {
    let inst = FloatingPointImmediate::new(FloatingPointImmediateParams {
        ptype: ftype,
        rd: dst,
        imm8,
    });

    buf.extend(inst.bytes());
}

/// `FMUL Sd/Dd, Sn/Dn, Sm/Dm` -> Multiply Sn/Dn by Sm/Dm and store the result in Sd/Dd.
#[inline(always)]
fn fmul_freg_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src1: AArch64FloatReg,
    src2: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingTwoSource::new(FloatingPointDataProcessingTwoSourceParams {
            opcode: 0b0000,
            ptype: ftype,
            rd: dst,
            rn: src1,
            rm: src2,
        });

    buf.extend(inst.bytes());
}

/// `FSQRT Sd/Dd, Sn/Dn` -> Compute the square root of Sn/Dn and store the result in Sd/Dd.
#[inline(always)]
fn fsqrt_freg_freg(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src: AArch64FloatReg,
) {
    let inst =
        FloatingPointDataProcessingOneSource::new(FloatingPointDataProcessingOneSourceParams {
            opcode: 0b000011,
            ptype: ftype,
            rd: dst,
            rn: src,
        });

    buf.extend(inst.bytes());
}

/// Currently, we're only using MOVI to set a float register to 0.0.
/// `MOVI Dd, #0.0` -> Move 0.0 to Dd
#[inline(always)]
fn movi_freg_zero(buf: &mut Vec<'_, u8>, dst: AArch64FloatReg) {
    let inst = AdvancedSimdModifiedImmediate::new(dst);

    buf.extend(inst.bytes());
}

/// `SCVTF Sd/Dd, Xn` -> Convert Xn to a float and store the result in Sd/Dd.
#[inline(always)]
fn scvtf_freg_reg64(
    buf: &mut Vec<'_, u8>,
    ftype: FloatWidth,
    dst: AArch64FloatReg,
    src: AArch64GeneralReg,
) {
    let inst = ConversionBetweenFloatingPointAndInteger::new(
        ConversionBetweenFloatingPointAndIntegerParams {
            opcode: 0b010,
            rmode: 0b00,
            ptype: ftype,
            rd: dst,
            rn: src,
        },
    );

    buf.extend(inst.bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disassembler_test;
    use capstone::prelude::*;

    enum ZRSPKind {
        UsesZR,
        UsesSP,
    }
    use ZRSPKind::*;

    impl AArch64GeneralReg {
        fn capstone_string(&self, zrsp_kind: ZRSPKind) -> String {
            match self {
                AArch64GeneralReg::XR => "x8".to_owned(),
                AArch64GeneralReg::IP0 => "x16".to_owned(),
                AArch64GeneralReg::IP1 => "x17".to_owned(),
                AArch64GeneralReg::PR => "x18".to_owned(),
                AArch64GeneralReg::FP => "x29".to_owned(),
                AArch64GeneralReg::LR => "x30".to_owned(),
                AArch64GeneralReg::ZRSP => match zrsp_kind {
                    UsesZR => "xzr".to_owned(),
                    UsesSP => "sp".to_owned(),
                },
                _ => format!("{}", self),
            }
        }
    }

    impl AArch64FloatReg {
        fn capstone_string(&self, float_type: FloatWidth) -> String {
            match float_type {
                FloatWidth::F32 => format!("s{}", self.id()),
                FloatWidth::F64 => format!("d{}", self.id()),
            }
        }
    }

    const TEST_U16: u16 = 0x1234;
    //const TEST_I32: i32 = 0x12345678;
    //const TEST_I64: i64 = 0x12345678_9ABCDEF0;

    const ALL_GENERAL_REGS: &[AArch64GeneralReg] = &[
        AArch64GeneralReg::X0,
        AArch64GeneralReg::X1,
        AArch64GeneralReg::X2,
        AArch64GeneralReg::X3,
        AArch64GeneralReg::X4,
        AArch64GeneralReg::X5,
        AArch64GeneralReg::X6,
        AArch64GeneralReg::X7,
        AArch64GeneralReg::XR,
        AArch64GeneralReg::X9,
        AArch64GeneralReg::X10,
        AArch64GeneralReg::X11,
        AArch64GeneralReg::X12,
        AArch64GeneralReg::X13,
        AArch64GeneralReg::X14,
        AArch64GeneralReg::X15,
        AArch64GeneralReg::IP0,
        AArch64GeneralReg::IP1,
        AArch64GeneralReg::PR,
        AArch64GeneralReg::X19,
        AArch64GeneralReg::X20,
        AArch64GeneralReg::X21,
        AArch64GeneralReg::X22,
        AArch64GeneralReg::X23,
        AArch64GeneralReg::X24,
        AArch64GeneralReg::X25,
        AArch64GeneralReg::X26,
        AArch64GeneralReg::X27,
        AArch64GeneralReg::X28,
        AArch64GeneralReg::FP,
        AArch64GeneralReg::LR,
        AArch64GeneralReg::ZRSP,
    ];

    const ALL_FLOAT_REGS: &[AArch64FloatReg] = &[
        AArch64FloatReg::V0,
        AArch64FloatReg::V1,
        AArch64FloatReg::V2,
        AArch64FloatReg::V3,
        AArch64FloatReg::V4,
        AArch64FloatReg::V5,
        AArch64FloatReg::V6,
        AArch64FloatReg::V7,
        AArch64FloatReg::V8,
        AArch64FloatReg::V9,
        AArch64FloatReg::V10,
        AArch64FloatReg::V11,
        AArch64FloatReg::V12,
        AArch64FloatReg::V13,
        AArch64FloatReg::V14,
        AArch64FloatReg::V15,
        AArch64FloatReg::V16,
        AArch64FloatReg::V17,
        AArch64FloatReg::V18,
        AArch64FloatReg::V19,
        AArch64FloatReg::V20,
        AArch64FloatReg::V21,
        AArch64FloatReg::V22,
        AArch64FloatReg::V23,
        AArch64FloatReg::V24,
        AArch64FloatReg::V25,
        AArch64FloatReg::V26,
        AArch64FloatReg::V27,
        AArch64FloatReg::V28,
        AArch64FloatReg::V29,
        AArch64FloatReg::V30,
        AArch64FloatReg::V31,
    ];

    const ALL_FLOAT_TYPES: &[FloatWidth] = &[FloatWidth::F32, FloatWidth::F64];

    const ALL_CONDITIONS: &[ConditionCode] = &[
        ConditionCode::EQ,
        ConditionCode::NE,
        ConditionCode::CSHS,
        ConditionCode::CCLO,
        ConditionCode::MI,
        ConditionCode::PL,
        ConditionCode::VS,
        ConditionCode::VC,
        ConditionCode::HI,
        ConditionCode::LS,
        ConditionCode::GE,
        ConditionCode::LT,
        ConditionCode::GT,
        ConditionCode::LE,
        ConditionCode::AL,
    ];

    fn setup_capstone_and_arena<T>(
        arena: &bumpalo::Bump,
    ) -> (bumpalo::collections::Vec<T>, Capstone) {
        let buf = bumpalo::vec![in arena];
        let cs = Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .detail(true)
            .build()
            .expect("Failed to create Capstone object");
        (buf, cs)
    }

    // Many of these instructions are aliases for each other,
    // and depending on their arguments, they might get disassembled to a different instruction.
    // That's why we need `if` expressions in some of these tests.
    // The "alias conditions" for each instruction are listed in the ARM manual.

    #[test]
    fn test_add_reg64_reg64_reg64() {
        disassembler_test!(
            add_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "add {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_add_reg64_reg64_imm12() {
        disassembler_test!(
            add_reg64_reg64_imm12,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, imm| format!(
                "add {}, {}, #0x{:x}",
                reg1.capstone_string(UsesSP),
                reg2.capstone_string(UsesSP),
                imm
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_and_reg64_reg64_reg64() {
        disassembler_test!(
            and_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "and {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_asr_reg64_reg64_reg64() {
        disassembler_test!(
            asr_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "asr {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_b_cond_imm19() {
        disassembler_test!(
            b_cond_imm19,
            |cond: ConditionCode, imm: i32| format!("b.{} #0x{:x}", cond, imm as i64),
            ALL_CONDITIONS,
            [0x120, -0x120, (1 << 20) - 4, -(1 << 20)]
        );
    }

    #[test]
    fn test_b_imm26() {
        disassembler_test!(
            b_imm26,
            |imm| format!("b #0x{:x}", imm as i64),
            [0x120, -0x120, (1 << 27) - 4, -(1 << 27)]
        );
    }

    #[test]
    fn test_cmp_reg64_imm12() {
        disassembler_test!(
            cmp_reg64_imm12,
            |reg1: AArch64GeneralReg, imm| format!(
                "cmp {}, #0x{:x}",
                reg1.capstone_string(UsesSP),
                imm
            ),
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_cmp_reg64_reg64() {
        disassembler_test!(
            cmp_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg| format!(
                "cmp {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_cneg_reg64_reg64_cond() {
        disassembler_test!(
            cneg_reg64_reg64_cond,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, cond: ConditionCode| {
                if cond == ConditionCode::AL {
                    format!(
                        "csneg {}, {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        cond.invert()
                    )
                } else {
                    format!(
                        "cneg {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        cond
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_CONDITIONS
        );
    }

    #[test]
    fn test_cset() {
        disassembler_test!(
            cset_reg64_cond,
            |reg1: AArch64GeneralReg, cond: ConditionCode| {
                if cond == ConditionCode::AL {
                    format!(
                        "csinc {}, xzr, xzr, {}",
                        reg1.capstone_string(UsesZR),
                        cond.invert()
                    )
                } else {
                    format!("cset {}, {}", reg1.capstone_string(UsesZR), cond)
                }
            },
            ALL_GENERAL_REGS,
            ALL_CONDITIONS
        );
    }

    #[test]
    fn test_csinc() {
        disassembler_test!(
            csinc_reg64_reg64_reg64_cond,
            |reg1: AArch64GeneralReg,
             reg2: AArch64GeneralReg,
             reg3: AArch64GeneralReg,
             cond: ConditionCode| {
                if reg3 != AArch64GeneralReg::ZRSP
                    && cond != ConditionCode::AL
                    && reg2 != AArch64GeneralReg::ZRSP
                    && reg2 == reg3
                {
                    format!(
                        "cinc {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        cond.invert()
                    )
                } else if reg3 == AArch64GeneralReg::ZRSP
                    && cond != ConditionCode::AL
                    && reg2 == AArch64GeneralReg::ZRSP
                {
                    format!("cset {}, {}", reg1.capstone_string(UsesZR), cond.invert())
                } else {
                    format!(
                        "csinc {}, {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR),
                        cond
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_CONDITIONS
        )
    }

    #[test]
    fn test_csneg() {
        disassembler_test!(
            csneg_reg64_reg64_reg64_cond,
            |reg1: AArch64GeneralReg,
             reg2: AArch64GeneralReg,
             reg3: AArch64GeneralReg,
             cond: ConditionCode| {
                if cond != ConditionCode::AL && reg2 == reg3 {
                    format!(
                        "cneg {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        cond.invert()
                    )
                } else {
                    format!(
                        "csneg {}, {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR),
                        cond
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_CONDITIONS
        )
    }

    #[test]
    fn test_eor_reg64_reg64_reg64() {
        disassembler_test!(
            eor_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "eor {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_ldr_reg64_reg64_imm12() {
        disassembler_test!(
            ldr_reg64_reg64_imm12,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, imm| format!(
                "ldr {}, [{}, #0x{:x}]",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesSP),
                imm << 3
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_lsl_reg64_reg64_reg64() {
        disassembler_test!(
            lsl_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "lsl {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_lsr_reg64_reg64_reg64() {
        disassembler_test!(
            lsr_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "lsr {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_madd_reg64_reg64_reg64_reg64() {
        disassembler_test!(
            madd_reg64_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg,
             reg2: AArch64GeneralReg,
             reg3: AArch64GeneralReg,
             reg4: AArch64GeneralReg| {
                if reg4 == AArch64GeneralReg::ZRSP {
                    format!(
                        "mul {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                } else {
                    format!(
                        "madd {}, {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR),
                        reg4.capstone_string(UsesZR)
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_mov_reg64_reg64() {
        disassembler_test!(
            mov_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg| format!(
                "mov {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_movk_reg64_imm16() {
        disassembler_test!(
            movk_reg64_imm16,
            |reg1: AArch64GeneralReg, imm, hw| format!(
                "movk {}, #0x{:x}{}",
                reg1.capstone_string(UsesZR),
                imm,
                if hw > 0 {
                    format!(", lsl #{}", hw * 16)
                } else {
                    "".to_owned()
                }
            ),
            ALL_GENERAL_REGS,
            [TEST_U16],
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn test_movz_reg64_imm16() {
        disassembler_test!(
            movz_reg64_imm16,
            |reg1: AArch64GeneralReg, imm, hw| format!(
                "mov {}, #0x{:x}{}",
                reg1.capstone_string(UsesZR),
                imm,
                "0000".repeat(hw as usize)
            ),
            ALL_GENERAL_REGS,
            [TEST_U16],
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn test_mul_reg64_reg64_reg64() {
        disassembler_test!(
            mul_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "mul {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_neg_reg64_reg64() {
        disassembler_test!(
            neg_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg| format!(
                "neg {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_orr_reg64_reg64_reg64() {
        disassembler_test!(
            orr_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| {
                if reg2 == AArch64GeneralReg::ZRSP {
                    format!(
                        "mov {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR),
                    )
                } else {
                    format!(
                        "orr {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR),
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_sdiv_reg64_reg64_reg64() {
        disassembler_test!(
            sdiv_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "sdiv {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_str_reg64_reg64_imm12() {
        disassembler_test!(
            str_reg64_reg64_imm12,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, imm| format!(
                "str {}, [{}, #0x{:x}]",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesSP),
                imm << 3
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_sub_reg64_reg64_imm12() {
        disassembler_test!(
            sub_reg64_reg64_imm12,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, imm| format!(
                "sub {}, {}, #0x{:x}",
                reg1.capstone_string(UsesSP),
                reg2.capstone_string(UsesSP),
                imm
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_sub_reg64_reg64_reg64() {
        disassembler_test!(
            sub_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| {
                if reg2 == AArch64GeneralReg::ZRSP {
                    // When the second register is ZR, it gets disassembled as neg,
                    // which is an alias for sub.
                    format!(
                        "neg {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                } else {
                    format!(
                        "sub {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]

    fn test_subs_reg64_reg64_imm12() {
        disassembler_test!(
            subs_reg64_reg64_imm12,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, imm| {
                if reg1 == AArch64GeneralReg::ZRSP {
                    // When the first register is SP, it gets disassembled as cmp,
                    // which is an alias for subs.
                    format!("cmp {}, #0x{:x}", reg2.capstone_string(UsesSP), imm)
                } else {
                    format!(
                        "subs {}, {}, #0x{:x}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesSP),
                        imm
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            [0x123]
        );
    }

    #[test]
    fn test_subs_reg64_reg64_reg64() {
        disassembler_test!(
            subs_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| {
                if reg1 == AArch64GeneralReg::ZRSP {
                    // When the first register is SP, it gets disassembled as cmp,
                    // which is an alias for subs.
                    format!(
                        "cmp {}, {}",
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                } else if reg2 == AArch64GeneralReg::ZRSP {
                    // When the second register is ZR, it gets disassembled as negs,
                    // which is an alias for subs.
                    format!(
                        "negs {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                } else {
                    format!(
                        "subs {}, {}, {}",
                        reg1.capstone_string(UsesZR),
                        reg2.capstone_string(UsesZR),
                        reg3.capstone_string(UsesZR)
                    )
                }
            },
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_ret_reg64() {
        disassembler_test!(
            ret_reg64,
            |reg1: AArch64GeneralReg| if reg1 == AArch64GeneralReg::LR {
                "ret".to_owned()
            } else {
                format!("ret {}", reg1.capstone_string(UsesZR))
            },
            ALL_GENERAL_REGS
        );
    }

    #[test]
    fn test_udiv_reg64_reg64_reg64() {
        disassembler_test!(
            udiv_reg64_reg64_reg64,
            |reg1: AArch64GeneralReg, reg2: AArch64GeneralReg, reg3: AArch64GeneralReg| format!(
                "udiv {}, {}, {}",
                reg1.capstone_string(UsesZR),
                reg2.capstone_string(UsesZR),
                reg3.capstone_string(UsesZR)
            ),
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS,
            ALL_GENERAL_REGS
        );
    }

    // Float instructions

    #[test]
    fn test_fabs_freg_freg() {
        disassembler_test!(
            fabs_freg_freg,
            |ftype: FloatWidth, reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fabs {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fadd_freg_freg_freg() {
        disassembler_test!(
            fadd_freg_freg_freg,
            |ftype: FloatWidth,
             reg1: AArch64FloatReg,
             reg2: AArch64FloatReg,
             reg3: AArch64FloatReg| format!(
                "fadd {}, {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype),
                reg3.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fcmp_freg_freg() {
        disassembler_test!(
            fcmp_freg_freg,
            |ftype: FloatWidth, reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fcmp {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fcvt_freg32_freg64() {
        disassembler_test!(
            fcvt_freg32_freg64,
            |reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fcvt {}, {}",
                reg1.capstone_string(FloatWidth::F32),
                reg2.capstone_string(FloatWidth::F64)
            ),
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fcvt_freg64_freg32() {
        disassembler_test!(
            fcvt_freg64_freg32,
            |reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fcvt {}, {}",
                reg1.capstone_string(FloatWidth::F64),
                reg2.capstone_string(FloatWidth::F32)
            ),
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fdiv_freg_freg_freg() {
        disassembler_test!(
            fdiv_freg_freg_freg,
            |ftype: FloatWidth,
             reg1: AArch64FloatReg,
             reg2: AArch64FloatReg,
             reg3: AArch64FloatReg| format!(
                "fdiv {}, {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype),
                reg3.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fmov_freg_freg() {
        disassembler_test!(
            fmov_freg_freg,
            |ftype: FloatWidth, reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fmov {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_encode_f32_to_imm8() {
        // See ARM manual Table C2-1: A64 Floating-point modified immediate constants
        assert_eq!(encode_f32_to_imm8(2.0), Some(0b0_000_0000));
        assert_eq!(encode_f32_to_imm8(4.0), Some(0b0_001_0000));
        assert_eq!(encode_f32_to_imm8(8.0), Some(0b0_010_0000));
        assert_eq!(encode_f32_to_imm8(16.0), Some(0b0_011_0000));
        assert_eq!(encode_f32_to_imm8(0.125), Some(0b0_100_0000));
        assert_eq!(encode_f32_to_imm8(0.25), Some(0b0_101_0000));
        assert_eq!(encode_f32_to_imm8(0.5), Some(0b0_110_0000));
        assert_eq!(encode_f32_to_imm8(1.0), Some(0b0_111_0000));

        assert_eq!(encode_f32_to_imm8(2.125), Some(0b0_000_0001));
        assert_eq!(encode_f32_to_imm8(2.25), Some(0b0_000_0010));
        assert_eq!(encode_f32_to_imm8(2.375), Some(0b0_000_0011));
        assert_eq!(encode_f32_to_imm8(2.5), Some(0b0_000_0100));
        assert_eq!(encode_f32_to_imm8(2.625), Some(0b0_000_0101));
        assert_eq!(encode_f32_to_imm8(2.75), Some(0b0_000_0110));
        assert_eq!(encode_f32_to_imm8(2.875), Some(0b0_000_0111));
        assert_eq!(encode_f32_to_imm8(3.0), Some(0b0_000_1000));
        assert_eq!(encode_f32_to_imm8(3.125), Some(0b0_000_1001));
        assert_eq!(encode_f32_to_imm8(3.25), Some(0b0_000_1010));
        assert_eq!(encode_f32_to_imm8(3.375), Some(0b0_000_1011));
        assert_eq!(encode_f32_to_imm8(3.5), Some(0b0_000_1100));
        assert_eq!(encode_f32_to_imm8(3.625), Some(0b0_000_1101));
        assert_eq!(encode_f32_to_imm8(3.75), Some(0b0_000_1110));
        assert_eq!(encode_f32_to_imm8(3.875), Some(0b0_000_1111));

        assert_eq!(encode_f32_to_imm8(-2.0), Some(0b1_000_0000));
        assert_eq!(encode_f32_to_imm8(-0.25), Some(0b1_101_0000));
        assert_eq!(encode_f32_to_imm8(-2.5), Some(0b1_000_0100));
        assert_eq!(encode_f32_to_imm8(-3.375), Some(0b1_000_1011));

        assert_eq!(encode_f32_to_imm8(1.9375), Some(0b0_111_1111));
        assert_eq!(encode_f32_to_imm8(-1.9375), Some(0b1_111_1111));

        assert_eq!(encode_f32_to_imm8(23.0), Some(0b0_011_0111));
        assert_eq!(encode_f32_to_imm8(-23.0), Some(0b1_011_0111));

        assert_eq!(encode_f32_to_imm8(0.0), None);
        assert_eq!(encode_f32_to_imm8(-0.0), None);
        assert_eq!(encode_f32_to_imm8(32.0), None);
        assert_eq!(encode_f32_to_imm8(-32.0), None);
        assert_eq!(encode_f32_to_imm8(0.0625), None);
        assert_eq!(encode_f32_to_imm8(-0.0625), None);
        assert_eq!(encode_f32_to_imm8(0.3), None);
        assert_eq!(encode_f32_to_imm8(-0.3), None);
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_encode_f64_to_imm8() {
        // See ARM manual Table C2-1: A64 Floating-point modified immediate constants
        assert_eq!(encode_f64_to_imm8(2.0), Some(0b0_000_0000));
        assert_eq!(encode_f64_to_imm8(4.0), Some(0b0_001_0000));
        assert_eq!(encode_f64_to_imm8(8.0), Some(0b0_010_0000));
        assert_eq!(encode_f64_to_imm8(16.0), Some(0b0_011_0000));
        assert_eq!(encode_f64_to_imm8(0.125), Some(0b0_100_0000));
        assert_eq!(encode_f64_to_imm8(0.25), Some(0b0_101_0000));
        assert_eq!(encode_f64_to_imm8(0.5), Some(0b0_110_0000));
        assert_eq!(encode_f64_to_imm8(1.0), Some(0b0_111_0000));

        assert_eq!(encode_f64_to_imm8(2.125), Some(0b0_000_0001));
        assert_eq!(encode_f64_to_imm8(2.25), Some(0b0_000_0010));
        assert_eq!(encode_f64_to_imm8(2.375), Some(0b0_000_0011));
        assert_eq!(encode_f64_to_imm8(2.5), Some(0b0_000_0100));
        assert_eq!(encode_f64_to_imm8(2.625), Some(0b0_000_0101));
        assert_eq!(encode_f64_to_imm8(2.75), Some(0b0_000_0110));
        assert_eq!(encode_f64_to_imm8(2.875), Some(0b0_000_0111));
        assert_eq!(encode_f64_to_imm8(3.0), Some(0b0_000_1000));
        assert_eq!(encode_f64_to_imm8(3.125), Some(0b0_000_1001));
        assert_eq!(encode_f64_to_imm8(3.25), Some(0b0_000_1010));
        assert_eq!(encode_f64_to_imm8(3.375), Some(0b0_000_1011));
        assert_eq!(encode_f64_to_imm8(3.5), Some(0b0_000_1100));
        assert_eq!(encode_f64_to_imm8(3.625), Some(0b0_000_1101));
        assert_eq!(encode_f64_to_imm8(3.75), Some(0b0_000_1110));
        assert_eq!(encode_f64_to_imm8(3.875), Some(0b0_000_1111));

        assert_eq!(encode_f64_to_imm8(-2.0), Some(0b1_000_0000));
        assert_eq!(encode_f64_to_imm8(-0.25), Some(0b1_101_0000));
        assert_eq!(encode_f64_to_imm8(-2.5), Some(0b1_000_0100));
        assert_eq!(encode_f64_to_imm8(-3.375), Some(0b1_000_1011));

        assert_eq!(encode_f64_to_imm8(1.9375), Some(0b0_111_1111));
        assert_eq!(encode_f64_to_imm8(-1.9375), Some(0b1_111_1111));

        assert_eq!(encode_f64_to_imm8(23.0), Some(0b0_011_0111));
        assert_eq!(encode_f64_to_imm8(-23.0), Some(0b1_011_0111));

        assert_eq!(encode_f64_to_imm8(0.0), None);
        assert_eq!(encode_f64_to_imm8(-0.0), None);
        assert_eq!(encode_f64_to_imm8(32.0), None);
        assert_eq!(encode_f64_to_imm8(-32.0), None);
        assert_eq!(encode_f64_to_imm8(0.0625), None);
        assert_eq!(encode_f64_to_imm8(-0.0625), None);
        assert_eq!(encode_f64_to_imm8(0.3), None);
        assert_eq!(encode_f64_to_imm8(-0.3), None);
    }

    #[test]
    fn test_fmov_freg_imm8() {
        disassembler_test!(
            |buf: &mut Vec<'_, u8>, ftype: FloatWidth, dst: AArch64FloatReg, imm: f32| {
                // We need to encode the float immediate to 8 bits first.
                let encoded = match ftype {
                    FloatWidth::F32 => encode_f32_to_imm8(imm),
                    FloatWidth::F64 => encode_f64_to_imm8(imm as f64),
                };
                fmov_freg_imm8(buf, ftype, dst, encoded.unwrap())
            },
            |ftype: FloatWidth, reg: AArch64FloatReg, imm: f32| format!(
                "fmov {}, #{:.8}",
                reg.capstone_string(ftype),
                imm
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            [
                // These are all of the possible values that can be encoded in an 8-bit float immediate.
                // See ARM manual Table C2-1: A64 Floating-point modified immediate constants.
                2.0, 4.0, 8.0, 16.0, 0.125, 0.25, 0.5, 1.0, 2.125, 4.25, 8.5, 17.0, 0.1328125,
                0.265625, 0.53125, 1.0625, 2.25, 4.5, 9.0, 18.0, 0.140625, 0.28125, 0.5625, 1.125,
                2.375, 4.75, 9.5, 19.0, 0.1484375, 0.296875, 0.59375, 1.1875, 2.5, 5.0, 10.0, 20.0,
                0.15625, 0.3125, 0.625, 1.25, 2.625, 5.25, 10.5, 21.0, 0.1640625, 0.328125,
                0.65625, 1.3125, 2.75, 5.5, 11.0, 22.0, 0.171875, 0.34375, 0.6875, 1.375, 2.875,
                5.75, 11.5, 23.0, 0.1796875, 0.359375, 0.71875, 1.4375, 3.0, 6.0, 12.0, 24.0,
                0.1875, 0.375, 0.75, 1.5, 3.125, 6.25, 12.5, 25.0, 0.1953125, 0.390625, 0.78125,
                1.5625, 3.25, 6.5, 13.0, 26.0, 0.203125, 0.40625, 0.8125, 1.625, 3.375, 6.75, 13.5,
                27.0, 0.2109375, 0.421875, 0.84375, 1.6875, 3.5, 7.0, 14.0, 28.0, 0.21875, 0.4375,
                0.875, 1.75, 3.625, 7.25, 14.5, 29.0, 0.2265625, 0.453125, 0.90625, 1.8125, 3.75,
                7.5, 15.0, 30.0, 0.234375, 0.46875, 0.9375, 1.875, 3.875, 7.75, 15.5, 31.0,
                0.2421875, 0.484375, 0.96875, 1.9375, -2.0, -4.0, -8.0, -16.0, -0.125, -0.25, -0.5,
                -1.0, -2.125, -4.25, -8.5, -17.0, -0.1328125, -0.265625, -0.53125, -1.0625, -2.25,
                -4.5, -9.0, -18.0, -0.140625, -0.28125, -0.5625, -1.125, -2.375, -4.75, -9.5,
                -19.0, -0.1484375, -0.296875, -0.59375, -1.1875, -2.5, -5.0, -10.0, -20.0,
                -0.15625, -0.3125, -0.625, -1.25, -2.625, -5.25, -10.5, -21.0, -0.1640625,
                -0.328125, -0.65625, -1.3125, -2.75, -5.5, -11.0, -22.0, -0.171875, -0.34375,
                -0.6875, -1.375, -2.875, -5.75, -11.5, -23.0, -0.1796875, -0.359375, -0.71875,
                -1.4375, -3.0, -6.0, -12.0, -24.0, -0.1875, -0.375, -0.75, -1.5, -3.125, -6.25,
                -12.5, -25.0, -0.1953125, -0.390625, -0.78125, -1.5625, -3.25, -6.5, -13.0, -26.0,
                -0.203125, -0.40625, -0.8125, -1.625, -3.375, -6.75, -13.5, -27.0, -0.2109375,
                -0.421875, -0.84375, -1.6875, -3.5, -7.0, -14.0, -28.0, -0.21875, -0.4375, -0.875,
                -1.75, -3.625, -7.25, -14.5, -29.0, -0.2265625, -0.453125, -0.90625, -1.8125,
                -3.75, -7.5, -15.0, -30.0, -0.234375, -0.46875, -0.9375, -1.875, -3.875, -7.75,
                -15.5, -31.0, -0.2421875, -0.484375, -0.96875, -1.9375,
            ]
        );
    }

    #[test]
    fn test_fmul_freg_freg_freg() {
        disassembler_test!(
            fmul_freg_freg_freg,
            |ftype: FloatWidth,
             reg1: AArch64FloatReg,
             reg2: AArch64FloatReg,
             reg3: AArch64FloatReg| format!(
                "fmul {}, {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype),
                reg3.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_fsqrt_freg_freg() {
        disassembler_test!(
            fsqrt_freg_freg,
            |ftype: FloatWidth, reg1: AArch64FloatReg, reg2: AArch64FloatReg| format!(
                "fsqrt {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(ftype)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_movi_freg_zero() {
        disassembler_test!(
            movi_freg_zero,
            |reg: AArch64FloatReg| format!(
                "movi {}, #0000000000000000",
                reg.capstone_string(FloatWidth::F64)
            ),
            ALL_FLOAT_REGS
        );
    }

    #[test]
    fn test_scvtf_freg_reg64() {
        disassembler_test!(
            scvtf_freg_reg64,
            |ftype: FloatWidth, reg1: AArch64FloatReg, reg2: AArch64GeneralReg| format!(
                "scvtf {}, {}",
                reg1.capstone_string(ftype),
                reg2.capstone_string(UsesZR)
            ),
            ALL_FLOAT_TYPES,
            ALL_FLOAT_REGS,
            ALL_GENERAL_REGS
        );
    }
}
