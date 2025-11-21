// Fusabi VM Micro-benchmarks
// Measures performance of individual VM operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fusabi_vm::{Chunk, Instruction, Value, Vm};

fn benchmark_arithmetic(c: &mut Criterion) {
    c.bench_function("vm_add_100_ints", |b| {
        let mut chunk = Chunk::new();

        // Generate bytecode: sum from 1 to 100
        chunk.add_constant(Value::Int(0)); // accumulator at const[0]
        chunk.emit(Instruction::LoadConst(0));

        for i in 1..=100 {
            chunk.add_constant(Value::Int(i));
            let idx = chunk.constants.len() - 1;
            chunk.emit(Instruction::LoadConst(idx as u16));
            chunk.emit(Instruction::Add);
        }

        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });

    c.bench_function("vm_multiply_50_ints", |b| {
        let mut chunk = Chunk::new();

        chunk.add_constant(Value::Int(1)); // start with 1
        chunk.emit(Instruction::LoadConst(0));

        for i in 1..=50 {
            chunk.add_constant(Value::Int(i % 10 + 1)); // Keep numbers small
            let idx = chunk.constants.len() - 1;
            chunk.emit(Instruction::LoadConst(idx as u16));
            chunk.emit(Instruction::Multiply);
        }

        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_stack_ops(c: &mut Criterion) {
    c.bench_function("vm_push_pop_1000", |b| {
        let mut chunk = Chunk::new();

        // Push 1000 values
        for i in 0..1000 {
            chunk.add_constant(Value::Int(i));
            let idx = chunk.constants.len() - 1;
            chunk.emit(Instruction::LoadConst(idx as u16));
        }

        // Pop all but one
        for _ in 0..999 {
            chunk.emit(Instruction::Pop);
        }

        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });

    c.bench_function("vm_dup_100", |b| {
        let mut chunk = Chunk::new();

        chunk.add_constant(Value::Int(42));
        chunk.emit(Instruction::LoadConst(0));

        // Duplicate 100 times
        for _ in 0..100 {
            chunk.emit(Instruction::Dup);
        }

        // Clean up stack
        for _ in 0..100 {
            chunk.emit(Instruction::Pop);
        }

        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_control_flow(c: &mut Criterion) {
    c.bench_function("vm_branch_100", |b| {
        let mut chunk = Chunk::new();

        // Simple branching test
        for _ in 0..100 {
            chunk.add_constant(Value::Bool(true));
            chunk.emit(Instruction::LoadConst(chunk.constants.len() as u16 - 1));
            chunk.emit(Instruction::JumpIfFalse(3)); // Skip next 2 instructions
            chunk.add_constant(Value::Int(1));
            chunk.emit(Instruction::LoadConst(chunk.constants.len() as u16 - 1));
            chunk.emit(Instruction::Pop);
        }

        chunk.add_constant(Value::Int(42));
        chunk.emit(Instruction::LoadConst(chunk.constants.len() as u16 - 1));
        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });

    c.bench_function("vm_loop_100", |b| {
        let mut chunk = Chunk::new();

        // Initialize counter
        chunk.add_constant(Value::Int(0));
        chunk.emit(Instruction::LoadConst(0));
        chunk.emit(Instruction::SetLocal(0));

        // Loop 100 times
        let loop_start = chunk.instructions.len();
        chunk.emit(Instruction::GetLocal(0));
        chunk.add_constant(Value::Int(1));
        chunk.emit(Instruction::LoadConst(1));
        chunk.emit(Instruction::Add);
        chunk.emit(Instruction::SetLocal(0));

        chunk.emit(Instruction::GetLocal(0));
        chunk.add_constant(Value::Int(100));
        chunk.emit(Instruction::LoadConst(2));
        chunk.emit(Instruction::LessThan);

        let jump_offset = -(chunk.instructions.len() as i16 - loop_start as i16 + 1);
        chunk.emit(Instruction::JumpIfTrue(jump_offset));

        chunk.emit(Instruction::GetLocal(0));
        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            vm.push_frame(1); // Allocate space for local
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_data_structures(c: &mut Criterion) {
    c.bench_function("vm_create_list_100", |b| {
        let mut chunk = Chunk::new();

        // Create a list [1, 2, ..., 100]
        chunk.emit(Instruction::Nil);

        for i in (1..=100).rev() {
            chunk.add_constant(Value::Int(i));
            chunk.emit(Instruction::LoadConst(chunk.constants.len() as u16 - 1));
            chunk.emit(Instruction::Cons);
        }

        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });

    c.bench_function("vm_create_tuple_10", |b| {
        let mut chunk = Chunk::new();

        // Create a 10-element tuple
        for i in 0..10 {
            chunk.add_constant(Value::Int(i));
            chunk.emit(Instruction::LoadConst(chunk.constants.len() as u16 - 1));
        }

        chunk.emit(Instruction::MakeTuple(10));
        chunk.emit(Instruction::Return);

        b.iter(|| {
            let mut vm = Vm::new();
            let result = vm.execute(chunk.clone()).unwrap();
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    benchmark_arithmetic,
    benchmark_stack_ops,
    benchmark_control_flow,
    benchmark_data_structures
);
criterion_main!(benches);