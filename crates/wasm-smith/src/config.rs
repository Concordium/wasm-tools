//! Configuring the shape of generated Wasm modules.

use arbitrary::Arbitrary;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HostFunction {
    pub mod_name: &'static str,
    pub name:     &'static str,
    pub hf_index: usize, // index in the host-function vector
    pub params:   Vec<ValType>,
    pub result:   Option<ValType>,
}

type FuncType = (Vec<ValType>, Option<ValType>);

/// Configuration for a generated module.
///
/// Don't care to configure your generated modules? Just use
/// [`Module`][crate::Module], which internally uses
/// [`DefaultConfig`][crate::DefaultConfig].
///
/// If you want to configure generated modules, then define a `MyConfig` type,
/// implement this trait for it, and use
/// [`ConfiguredModule<MyConfig>`][crate::ConfiguredModule] instead of `Module`.
///
/// Every trait method has a provided default implementation, so that you only
/// need to override the methods for things you want to change away from the
/// default.
pub trait Config: Arbitrary + Default + Clone {
    /// The minimum number of types to generate, excluding types of imports and
    /// auxiliary export types. Defaults to 0.
    fn min_types(&self) -> usize { 0 }

    /// The maximum number of types to generate. Defaults to 100.
    fn max_types(&self) -> usize { 100 }

    /// The return types of functions that can be exported. If None, any return
    /// types are allowed.
    fn allowed_export_types(&self) -> Option<Vec<FuncType>> { None }

    /// The minimum number of imports to generate. Defaults to 0.
    ///
    /// Note that if the sum of the maximum function[^1], table, global and
    /// memory counts is less than the minimum number of imports, then it will
    /// not be possible to satisfy all constraints (because imports count
    /// against the limits for those element kinds). In that case, we strictly
    /// follow the max-constraints, and can fail to satisfy this minimum number.
    ///
    /// [^1]: the maximum number of functions is also limited by the number of
    ///       function types arbitrarily chosen; strictly speaking, then, the
    ///       maximum number of imports that can be created due to
    ///       max-constraints is `sum(min(num_func_types, max_funcs),
    /// max_tables,       max_globals, max_memories)`.
    fn min_imports(&self) -> usize { 0 }

    /// The maximum number of imports to generate. Defaults to 100.
    fn max_imports(&self) -> usize { 20 }

    /// The minimum number of functions to generate. Defaults to 0.  This
    /// includes imported functions.
    fn min_funcs(&self) -> usize { 0 }

    /// The maximum number of functions to generate. Defaults to 100.  This
    /// includes imported functions.
    fn max_funcs(&self) -> usize { 100 }

    /// The minimum number of globals to generate. Defaults to 0.  This includes
    /// imported globals.
    fn min_globals(&self) -> usize { 0 }

    /// The maximum number of globals to generate. Defaults to 100.  This
    /// includes imported globals.
    fn max_globals(&self) -> usize { 100 }

    /// The minimum number of exports to generate. Defaults to 0.
    fn min_exports(&self) -> usize { 0 }

    /// The maximum number of exports to generate. Defaults to 100.
    fn max_exports(&self) -> usize { 100 }

    /// The minimum number of element segments to generate. Defaults to 0.
    fn min_element_segments(&self) -> usize { 0 }

    /// The maximum number of element segments to generate. Defaults to 100.
    fn max_element_segments(&self) -> usize { 100 }

    /// The minimum number of elements within a segment to generate. Defaults to
    /// 0.
    fn min_elements(&self) -> usize { 0 }

    /// The maximum number of elements within a segment to generate. Defaults to
    /// 100.
    fn max_elements(&self) -> usize { 100 }

    /// The minimum number of dataelements within a data segment to generate.
    /// Defaults to 0.
    fn min_data_elements(&self) -> usize { 0 }

    /// The maximum number of elements within a segment to generate. Defaults to
    /// 100.
    fn max_data_elements(&self) -> usize { 100 }

    /// The minimum number of data segments to generate. Defaults to 0.
    fn min_data_segments(&self) -> usize { 0 }

    /// The maximum number of data segments to generate. Defaults to 100.
    fn max_data_segments(&self) -> usize { 100 }

    /// The maximum number of instructions to generate in a function
    /// body. Defaults to 100.
    ///
    /// Note that some additional `end`s, `else`s, and `unreachable`s may be
    /// appended to the function body to finish block scopes.
    fn max_instructions(&self) -> usize { 100 }

    /// The minimum number of memories to use. Defaults to 0. This includes
    /// imported memories.
    fn min_memories(&self) -> u32 { 0 }

    /// The maximum number of memories to use. Defaults to 1. This includes
    /// imported memories.
    ///
    /// Note that more than one memory is in the realm of the multi-memory wasm
    /// proposal.
    fn max_memories(&self) -> usize { 1 }

    /// The minimum number of tables to use. Defaults to 0. This includes
    /// imported tables.
    fn min_tables(&self) -> u32 { 0 }

    /// The upper bound on the minimum limit when a table is initialized
    fn max_init_table_size(&self) -> u32 { 1_000_000 }

    /// The maximum number of tables to use. Defaults to 1. This includes
    /// imported tables.
    ///
    /// Note that more than one table is in the realm of the reference types
    /// proposal.
    fn max_tables(&self) -> usize { 1 }

    /// The maximum, in 64k Wasm pages, of any memory's initial or maximum size.
    /// Defaults to 2^16 = 65536 (the maximum possible for 32-bit Wasm).
    fn max_memory_pages(&self) -> u32 { 65536 }

    /// Whether every Wasm memory must have a maximum size specified. Defaults
    /// to `false`.
    fn memory_max_size_required(&self) -> bool { false }

    /// Control the probability of generating memory offsets that are in bounds
    /// vs. potentially out of bounds.
    ///
    /// Return a tuple `(a, b, c)` where
    ///
    /// * `a / (a+b+c)` is the probability of generating a memory offset within
    ///   `0..memory.min_size`, i.e. an offset that is definitely in bounds of a
    ///   non-empty memory. (Note that if a memory is zero-sized, however, no
    ///   offset will ever be in bounds.)
    ///
    /// * `b / (a+b+c)` is the probability of generating a memory offset within
    ///   `memory.min_size..memory.max_size`, i.e. an offset that is possibly in
    ///   bounds if the memory has been grown.
    ///
    /// * `c / (a+b+c)` is the probability of generating a memory offset within
    ///   the range `memory.max_size..`, i.e. an offset that is definitely out
    ///   of bounds.
    ///
    /// At least one of `a`, `b`, and `c` must be non-zero.
    ///
    /// If you want to always generate memory offsets that are definitely in
    /// bounds of a non-zero-sized memory, for example, you could return `(1, 0,
    /// 0)`.
    ///
    /// By default, returns `(75, 24, 1)`.
    fn memory_offset_choices(&self) -> (u32, u32, u32) { (75, 24, 1) }

    /// The minimum size, in bytes, of all leb-encoded integers. Defaults to 1.
    ///
    /// This is useful for ensuring that all leb-encoded integers are decoded as
    /// such rather than as simply one byte. This will forcibly extend leb
    /// integers with an over-long encoding in some locations if the size would
    /// otherwise be smaller than number returned here.
    fn min_uleb_size(&self) -> u8 { 1 }

    /// Determines whether a `start` export may be included. Defaults to `true`.
    fn allow_start_export(&self) -> bool { true }

    /// The set of admissible host functions that can be imported into the
    /// module
    fn host_functions(&self) -> Vec<HostFunction> { Vec::new() }

    /// Allow arbitrary instructions?
    fn allow_arbitrary_instr(&self) -> bool { false }

    /// Allow global reads in offsets of elem and data sections?
    fn allow_globalget_in_elem_and_data_offsets(&self) -> bool { true }

    /// Allow function block type?
    fn allow_function_blocktype(&self) -> bool { false }

    /// Maximum number of function parameters to generate
    fn max_parameters(&self) -> usize { 20 }
}

/// The default configuration.
#[derive(Arbitrary, Debug, Default, Copy, Clone)]
pub struct DefaultConfig;

impl Config for DefaultConfig {}

/// A module configuration for a Concordium smart-contract module
#[derive(Default, Debug, Arbitrary, Clone)]
pub struct InterpreterConfig;

impl Config for InterpreterConfig {
    fn host_functions(&self) -> Vec<HostFunction> {
        let hosts = [
            ("accept", Vec::new(), Some(I32)),
            ("simple_transfer", vec![I32, I64], Some(I32)),
            ("send", vec![I64, I64, I32, I32, I64, I32, I32], Some(I32)),
            ("combine_and", vec![I32, I32], Some(I32)),
            ("combine_or", vec![I32, I32], Some(I32)),
            ("get_parameter_size", Vec::new(), Some(I32)),
            ("get_parameter_section", vec![I32, I32, I32], Some(I32)),
            ("get_policy_section", vec![I32, I32, I32], Some(I32)),
            ("log_event", vec![I32, I32], None),
            ("load_state", vec![I32, I32, I32], Some(I32)),
            ("write_state", vec![I32, I32, I32], Some(I32)),
            ("resize_state", vec![I32], Some(I32)),
            ("state_size", Vec::new(), Some(I32)),
            ("get_init_origin", vec![I32], None),
            ("get_receive_invoker", vec![I32], None),
            ("get_receive_self_address", vec![I32], None),
            ("get_receive_self_balance", Vec::new(), Some(I64)),
            ("get_receive_sender", vec![I32], None),
            ("get_receive_owner", vec![I32], None),
            ("get_slot_time", Vec::new(), Some(I64)),
        ];
        let mut host_funs = Vec::new();
        for (idx, (name, params, ret)) in hosts.iter().enumerate() {
            host_funs.push(HostFunction {
                mod_name: "concordium",
                name,
                hf_index: idx,
                params: params.to_vec(),
                result: *ret,
            });
        }
        host_funs
    }

    fn min_imports(&self) -> usize { 10 }

    fn max_imports(&self) -> usize { self.host_functions().len() }

    fn min_exports(&self) -> usize { 1 }

    fn max_exports(&self) -> usize { 10 }

    fn min_types(&self) -> usize { 2 }

    fn min_funcs(&self) -> usize { 2 }

    fn max_funcs(&self) -> usize { 50 }

    fn allow_start_export(&self) -> bool { false }

    fn allow_arbitrary_instr(&self) -> bool { false }

    fn max_globals(&self) -> usize { 100 }

    fn max_init_table_size(&self) -> u32 { 1_000 }

    fn allowed_export_types(&self) -> Option<Vec<FuncType>> {
        Some(vec![(vec![ValType::I64], Some(ValType::I32))])
    }

    fn allow_globalget_in_elem_and_data_offsets(&self) -> bool { false }

    // TODO (MRA) When CB-1165 is done, set this to true

    fn max_memory_pages(&self) -> u32 { 32 }
}
