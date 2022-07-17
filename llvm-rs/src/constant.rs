use super::*;

pub struct Constant {
    ptr: c::llvm_ConstantRef,
}

impl_from_as_ptr!(Constant, c::llvm_ConstantRef);

impl Constant {
    pub fn new_integer_value(ty: &IntegerType, val: u64) -> Self {
        unsafe { Self::from_ptr(c::llvm_Constant_getIntegerValue(ty.as_ptr(), val)) }
    }

    pub fn new_null_value(ty: &Type) -> Self {
        unsafe { Self::from_ptr(c::llvm_Constant_getNullValue(ty.as_ptr())) }
    }
}
