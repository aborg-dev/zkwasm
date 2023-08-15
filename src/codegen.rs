use std::collections::HashMap;

use wasmparser::{BlockType, BrTable, Ieee32, Ieee64, MemArg, Payload::*, ValType};

use anyhow::Result;

pub struct ZkAssembler {
    instructions: Vec<String>,
}

#[derive(PartialEq, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    E,
}

impl Register {
    fn name(self) -> &'static str {
        match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::E => "E",
        }
    }
}

impl ZkAssembler {
    fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }

    fn add_instruction(&mut self, instruction: &str) {
        self.instructions
            .push(format!("\t{instruction}").to_string());
    }

    fn label(&mut self, value: &str) {
        self.instructions.push(format!("{value}: ").to_string());
    }

    fn jump(&mut self, dst: &str) {
        self.add_instruction(&format!(":JMP({dst})"));
    }

    fn jump_if_zero(&mut self, register: Register, dst: &str) {
        self.add_instruction(&format!("{} :JMPZ({dst})", register.name()));
    }

    fn jump_if_nonzero(&mut self, register: Register, dst: &str) {
        self.add_instruction(&format!("{} :JMPNZ({dst})", register.name()));
    }

    fn stack_push_const(&mut self, value: i32) {
        self.add_instruction(&format!("{value} :MSTORE(SP++)"));
    }

    fn stack_push_register(&mut self, register: Register) {
        self.add_instruction(&format!("{} :MSTORE(SP++)", register.name()));
    }

    fn stack_pop(&mut self, register: Register) {
        self.add_instruction("SP - 1 => SP");
        self.add_instruction(&format!("$ => {}: MLOAD(SP)", register.name()));
    }

    fn get_stack_address(offset: i32) -> String {
        match offset.cmp(&0) {
            std::cmp::Ordering::Less => format!("SP - {}", -offset),
            std::cmp::Ordering::Equal => "SP".to_string(),
            std::cmp::Ordering::Greater => format!("SP + {}", offset),
        }
    }

    fn stack_set(&mut self, register: Register, offset: i32) {
        self.add_instruction(&format!(
            "{} :MSTORE({})",
            register.name(),
            ZkAssembler::get_stack_address(offset)
        ));
    }

    fn stack_get(&mut self, register: Register, offset: i32) {
        self.add_instruction(&format!(
            "$ => {} :MLOAD({})",
            register.name(),
            ZkAssembler::get_stack_address(offset)
        ));
    }

    fn add(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :ADD", register.name()));
    }

    fn sub(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :SUB", register.name()));
    }

    fn and(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :AND", register.name()));
    }

    fn or(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :OR", register.name()));
    }

    fn xor(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :XOR", register.name()));
    }

    fn eq(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :EQ", register.name()));
    }

    fn unsigned_less_then(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :LT", register.name()));
    }

    fn signed_less_then(&mut self, register: Register) {
        self.add_instruction(&format!("$ => {} :SLT", register.name()));
    }

    fn assert(&mut self, register: Register) {
        self.add_instruction(&format!("{} :ASSERT", register.name()));
    }

    fn assert_const(&mut self, value: i32) {
        self.add_instruction(&format!("{value} :ASSERT"));
    }

    fn finalize(self) -> String {
        self.instructions.join("\n")
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Location {
    // The local is in the stack with a given offset.
    Stack(i32),
    // The local is in a given register.
    Register(Register),
    // The local is not initialized.
    Uninitialized,
}

struct Local {
    location: Location,
    ty: ValType,
}

struct ZkCodegenVisitor {
    assembler: ZkAssembler,
    locals: Vec<Local>,
    stack_depth: i32,
}

impl ZkCodegenVisitor {
    fn new(mut assembler: ZkAssembler, local_counts: Vec<(u32, ValType)>) -> Self {
        assembler.label("start");
        let mut locals = Vec::new();
        for (count, ty) in local_counts {
            for _ in 0..count {
                locals.push(Local {
                    location: Location::Uninitialized,
                    ty,
                });
            }
        }
        Self {
            assembler,
            locals,
            stack_depth: 0,
        }
    }

    fn stack_pop(&mut self, dst: Register) {
        self.assembler.stack_pop(dst);
        self.stack_depth -= 1;
    }

    fn stack_push_register(&mut self, src: Register) {
        self.assembler.stack_push_register(src);
        self.stack_depth += 1;
    }

    fn stack_push_const(&mut self, value: i32) {
        self.assembler.stack_push_const(value);
        self.stack_depth += 1;
    }

    fn finalize(self) -> String {
        self.assembler.finalize()
    }
}

pub fn parse(module: &[u8]) -> Result<String> {
    let parser = wasmparser::Parser::new(0);
    let mut program = String::new();

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
                let mut locals = Vec::new();
                for local in body.get_locals_reader()? {
                    locals.push(local?);
                }
                let assembler = ZkAssembler::new();
                let mut visitor = ZkCodegenVisitor::new(assembler, locals);
                let mut operator_reader = body.get_operators_reader()?;
                while !operator_reader.eof() {
                    operator_reader.visit_operator(&mut visitor)?;
                }
                program += &visitor.finalize();
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

    program += "
finalizeExecution:
	${beforeLast()}  :JMPN(finalizeExecution)
                     :JMP(start)";

    Ok(program)
}

macro_rules! define_visit_once {
    (@mvp $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {};
    (@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {
        fn $visit(&mut self $($(,$arg: $argty)*)?) {
            panic!("Operator not implemented");
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
        // TODO: Do I need to do anything here?
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
        // TODO: Allow to call arbitrary functions.
        self.stack_pop(Register::A);
        self.assembler.assert_const(1);
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

    fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
        let local = self
            .locals
            .get_mut(local_index as usize)
            .expect(&format!("Can't find local {}", local_index));

        match local.location {
            Location::Stack(offset) => {
                self.assembler.stack_get(Register::E, offset - self.stack_depth);
                self.stack_push_register(Register::E);
            }
            Location::Register(register) => {
                self.stack_push_register(register);
            }
            Location::Uninitialized => {
                panic!("Local {local_index} is not initialized");
            }
        }
    }

    fn visit_local_set(&mut self, local_index: u32) -> Self::Output {
        let location = self
            .locals
            .get(local_index as usize)
            .expect(&format!("Can't find local {}", local_index))
            .location;

        match location {
            Location::Stack(offset) => {
                self.stack_pop(Register::E);
                self.assembler.stack_set(Register::E, offset - self.stack_depth);
            }
            Location::Register(register) => {
                self.stack_pop(register);
            }
            Location::Uninitialized => {
                self.stack_pop(Register::E);
                self.locals[local_index as usize].location = Location::Stack(self.stack_depth);
                self.stack_push_register(Register::E);
            }
        }
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

    fn visit_i32_const(&mut self, value: i32) -> Self::Output {
        self.stack_push_const(value);
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
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.eq(Register::A);
        self.stack_push_register(Register::A);
    }

    fn visit_i32_ne(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_i32_lt_s(&mut self) -> Self::Output {
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.signed_less_then(Register::A);
        self.stack_push_register(Register::A);
    }

    fn visit_i32_lt_u(&mut self) -> Self::Output {
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.unsigned_less_then(Register::A);
        self.stack_push_register(Register::A);
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
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.add(Register::A);
        self.stack_push_register(Register::A);
    }

    fn visit_i32_sub(&mut self) -> Self::Output {
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.sub(Register::A);
        self.stack_push_register(Register::A);
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
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.and(Register::A);
        self.stack_push_register(Register::A);
    }

    fn visit_i32_or(&mut self) -> Self::Output {
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.or(Register::A);
        self.stack_push_register(Register::A);
    }

    fn visit_i32_xor(&mut self) -> Self::Output {
        self.stack_pop(Register::A);
        self.stack_pop(Register::B);
        self.assembler.xor(Register::A);
        self.stack_push_register(Register::A);
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
