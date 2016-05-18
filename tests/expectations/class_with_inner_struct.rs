/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[derive(Copy, Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_A {
    pub c: ::std::os::raw::c_uint,
    pub named_union: Union_A_class_with_inner_struct_hpp_unnamed_1,
    pub A_class_with_inner_struct_hpp_unnamed_2: Union_A_class_with_inner_struct_hpp_unnamed_2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_A_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_A_Segment {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_A_Segment() {
    assert_eq!(::std::mem::size_of::<Struct_A_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_A_Segment>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_A_class_with_inner_struct_hpp_unnamed_1 {
    pub f: __BindgenUnionField<::std::os::raw::c_int>,
    pub _bindgen_data_: u32,
}
impl Union_A_class_with_inner_struct_hpp_unnamed_1 {
    pub unsafe fn f(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_A_class_with_inner_struct_hpp_unnamed_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_A_class_with_inner_struct_hpp_unnamed_1() {
    assert_eq!(::std::mem::size_of::<Union_A_class_with_inner_struct_hpp_unnamed_1>()
               , 4usize);
    assert_eq!(::std::mem::align_of::<Union_A_class_with_inner_struct_hpp_unnamed_1>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_A_class_with_inner_struct_hpp_unnamed_2 {
    pub d: __BindgenUnionField<::std::os::raw::c_int>,
    pub _bindgen_data_: u32,
}
impl Union_A_class_with_inner_struct_hpp_unnamed_2 {
    pub unsafe fn d(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_A_class_with_inner_struct_hpp_unnamed_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_A_class_with_inner_struct_hpp_unnamed_2() {
    assert_eq!(::std::mem::size_of::<Union_A_class_with_inner_struct_hpp_unnamed_2>()
               , 4usize);
    assert_eq!(::std::mem::align_of::<Union_A_class_with_inner_struct_hpp_unnamed_2>()
               , 4usize);
}
impl ::std::clone::Clone for Struct_A {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_A() {
    assert_eq!(::std::mem::size_of::<Struct_A>() , 12usize);
    assert_eq!(::std::mem::align_of::<Struct_A>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_B {
    pub d: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_B_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_B_Segment {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_B_Segment() {
    assert_eq!(::std::mem::size_of::<Struct_B_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_B_Segment>() , 4usize);
}
impl ::std::clone::Clone for Struct_B {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_B() {
    assert_eq!(::std::mem::size_of::<Struct_B>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_B>() , 4usize);
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Enum_StepSyntax {
    Keyword = 0,
    FunctionalWithoutKeyword = 1,
    FunctionalWithStartKeyword = 2,
    FunctionalWithEndKeyword = 3,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_C {
    pub d: ::std::os::raw::c_uint,
    pub C_class_with_inner_struct_hpp_unnamed_3: Union_C_class_with_inner_struct_hpp_unnamed_3,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_C_class_with_inner_struct_hpp_unnamed_3 {
    pub mFunc: __BindgenUnionField<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4>,
    pub C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5: __BindgenUnionField<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5>,
    pub _bindgen_data_: [u32; 4usize],
}
impl Union_C_class_with_inner_struct_hpp_unnamed_3 {
    pub unsafe fn mFunc(&mut self)
     ->
         *mut Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5(&mut self)
     ->
         *mut Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_C_class_with_inner_struct_hpp_unnamed_3 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_C_class_with_inner_struct_hpp_unnamed_3() {
    assert_eq!(::std::mem::size_of::<Union_C_class_with_inner_struct_hpp_unnamed_3>()
               , 16usize);
    assert_eq!(::std::mem::align_of::<Union_C_class_with_inner_struct_hpp_unnamed_3>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4 {
    pub mX1: f32,
    pub mY1: f32,
    pub mX2: f32,
    pub mY2: f32,
}
impl ::std::clone::Clone for
 Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4
 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4() {
    assert_eq!(::std::mem::size_of::<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4>()
               , 16usize);
    assert_eq!(::std::mem::align_of::<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_4>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5 {
    pub mStepSyntax: Enum_StepSyntax,
    pub mSteps: ::std::os::raw::c_uint,
}
impl ::std::clone::Clone for
 Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5
 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5() {
    assert_eq!(::std::mem::size_of::<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_C_class_with_inner_struct_hpp_unnamed_3_class_with_inner_struct_hpp_unnamed_5>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_C_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_C_Segment {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_C_Segment() {
    assert_eq!(::std::mem::size_of::<Struct_C_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_C_Segment>() , 4usize);
}
impl ::std::clone::Clone for Struct_C {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_C() {
    assert_eq!(::std::mem::size_of::<Struct_C>() , 20usize);
    assert_eq!(::std::mem::align_of::<Struct_C>() , 4usize);
}
