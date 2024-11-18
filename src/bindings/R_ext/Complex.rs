/* automatically generated by rust-bindgen 0.70.1 */

/* rustc 1.81.0 (eeb90cda1 2024-09-04) */
/* r version: 4.4.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex__bindgen_ty_1 {
    pub r: f64,
    pub i: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rcomplex__bindgen_ty_1"][::std::mem::size_of::<Rcomplex__bindgen_ty_1>() - 16usize];
    ["Alignment of Rcomplex__bindgen_ty_1"]
        [::std::mem::align_of::<Rcomplex__bindgen_ty_1>() - 8usize];
    ["Offset of field: Rcomplex__bindgen_ty_1::r"]
        [::std::mem::offset_of!(Rcomplex__bindgen_ty_1, r) - 0usize];
    ["Offset of field: Rcomplex__bindgen_ty_1::i"]
        [::std::mem::offset_of!(Rcomplex__bindgen_ty_1, i) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rcomplex"][::std::mem::size_of::<Rcomplex>() - 16usize];
    ["Alignment of Rcomplex"][::std::mem::align_of::<Rcomplex>() - 8usize];
    ["Offset of field: Rcomplex::private_data_c"]
        [::std::mem::offset_of!(Rcomplex, private_data_c) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union Rcomplex {
    pub __bindgen_anon_1: Rcomplex__bindgen_ty_1,
    pub private_data_c: __BindgenComplex<f64>,
}
