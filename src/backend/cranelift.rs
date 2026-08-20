use cranelift::prelude::*;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::middleend::ir::{IrInstructions, IrProgram};

// The principal function to do the compile to work.
pub fn compile(ir: &IrProgram) -> Result<(), String> {
    // Detects the computer's arquitecture current.
    let isa_builder = cranelift_native::builder().map_err(|error| error.to_string())?;

    // Standard Cranelift configurations.
    let flags = settings::Flags::new(settings::builder());

    // Create the final ISA.
    //
    // Example to my pc:
    // x86_64
    let isa = isa_builder
        .finish(flags)
        .map_err(|error| error.to_string())?;

    // Create the object file builder.
    let object_builder = ObjectBuilder::new(isa, "rune", default_libcall_names())
        .map_err(|error| error.to_string())?;

    // Create the module that will generate the '.o'.
    let mut module = ObjectModule::new(object_builder);

    // Architecture current pointer type.
    //
    // In x86_64, usually is I64.
    let pointer_type = module.target_config().pointer_type();

    // Create a signature.
    //
    // print_signature.
    let mut print_signature = module.make_signature();

    print_signature.params.push(AbiParam::new(pointer_type));
    print_signature.params.push(AbiParam::new(pointer_type));

    // Declares rune_print, a exported function.
    let print_id = module
        .declare_function("rune_print", Linkage::Import, &print_signature)
        .map_err(|error| error.to_string())?;

    // Create a signature.
    //
    // println_signature.
    let mut println_signature = module.make_signature();

    println_signature.params.push(AbiParam::new(pointer_type));
    println_signature.params.push(AbiParam::new(pointer_type));

    // Declares rune_print, a exported function.
    let println_id = module
        .declare_function("rune_println", Linkage::Import, &println_signature)
        .map_err(|error| error.to_string())?;

    // Create a signature.
    //
    // rune_main() -> i32
    let mut signature = module.make_signature();

    signature.returns.push(AbiParam::new(types::I32));

    // Declares rune_main(), a exported function.
    let function_id = module
        .declare_function("rune_main", Linkage::Export, &signature)
        .map_err(|error| error.to_string())?;

    // Create the context where the function will be created.
    let mut context = module.make_context();

    context.func.signature = signature;

    // FunctionBuilder's auxiliar context.
    let mut builder_context = FunctionBuilderContext::new();

    // Function builder.
    let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

    // Create the initial function block.
    let entry_block = builder.create_block();

    // Start writing instruction in this block.
    builder.switch_to_block(entry_block);

    // Says that here will be no more predecessors coming
    // in this block.
    builder.seal_block(entry_block);

    // Call rune_print.
    let print_ref = module.declare_func_in_func(print_id, builder.func);

    // Call rune_print.
    let println_ref = module.declare_func_in_func(println_id, builder.func);

    // String counter.
    let mut string_id = 0usize;

    // Go through the IR.
    for instructions in &ir.instructions {
        // Check the IR instruction.
        match instructions {
            // If the instruction is 'PrintString("value")'.
            IrInstructions::PrintString(value) => {
                // Transforms the Rust's String in bytes.
                //
                // "Hello".
                //
                // Will be:
                //
                // [72, 101, 108, 108, 111].
                //
                // '.as_byte()' returns a slice &[u8].
                // 'to_vec()' create an own Vec<u8>.
                let bytes = value.as_bytes().to_vec();

                // Take the how bytes the string have.
                // The 'as i64' converts usize to i64.
                let length = bytes.len() as i64;

                // Create a Cranelift data description.
                //
                // Will be used to say:
                //
                // "I want put these bytes in object file.".
                let mut data = DataDescription::new();

                // Defines which bytes belong to this data.
                data.define(bytes.into_boxed_slice());

                // Create a unic name to this string.
                //
                // first:
                //
                // rune_string_0
                //
                // second:
                //
                // rune_string_1
                let data_name = format!("rune_string_{}", string_id);

                // Increases the counter to the next string.
                //
                // 0 -> 1
                // 1 -> 2
                // 2 -> 3
                string_id += 1;

                // Declare this data block in ObjectModule.
                //
                // It how to say to Cranelift:
                //
                // "Will exists a data with name rune_string_0".
                let data_id = module
                    .declare_data(
                        // Data name.
                        &data_name,
                        // It means this data belongs to this object and do not need be exported.
                        Linkage::Export,
                        // writable = false
                        //
                        // The string will not be modificable.
                        false,
                        //tls = false
                        //
                        // It is not a Thread Local Storage.
                        false,
                    )
                    // If declare_data and give error.
                    // Converts th error to string.
                    .map_err(|error| error.to_string())?;

                // Now the data was declared.
                // We need DEFINES her content.
                //
                // declare_data:
                // "Exists rune_string_0".
                //
                // define_data:
                // "rune_string_0 has these bytes".
                module
                    .define_data(data_id, &data)
                    .map_err(|error| error.to_string())?;

                // Create a reference for this data.
                // In this current Cranelift function.
                //
                // The 'data_id' belongs to module.
                //
                // But to use this data in 'rune_main'.
                // we need converts to a function's GlobalValue.
                let global_value = module.declare_data_in_func(data_id, builder.func);

                // Generates a Cranelift instruction,
                // and have the string ADDRESS in memory.
                let address = builder.ins().global_value(
                    // Pointer type.
                    //
                    // In x86-64, is I64.
                    pointer_type,
                    // What data we want to enter.
                    global_value,
                );

                // Create a constant containing the string length.
                //
                // For:
                //
                // "Hello"
                //
                // Be:
                //
                // length_value = 5.
                let length_value = builder.ins().iconst(
                    // Same type used for the size.
                    // Expected for rune_print.
                    pointer_type,
                    // Bytes quantity.
                    length,
                );

                // Finally the call:
                //
                // rune_print(adress, length).
                //
                // Example:
                //
                // rune_print(0x401000, 5).
                builder.ins().call(
                    // Reference to rune_print.
                    print_ref,
                    // Function's arguments.
                    &[address, length_value],
                );
            }

            // If the instruction is 'PrintlnString("value")'.
            IrInstructions::PrintlnString(value) => {
                // Transforms the Rust's String in bytes.
                //
                // "Hello".
                //
                // Will be:
                //
                // [72, 101, 108, 108, 111].
                //
                // '.as_byte()' returns a slice &[u8].
                // 'to_vec()' create an own Vec<u8>.
                let bytes = value.as_bytes().to_vec();

                // Take the how bytes the string have.
                // The 'as i64' converts usize to i64.
                let length = bytes.len() as i64;

                // Create a Cranelift data description.
                //
                // Will be used to say:
                //
                // "I want put these bytes in object file.".
                let mut data = DataDescription::new();

                // Defines which bytes belong to this data.
                data.define(bytes.into_boxed_slice());

                // Create a unic name to this string.
                //
                // first:
                //
                // rune_string_0
                //
                // second:
                //
                // rune_string_1
                let data_name = format!("rune_string_{}", string_id);

                // Increases the counter to the next string.
                //
                // 0 -> 1
                // 1 -> 2
                // 2 -> 3
                string_id += 1;

                // Declare this data block in ObjectModule.
                //
                // It how to say to Cranelift:
                //
                // "Will exists a data with name rune_string_0".
                let data_id = module
                    .declare_data(
                        // Data name.
                        &data_name,
                        // It means this data belongs to this object and do not need be exported.
                        Linkage::Export,
                        // writable = false
                        //
                        // The string will not be modificable.
                        false,
                        //tls = false
                        //
                        // It is not a Thread Local Storage.
                        false,
                    )
                    // If declare_data and give error.
                    // Converts th error to string.
                    .map_err(|error| error.to_string())?;

                // Now the data was declared.
                // We need DEFINES her content.
                //
                // declare_data:
                // "Exists rune_string_0".
                //
                // define_data:
                // "rune_string_0 has these bytes".
                module
                    .define_data(data_id, &data)
                    .map_err(|error| error.to_string())?;

                // Create a reference for this data.
                // In this current Cranelift function.
                //
                // The 'data_id' belongs to module.
                //
                // But to use this data in 'rune_main'.
                // we need converts to a function's GlobalValue.
                let global_value = module.declare_data_in_func(data_id, builder.func);

                // Generates a Cranelift instruction,
                // and have the string ADDRESS in memory.
                let address = builder.ins().global_value(
                    // Pointer type.
                    //
                    // In x86-64, is I64.
                    pointer_type,
                    // What data we want to enter.
                    global_value,
                );

                // Create a constant containing the string length.
                //
                // For:
                //
                // "Hello"
                //
                // Be:
                //
                // length_value = 5.
                let length_value = builder.ins().iconst(
                    // Same type used for the size.
                    // Expected for rune_println.
                    pointer_type,
                    // Bytes quantity.
                    length,
                );

                // Finally the call:
                //
                // rune_println(adress, length).
                //
                // Example:
                //
                // rune_println(0x401000, 5).
                builder.ins().call(
                    // Reference to rune_println.
                    println_ref,
                    // Function's arguments.
                    &[address, length_value],
                );
            }
        }
    }

    // Create the whole constant 0.
    let zero = builder.ins().iconst(types::I32, 0);

    // Generate:
    //
    // return 0.
    builder.ins().return_(&[zero]);

    // Finished the function construction.
    builder.finalize();

    // Give the finished function to ObjectModule.
    module
        .define_function(function_id, &mut context)
        .map_err(|error| error.to_string())?;

    // Finished the context to can use again.
    module.clear_context(&mut context);

    // Finished the object's generation.
    let object = module.finish();

    // Converts the object in bytes.
    let bytes = object.emit().map_err(|error| error.to_string())?;

    // Writes the 'src/runtime/program.o' in disk.
    std::fs::write("src/runtime/program.o", bytes).map_err(|error| error.to_string())?;

    Ok(()) // Returns.
}

#[cfg(test)]
mod tests {
    use crate::middleend::ir::{IrInstructions, IrProgram};

    #[test]
    fn test_cranelift_receives_each_instruction_once() {
        let ir = IrProgram {
            instructions: vec![
                IrInstructions::PrintString("Hello, ".to_string()),
                IrInstructions::PrintString("world!".to_string()),
            ],
        };

        for instruction in &ir.instructions {
            println!("BACKEND: {:#?}", instruction);
        }
    }
}
