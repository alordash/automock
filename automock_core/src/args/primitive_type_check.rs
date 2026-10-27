use std::any::TypeId;
use std::collections::HashSet;
use std::sync::LazyLock;

pub fn is_primitive_type<T>() -> bool {
    let tid = typeid::of::<T>();
    return PRIMITIVE_TYPES.contains(&tid);
}

static PRIMITIVE_TYPES: LazyLock<HashSet<TypeId>> = LazyLock::new(|| {
    HashSet::from([
        TypeId::of::<u8>(),
        TypeId::of::<u16>(),
        TypeId::of::<u32>(),
        TypeId::of::<u64>(),
        TypeId::of::<u128>(),
        TypeId::of::<usize>(),
        TypeId::of::<i8>(),
        TypeId::of::<i16>(),
        TypeId::of::<i32>(),
        TypeId::of::<i64>(),
        TypeId::of::<i128>(),
        TypeId::of::<isize>(),
        TypeId::of::<f32>(),
        TypeId::of::<f64>(),
        TypeId::of::<bool>(),
        TypeId::of::<char>(),
        TypeId::of::<&str>(),
        TypeId::of::<String>(),
    ])
});
