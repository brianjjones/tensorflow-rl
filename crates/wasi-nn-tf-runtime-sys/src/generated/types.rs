pub const TF_TSTRING_LITTLE_ENDIAN: u32 = 1;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TF_AttrType {
    TF_ATTR_STRING = 0,
    TF_ATTR_INT = 1,
    TF_ATTR_FLOAT = 2,
    TF_ATTR_BOOL = 3,
    TF_ATTR_TYPE = 4,
    TF_ATTR_SHAPE = 5,
    TF_ATTR_TENSOR = 6,
    TF_ATTR_PLACEHOLDER = 7,
    TF_ATTR_FUNC = 8,
}
impl TF_DataType {
    pub const TF_COMPLEX: TF_DataType = TF_DataType::TF_COMPLEX64;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TF_DataType {
    TF_FLOAT = 1,
    TF_DOUBLE = 2,
    TF_INT32 = 3,
    TF_UINT8 = 4,
    TF_INT16 = 5,
    TF_INT8 = 6,
    TF_STRING = 7,
    TF_COMPLEX64 = 8,
    TF_INT64 = 9,
    TF_BOOL = 10,
    TF_QINT8 = 11,
    TF_QUINT8 = 12,
    TF_QINT32 = 13,
    TF_BFLOAT16 = 14,
    TF_QINT16 = 15,
    TF_QUINT16 = 16,
    TF_UINT16 = 17,
    TF_COMPLEX128 = 18,
    TF_HALF = 19,
    TF_RESOURCE = 20,
    TF_VARIANT = 21,
    TF_UINT32 = 22,
    TF_UINT64 = 23,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Status {
    _unused: [u8; 0],
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TF_Code {
    TF_OK = 0,
    TF_CANCELLED = 1,
    TF_UNKNOWN = 2,
    TF_INVALID_ARGUMENT = 3,
    TF_DEADLINE_EXCEEDED = 4,
    TF_NOT_FOUND = 5,
    TF_ALREADY_EXISTS = 6,
    TF_PERMISSION_DENIED = 7,
    TF_UNAUTHENTICATED = 16,
    TF_RESOURCE_EXHAUSTED = 8,
    TF_FAILED_PRECONDITION = 9,
    TF_ABORTED = 10,
    TF_OUT_OF_RANGE = 11,
    TF_UNIMPLEMENTED = 12,
    TF_INTERNAL = 13,
    TF_UNAVAILABLE = 14,
    TF_DATA_LOSS = 15,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_AllocatorAttributes {
    pub struct_size: usize,
    pub on_host: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Tensor {
    _unused: [u8; 0],
}
impl TF_TString_Type {
    pub const TF_TSTR_TYPE_MASK: TF_TString_Type = TF_TString_Type::TF_TSTR_VIEW;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TF_TString_Type {
    TF_TSTR_SMALL = 0,
    TF_TSTR_LARGE = 1,
    TF_TSTR_OFFSET = 2,
    TF_TSTR_VIEW = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_TString_Large {
    pub size: usize,
    pub cap: usize,
    pub ptr: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_TString_Offset {
    pub size: u32,
    pub offset: u32,
    pub count: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_TString_View {
    pub size: usize,
    pub ptr: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_TString_Raw {
    pub raw: [u8; 24usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TF_TString_Union {
    pub large: TF_TString_Large,
    pub offset: TF_TString_Offset,
    pub view: TF_TString_View,
    pub raw: TF_TString_Raw,
}
pub const TF_TString_SmallCapacity: _bindgen_ty_1 = _bindgen_ty_1::TF_TString_SmallCapacity;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    TF_TString_SmallCapacity = 22,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_TString_Small {
    pub size: u8,
    pub str_: [::std::os::raw::c_char; 23usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TF_TString {
    pub u: TF_TString__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TF_TString__bindgen_ty_1 {
    pub smll: TF_TString_Small,
    pub large: TF_TString_Large,
    pub offset: TF_TString_Offset,
    pub view: TF_TString_View,
    pub raw: TF_TString_Raw,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Buffer {
    pub data: *const ::std::os::raw::c_void,
    pub length: usize,
    pub data_deallocator: ::std::option::Option<
        unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, length: usize),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_StringView {
    pub data: *const ::std::os::raw::c_char,
    pub len: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_SessionOptions {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Graph {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_OperationDescription {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Operation {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Input {
    pub oper: *mut TF_Operation,
    pub index: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Output {
    pub oper: *mut TF_Operation,
    pub index: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Function {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_FunctionOptions {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_AttrMetadata {
    pub is_list: ::std::os::raw::c_uchar,
    pub list_size: i64,
    pub type_: TF_AttrType,
    pub total_size: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_ImportGraphDefOptions {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_ImportGraphDefResults {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_WhileParams {
    pub ninputs: ::std::os::raw::c_int,
    pub cond_graph: *mut TF_Graph,
    pub cond_inputs: *const TF_Output,
    pub cond_output: TF_Output,
    pub body_graph: *mut TF_Graph,
    pub body_inputs: *const TF_Output,
    pub body_outputs: *mut TF_Output,
    pub name: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Session {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_DeprecatedSession {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_DeviceList {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Library {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_ApiDefMap {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TF_Server {
    _unused: [u8; 0],
}
