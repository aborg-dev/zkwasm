use std::{env, fs};

use anyhow::Result;

use wasmparser::{BlockType, BrTable, Ieee32, Ieee64, MemArg, Payload::*};

fn parse(module: &[u8]) -> Result<()> {
    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(module) {
        match payload? {
            // Sections for WebAssembly modules
            Version { .. } => { /* ... */ }
            TypeSection(_) => { /* ... */ }
            ImportSection(_) => { /* ... */ }
            FunctionSection(_) => { /* ... */ }
            TableSection(_) => { /* ... */ }
            MemorySection(_) => { /* ... */ }
            TagSection(_) => { /* ... */ }
            GlobalSection(_) => { /* ... */ }
            ExportSection(_) => { /* ... */ }
            StartSection { .. } => { /* ... */ }
            ElementSection(_) => { /* ... */ }
            DataCountSection { .. } => { /* ... */ }
            DataSection(_) => { /* ... */ }

            // Here we know how many functions we'll be receiving as
            // `CodeSectionEntry`, so we can prepare for that, and
            // afterwards we can parse and handle each function
            // individually.
            CodeSectionStart { .. } => { /* ... */ }
            CodeSectionEntry(body) => {
                // here we can iterate over `body` to parse the function
                // and its locals
                dbg!(body);
            }

            // Sections for WebAssembly components
            ModuleSection { .. } => { /* ... */ }
            InstanceSection(_) => { /* ... */ }
            CoreTypeSection(_) => { /* ... */ }
            ComponentSection { .. } => { /* ... */ }
            ComponentInstanceSection(_) => { /* ... */ }
            ComponentAliasSection(_) => { /* ... */ }
            ComponentTypeSection(_) => { /* ... */ }
            ComponentCanonicalSection(_) => { /* ... */ }
            ComponentStartSection { .. } => { /* ... */ }
            ComponentImportSection(_) => { /* ... */ }
            ComponentExportSection(_) => { /* ... */ }

            CustomSection(_) => { /* ... */ }

            // most likely you'd return an error here
            UnknownSection { .. } => { /* ... */ }

            // Once we've reached the end of a parser we either resume
            // at the parent parser or the payload iterator is at its
            // end and we're done.
            End(_) => {}
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: main WASM_FILEPATH");
    }
    let module = fs::read_to_string(&args[1]).expect("Failed to read file");
    parse(module.as_bytes())?;
    Ok(())
}

macro_rules! define_visit_once {
    (@mvp $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {};
    (@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {
        fn $visit(&mut self $($(,$arg: $argty)*)?) {
            // do nothing for this example
        }
    };
}

macro_rules! define_visit_operator {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            define_visit_once! {
                @$proposal $op $({ $($arg: $argty),* })? => $visit
            }
         )*
    }
}

pub struct ZkCodegenVisitor;

impl<'a> wasmparser::VisitOperator<'a> for ZkCodegenVisitor {
    type Output = ();

    wasmparser::for_each_operator!(define_visit_operator);

    fn visit_unreachable(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_nop(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_block(&mut self, _blockty: BlockType) -> Self::Output {
        todo!()
    }

    fn visit_loop(&mut self, _blockty: BlockType) -> Self::Output {
        todo!()
    }

    fn visit_if(&mut self, _blockty: BlockType) -> Self::Output {
        todo!()
    }

    fn visit_else(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_end(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_br(&mut self, _relative_depth: u32) -> Self::Output {
        todo!()
    }

    fn visit_br_if(&mut self, _relative_depth: u32) -> Self::Output {
        todo!()
    }

    fn visit_br_table(&mut self, _targets: BrTable<'a>) -> Self::Output {
        todo!()
    }

    fn visit_return(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_call(&mut self, _function_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_call_indirect(
        &mut self,
        _type_index: u32,
        _table_index: u32,
        _table_byte: u8,
    ) -> Self::Output {
        todo!()
    }

    fn visit_drop(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_select(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_local_get(&mut self, _local_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_local_set(&mut self, _local_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_local_tee(&mut self, _local_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_global_get(&mut self, _global_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_global_set(&mut self, _global_index: u32) -> Self::Output {
        todo!()
    }

    fn visit_i32_load(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_f32_load(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_f64_load(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_load8_s(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_load8_u(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_load16_s(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_load16_u(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load8_s(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load8_u(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load16_s(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load16_u(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load32_s(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_load32_u(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_store(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_store(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_f32_store(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_f64_store(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_store8(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i32_store16(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_store8(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_store16(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_i64_store32(&mut self, _mamarg: MemArg) -> Self::Output {
        todo!()
    }

    fn visit_memory_size(&mut self, _mem: u32, _mem_byte: u8) -> Self::Output {
        todo!()
    }

    fn visit_memory_grow(&mut self, _mem: u32, _mem_byte: u8) -> Self::Output {
        todo!()
    }

    fn visit_i32_const(&mut self, _value: i32) -> Self::Output {
        todo!()
    }

    fn visit_i64_const(&mut self, _value: i64) -> Self::Output {
        todo!()
    }

    fn visit_f32_const(&mut self, _value: Ieee32) -> Self::Output {
        todo!()
    }

    fn visit_f64_const(&mut self, _value: Ieee64) -> Self::Output {
        todo!()
    }

    fn visit_i32_eqz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_eq(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_ne(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_lt_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_lt_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_gt_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_gt_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_le_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_le_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_ge_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_ge_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_eqz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_eq(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_ne(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_lt_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_lt_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_gt_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_gt_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_le_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_le_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_ge_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_ge_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_eq(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_ne(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_lt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_gt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_le(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_ge(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_eq(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_ne(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_lt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_gt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_le(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_ge(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_clz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_ctz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_popcnt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_add(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_sub(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_mul(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_div_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_div_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_rem_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_rem_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_and(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_or(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_xor(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_shl(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_shr_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_shr_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_rotl(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_rotr(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_clz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_ctz(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_popcnt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_add(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_sub(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_mul(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_div_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_div_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_rem_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_rem_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_and(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_or(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_xor(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_shl(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_shr_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_shr_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_rotl(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_rotr(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_abs(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_neg(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_ceil(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_floor(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_trunc(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_nearest(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_sqrt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_add(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_sub(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_mul(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_div(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_min(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_max(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_copysign(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_abs(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_neg(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_ceil(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_floor(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_trunc(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_nearest(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_sqrt(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_add(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_sub(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_mul(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_div(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_min(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_max(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_copysign(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_wrap_i64(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_trunc_f32_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_trunc_f32_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_trunc_f64_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_trunc_f64_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_extend_i32_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_extend_i32_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_trunc_f32_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_trunc_f32_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_trunc_f64_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_trunc_f64_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_convert_i32_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_convert_i32_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_convert_i64_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_convert_i64_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_demote_f64(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_convert_i32_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_convert_i32_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_convert_i64_s(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_convert_i64_u(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_promote_f32(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_reinterpret_f32(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i64_reinterpret_f64(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f32_reinterpret_i32(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_f64_reinterpret_i64(&mut self) -> Self::Output {
        todo!()
    }
}
