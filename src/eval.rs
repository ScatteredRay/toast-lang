use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

pub fn eval_tree() -> i32 {
    let mut builder_flags = settings::builder();
    builder_flags.set("use_colocated_libcalls", "false").unwrap();
    builder_flags.set("is_pic", "false").unwrap();
    let builder = JITBuilder::with_isa(
        cranelift_native::builder()
            .expect("Current host is not supported")
            .finish(settings::Flags::new(
                builder_flags
                )).unwrap(),
        cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder);

    let mut fun_sig = module.make_signature();
    fun_sig.returns.push(AbiParam::new(types::I32));
    let fun = module.declare_function("fn", cranelift_module::Linkage::Local, &fun_sig).unwrap();

    let mut ctx = module.make_context();
    let mut fun_ctx = FunctionBuilderContext::new();
    ctx.func.signature = fun_sig;
    //ctx.func.name

    let mut fbuilder = FunctionBuilder::new(&mut ctx.func, &mut fun_ctx);
    let block = fbuilder.create_block();
    fbuilder.switch_to_block(block);
    fbuilder.append_block_params_for_function_params(block);
    let a = fbuilder.ins().iconst(types::I32, 2);
    let b = fbuilder.ins().iconst(types::I32, 3);
    let r = fbuilder.ins().iadd(a, b);
    fbuilder.ins().return_(&[r]);
    fbuilder.seal_all_blocks();
    fbuilder.finalize();

    module.define_function(fun, &mut ctx).unwrap();
    module.clear_context(&mut ctx);

    module.finalize_definitions().unwrap();
    let fun_raw = module.get_finalized_function(fun);

    let fun_ptr = unsafe { std::mem::transmute::<_, extern "C" fn() -> i32>(fun_raw) };
    return fun_ptr();
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_math() {
        assert_eq!(crate::eval::eval_tree(), 5);
    }
}