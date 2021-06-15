struct ModuleInfo {
    custom_sections: IndexMap<String, CustomSectionIndex>,
    custom_sections_data: PrimaryMap<CustomSectionIndex, Arc<[u8]>>,
    exports: IndexMap<String, ExportIndex>,
    function_names: HashMap<FunctionIndex, String>,
    functions: PrimaryMap<FunctionIndex, SignatureIndex>,
    global_initializers: PrimaryMap<LocalGlobalIndex, GlobalInit>,
    globals: PrimaryMap<GlobalIndex, GlobalType>,
    id: ModuleId,
    imports: IndexMap<(String, String, u32), ImportIndex>,
    memories: PrimaryMap<MemoryIndex, MemoryType>,
    name: Option<String>,
    num_imported_functions: usize,
    num_imported_globals: usize,
    num_imported_memories: usize,
    num_imported_tables: usize,
    passive_data: HashMap<DataIndex, Arc<[u8]>>,
    passive_elements: HashMap<ElemIndex, Box<[FunctionIndex]>>,
    signatures: PrimaryMap<SignatureIndex, FunctionType>,
    start_function: Option<FunctionIndex>,
    table_initializers: Vec<TableInitializer>,
    tables: PrimaryMap<TableIndex, TableType>,
}
