use super::types::*;
use crate::link;
link! {
    extern "C" {
        pub fn TF_DataTypeSize(dt: TF_DataType) -> usize;
    }
    extern "C" {
        pub fn TF_NewStatus() -> *mut TF_Status;
    }
    extern "C" {
        pub fn TF_DeleteStatus(arg1: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_SetStatus(s: *mut TF_Status, code: TF_Code, msg: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn TF_SetStatusFromIOError(
            s: *mut TF_Status,
            error_code: ::std::os::raw::c_int,
            context: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn TF_GetCode(s: *const TF_Status) -> TF_Code;
    }
    extern "C" {
        pub fn TF_Message(s: *const TF_Status) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_NewTensor(
            arg1: TF_DataType,
            dims: *const i64,
            num_dims: ::std::os::raw::c_int,
            data: *mut ::std::os::raw::c_void,
            len: usize,
            deallocator: ::std::option::Option<
                unsafe extern "C" fn(
                    data: *mut ::std::os::raw::c_void,
                    len: usize,
                    arg: *mut ::std::os::raw::c_void,
                ),
            >,
            deallocator_arg: *mut ::std::os::raw::c_void,
        ) -> *mut TF_Tensor;
    }
    extern "C" {
        pub fn TF_AllocateTensor(
            arg1: TF_DataType,
            dims: *const i64,
            num_dims: ::std::os::raw::c_int,
            len: usize,
        ) -> *mut TF_Tensor;
    }
    extern "C" {
        pub fn TF_TensorMaybeMove(tensor: *mut TF_Tensor) -> *mut TF_Tensor;
    }
    extern "C" {
        pub fn TF_DeleteTensor(arg1: *mut TF_Tensor);
    }
    extern "C" {
        pub fn TF_TensorType(arg1: *const TF_Tensor) -> TF_DataType;
    }
    extern "C" {
        pub fn TF_NumDims(arg1: *const TF_Tensor) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_Dim(tensor: *const TF_Tensor, dim_index: ::std::os::raw::c_int) -> i64;
    }
    extern "C" {
        pub fn TF_TensorByteSize(arg1: *const TF_Tensor) -> usize;
    }
    extern "C" {
        pub fn TF_TensorData(arg1: *const TF_Tensor) -> *mut ::std::os::raw::c_void;
    }
    extern "C" {
        pub fn TF_TensorElementCount(tensor: *const TF_Tensor) -> i64;
    }
    extern "C" {
        pub fn TF_TensorBitcastFrom(
            from: *const TF_Tensor,
            type_: TF_DataType,
            to: *mut TF_Tensor,
            new_dims: *const i64,
            num_new_dims: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_TensorIsAligned(arg1: *const TF_Tensor) -> bool;
    }
    extern "C" {
        pub fn TF_StringInit(t: *mut TF_TString);
    }
    extern "C" {
        pub fn TF_StringCopy(dst: *mut TF_TString, src: *const ::std::os::raw::c_char, size: usize);
    }
    extern "C" {
        pub fn TF_StringAssignView(
            dst: *mut TF_TString,
            src: *const ::std::os::raw::c_char,
            size: usize,
        );
    }
    extern "C" {
        pub fn TF_StringGetDataPointer(tstr: *const TF_TString) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_StringGetType(str_: *const TF_TString) -> TF_TString_Type;
    }
    extern "C" {
        pub fn TF_StringGetSize(tstr: *const TF_TString) -> usize;
    }
    extern "C" {
        pub fn TF_StringGetCapacity(str_: *const TF_TString) -> usize;
    }
    extern "C" {
        pub fn TF_StringDealloc(tstr: *mut TF_TString);
    }
    extern "C" {
        pub fn TF_Version() -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_NewBufferFromString(
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
        ) -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_NewBuffer() -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_DeleteBuffer(arg1: *mut TF_Buffer);
    }
    extern "C" {
        pub fn TF_GetBuffer(buffer: *mut TF_Buffer) -> TF_Buffer;
    }
    extern "C" {
        pub fn TF_NewSessionOptions() -> *mut TF_SessionOptions;
    }
    extern "C" {
        pub fn TF_SetTarget(options: *mut TF_SessionOptions, target: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn TF_SetConfig(
            options: *mut TF_SessionOptions,
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_DeleteSessionOptions(arg1: *mut TF_SessionOptions);
    }
    extern "C" {
        pub fn TF_NewGraph() -> *mut TF_Graph;
    }
    extern "C" {
        pub fn TF_DeleteGraph(arg1: *mut TF_Graph);
    }
    extern "C" {
        pub fn TF_GraphSetTensorShape(
            graph: *mut TF_Graph,
            output: TF_Output,
            dims: *const i64,
            num_dims: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphGetTensorNumDims(
            graph: *mut TF_Graph,
            output: TF_Output,
            status: *mut TF_Status,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_GraphGetTensorShape(
            graph: *mut TF_Graph,
            output: TF_Output,
            dims: *mut i64,
            num_dims: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_NewOperation(
            graph: *mut TF_Graph,
            op_type: *const ::std::os::raw::c_char,
            oper_name: *const ::std::os::raw::c_char,
        ) -> *mut TF_OperationDescription;
    }
    extern "C" {
        pub fn TF_SetDevice(desc: *mut TF_OperationDescription, device: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn TF_AddInput(desc: *mut TF_OperationDescription, input: TF_Output);
    }
    extern "C" {
        pub fn TF_AddInputList(
            desc: *mut TF_OperationDescription,
            inputs: *const TF_Output,
            num_inputs: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_AddControlInput(desc: *mut TF_OperationDescription, input: *mut TF_Operation);
    }
    extern "C" {
        pub fn TF_ColocateWith(desc: *mut TF_OperationDescription, op: *mut TF_Operation);
    }
    extern "C" {
        pub fn TF_SetAttrString(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_void,
            length: usize,
        );
    }
    extern "C" {
        pub fn TF_SetAttrStringList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const *const ::std::os::raw::c_void,
            lengths: *const usize,
            num_values: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrInt(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: i64,
        );
    }
    extern "C" {
        pub fn TF_SetAttrIntList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const i64,
            num_values: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrFloat(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: f32,
        );
    }
    extern "C" {
        pub fn TF_SetAttrFloatList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const f32,
            num_values: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrBool(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: ::std::os::raw::c_uchar,
        );
    }
    extern "C" {
        pub fn TF_SetAttrBoolList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const ::std::os::raw::c_uchar,
            num_values: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrType(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: TF_DataType,
        );
    }
    extern "C" {
        pub fn TF_SetAttrTypeList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const TF_DataType,
            num_values: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrPlaceholder(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            placeholder: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn TF_SetAttrFuncName(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
            length: usize,
        );
    }
    extern "C" {
        pub fn TF_SetAttrShape(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            dims: *const i64,
            num_dims: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrShapeList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            dims: *const *const i64,
            num_dims: *const ::std::os::raw::c_int,
            num_shapes: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_SetAttrTensorShapeProto(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SetAttrTensorShapeProtoList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            protos: *const *const ::std::os::raw::c_void,
            proto_lens: *const usize,
            num_shapes: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SetAttrTensor(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut TF_Tensor,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SetAttrTensorList(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            values: *const *mut TF_Tensor,
            num_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SetAttrValueProto(
            desc: *mut TF_OperationDescription,
            attr_name: *const ::std::os::raw::c_char,
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_FinishOperation(
            desc: *mut TF_OperationDescription,
            status: *mut TF_Status,
        ) -> *mut TF_Operation;
    }
    extern "C" {
        pub fn TF_OperationName(oper: *mut TF_Operation) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_OperationOpType(oper: *mut TF_Operation) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_OperationDevice(oper: *mut TF_Operation) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_OperationNumOutputs(oper: *mut TF_Operation) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationOutputType(oper_out: TF_Output) -> TF_DataType;
    }
    extern "C" {
        pub fn TF_OperationOutputListLength(
            oper: *mut TF_Operation,
            arg_name: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationNumInputs(oper: *mut TF_Operation) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationInputType(oper_in: TF_Input) -> TF_DataType;
    }
    extern "C" {
        pub fn TF_OperationInputListLength(
            oper: *mut TF_Operation,
            arg_name: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationInput(oper_in: TF_Input) -> TF_Output;
    }
    extern "C" {
        pub fn TF_OperationAllInputs(
            oper: *mut TF_Operation,
            inputs: *mut TF_Output,
            max_inputs: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_OperationOutputNumConsumers(oper_out: TF_Output) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationOutputConsumers(
            oper_out: TF_Output,
            consumers: *mut TF_Input,
            max_consumers: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationNumControlInputs(oper: *mut TF_Operation) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationGetControlInputs(
            oper: *mut TF_Operation,
            control_inputs: *mut *mut TF_Operation,
            max_control_inputs: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationNumControlOutputs(oper: *mut TF_Operation) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationGetControlOutputs(
            oper: *mut TF_Operation,
            control_outputs: *mut *mut TF_Operation,
            max_control_outputs: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationGetAttrMetadata(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        ) -> TF_AttrMetadata;
    }
    extern "C" {
        pub fn TF_OperationGetAttrString(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
            max_length: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrStringList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut *mut ::std::os::raw::c_void,
            lengths: *mut usize,
            max_values: ::std::os::raw::c_int,
            storage: *mut ::std::os::raw::c_void,
            storage_size: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrInt(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut i64,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrIntList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut i64,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrFloat(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut f32,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrFloatList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut f32,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrBool(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_uchar,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrBoolList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut ::std::os::raw::c_uchar,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrType(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut TF_DataType,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrTypeList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut TF_DataType,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrShape(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut i64,
            num_dims: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrShapeList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            dims: *mut *mut i64,
            num_dims: *mut ::std::os::raw::c_int,
            num_shapes: ::std::os::raw::c_int,
            storage: *mut i64,
            storage_size: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrTensorShapeProto(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrTensorShapeProtoList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut *mut TF_Buffer,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrTensor(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            value: *mut *mut TF_Tensor,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrTensorList(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            values: *mut *mut TF_Tensor,
            max_values: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_OperationGetAttrValueProto(
            oper: *mut TF_Operation,
            attr_name: *const ::std::os::raw::c_char,
            output_attr_value: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphOperationByName(
            graph: *mut TF_Graph,
            oper_name: *const ::std::os::raw::c_char,
        ) -> *mut TF_Operation;
    }
    extern "C" {
        pub fn TF_GraphNextOperation(graph: *mut TF_Graph, pos: *mut usize) -> *mut TF_Operation;
    }
    extern "C" {
        pub fn TF_GraphToGraphDef(
            graph: *mut TF_Graph,
            output_graph_def: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphGetOpDef(
            graph: *mut TF_Graph,
            op_name: *const ::std::os::raw::c_char,
            output_op_def: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphVersions(
            graph: *mut TF_Graph,
            output_version_def: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_NewImportGraphDefOptions() -> *mut TF_ImportGraphDefOptions;
    }
    extern "C" {
        pub fn TF_DeleteImportGraphDefOptions(opts: *mut TF_ImportGraphDefOptions);
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsSetPrefix(
            opts: *mut TF_ImportGraphDefOptions,
            prefix: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsSetDefaultDevice(
            opts: *mut TF_ImportGraphDefOptions,
            device: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsSetUniquifyNames(
            opts: *mut TF_ImportGraphDefOptions,
            uniquify_names: ::std::os::raw::c_uchar,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsSetUniquifyPrefix(
            opts: *mut TF_ImportGraphDefOptions,
            uniquify_prefix: ::std::os::raw::c_uchar,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsAddInputMapping(
            opts: *mut TF_ImportGraphDefOptions,
            src_name: *const ::std::os::raw::c_char,
            src_index: ::std::os::raw::c_int,
            dst: TF_Output,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsRemapControlDependency(
            opts: *mut TF_ImportGraphDefOptions,
            src_name: *const ::std::os::raw::c_char,
            dst: *mut TF_Operation,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsAddControlDependency(
            opts: *mut TF_ImportGraphDefOptions,
            oper: *mut TF_Operation,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsAddReturnOutput(
            opts: *mut TF_ImportGraphDefOptions,
            oper_name: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsNumReturnOutputs(
            opts: *const TF_ImportGraphDefOptions,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsAddReturnOperation(
            opts: *mut TF_ImportGraphDefOptions,
            oper_name: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefOptionsNumReturnOperations(
            opts: *const TF_ImportGraphDefOptions,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_ImportGraphDefResultsReturnOutputs(
            results: *mut TF_ImportGraphDefResults,
            num_outputs: *mut ::std::os::raw::c_int,
            outputs: *mut *mut TF_Output,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefResultsReturnOperations(
            results: *mut TF_ImportGraphDefResults,
            num_opers: *mut ::std::os::raw::c_int,
            opers: *mut *mut *mut TF_Operation,
        );
    }
    extern "C" {
        pub fn TF_ImportGraphDefResultsMissingUnusedInputMappings(
            results: *mut TF_ImportGraphDefResults,
            num_missing_unused_input_mappings: *mut ::std::os::raw::c_int,
            src_names: *mut *mut *const ::std::os::raw::c_char,
            src_indexes: *mut *mut ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn TF_DeleteImportGraphDefResults(results: *mut TF_ImportGraphDefResults);
    }
    extern "C" {
        pub fn TF_GraphImportGraphDefWithResults(
            graph: *mut TF_Graph,
            graph_def: *const TF_Buffer,
            options: *const TF_ImportGraphDefOptions,
            status: *mut TF_Status,
        ) -> *mut TF_ImportGraphDefResults;
    }
    extern "C" {
        pub fn TF_GraphImportGraphDefWithReturnOutputs(
            graph: *mut TF_Graph,
            graph_def: *const TF_Buffer,
            options: *const TF_ImportGraphDefOptions,
            return_outputs: *mut TF_Output,
            num_return_outputs: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphImportGraphDef(
            graph: *mut TF_Graph,
            graph_def: *const TF_Buffer,
            options: *const TF_ImportGraphDefOptions,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphCopyFunction(
            g: *mut TF_Graph,
            func: *const TF_Function,
            grad: *const TF_Function,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_GraphNumFunctions(g: *mut TF_Graph) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_GraphGetFunctions(
            g: *mut TF_Graph,
            funcs: *mut *mut TF_Function,
            max_func: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_OperationToNodeDef(
            oper: *mut TF_Operation,
            output_node_def: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_NewWhile(
            g: *mut TF_Graph,
            inputs: *mut TF_Output,
            ninputs: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> TF_WhileParams;
    }
    extern "C" {
        pub fn TF_FinishWhile(
            params: *const TF_WhileParams,
            status: *mut TF_Status,
            outputs: *mut TF_Output,
        );
    }
    extern "C" {
        pub fn TF_AbortWhile(params: *const TF_WhileParams);
    }
    extern "C" {
        pub fn TF_AddGradients(
            g: *mut TF_Graph,
            y: *mut TF_Output,
            ny: ::std::os::raw::c_int,
            x: *mut TF_Output,
            nx: ::std::os::raw::c_int,
            dx: *mut TF_Output,
            status: *mut TF_Status,
            dy: *mut TF_Output,
        );
    }
    extern "C" {
        pub fn TF_AddGradientsWithPrefix(
            g: *mut TF_Graph,
            prefix: *const ::std::os::raw::c_char,
            y: *mut TF_Output,
            ny: ::std::os::raw::c_int,
            x: *mut TF_Output,
            nx: ::std::os::raw::c_int,
            dx: *mut TF_Output,
            status: *mut TF_Status,
            dy: *mut TF_Output,
        );
    }
    // extern "C" {
    //     pub fn TF_GraphToFunction(
    //         fn_body: *const TF_Graph,
    //         fn_name: *const ::std::os::raw::c_char,
    //         append_hash_to_fn_name: ::std::os::raw::c_uchar,
    //         num_opers: ::std::os::raw::c_int,
    //         opers: *const *const TF_Operation,
    //         ninputs: ::std::os::raw::c_int,
    //         inputs: *const TF_Output,
    //         noutputs: ::std::os::raw::c_int,
    //         outputs: *const TF_Output,
    //         output_names: *const *const ::std::os::raw::c_char,
    //         opts: *const TF_FunctionOptions,
    //         description: *const ::std::os::raw::c_char,
    //         status: *mut TF_Status,
    //     ) -> *mut TF_Function;
    // }
    // extern "C" {
    //     pub fn TF_GraphToFunctionWithControlOutputs(
    //         fn_body: *const TF_Graph,
    //         fn_name: *const ::std::os::raw::c_char,
    //         append_hash_to_fn_name: ::std::os::raw::c_uchar,
    //         num_opers: ::std::os::raw::c_int,
    //         opers: *const *const TF_Operation,
    //         ninputs: ::std::os::raw::c_int,
    //         inputs: *const TF_Output,
    //         noutputs: ::std::os::raw::c_int,
    //         outputs: *const TF_Output,
    //         output_names: *const *const ::std::os::raw::c_char,
    //         ncontrol_outputs: ::std::os::raw::c_int,
    //         control_outputs: *const *const TF_Operation,
    //         control_output_names: *const *const ::std::os::raw::c_char,
    //         opts: *const TF_FunctionOptions,
    //         description: *const ::std::os::raw::c_char,
    //         status: *mut TF_Status,
    //     ) -> *mut TF_Function;
    // }
    extern "C" {
        pub fn TF_FunctionName(func: *mut TF_Function) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_FunctionToFunctionDef(
            func: *mut TF_Function,
            output_func_def: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_FunctionImportFunctionDef(
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        ) -> *mut TF_Function;
    }
    extern "C" {
        pub fn TF_FunctionSetAttrValueProto(
            func: *mut TF_Function,
            attr_name: *const ::std::os::raw::c_char,
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_FunctionGetAttrValueProto(
            func: *mut TF_Function,
            attr_name: *const ::std::os::raw::c_char,
            output_attr_value: *mut TF_Buffer,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_DeleteFunction(func: *mut TF_Function);
    }
    extern "C" {
        pub fn TF_TryEvaluateConstant(
            graph: *mut TF_Graph,
            output: TF_Output,
            result: *mut *mut TF_Tensor,
            status: *mut TF_Status,
        ) -> ::std::os::raw::c_uchar;
    }
    extern "C" {
        pub fn TF_NewSession(
            graph: *mut TF_Graph,
            opts: *const TF_SessionOptions,
            status: *mut TF_Status,
        ) -> *mut TF_Session;
    }
    extern "C" {
        pub fn TF_LoadSessionFromSavedModel(
            session_options: *const TF_SessionOptions,
            run_options: *const TF_Buffer,
            export_dir: *const ::std::os::raw::c_char,
            tags: *const *const ::std::os::raw::c_char,
            tags_len: ::std::os::raw::c_int,
            graph: *mut TF_Graph,
            meta_graph_def: *mut TF_Buffer,
            status: *mut TF_Status,
        ) -> *mut TF_Session;
    }
    extern "C" {
        pub fn TF_CloseSession(arg1: *mut TF_Session, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_DeleteSession(arg1: *mut TF_Session, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_SessionRun(
            session: *mut TF_Session,
            run_options: *const TF_Buffer,
            inputs: *const TF_Output,
            input_values: *const *mut TF_Tensor,
            ninputs: ::std::os::raw::c_int,
            outputs: *const TF_Output,
            output_values: *mut *mut TF_Tensor,
            noutputs: ::std::os::raw::c_int,
            target_opers: *const *const TF_Operation,
            ntargets: ::std::os::raw::c_int,
            run_metadata: *mut TF_Buffer,
            arg1: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SessionPRunSetup(
            arg1: *mut TF_Session,
            inputs: *const TF_Output,
            ninputs: ::std::os::raw::c_int,
            outputs: *const TF_Output,
            noutputs: ::std::os::raw::c_int,
            target_opers: *const *const TF_Operation,
            ntargets: ::std::os::raw::c_int,
            handle: *mut *const ::std::os::raw::c_char,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SessionPRun(
            arg1: *mut TF_Session,
            handle: *const ::std::os::raw::c_char,
            inputs: *const TF_Output,
            input_values: *const *mut TF_Tensor,
            ninputs: ::std::os::raw::c_int,
            outputs: *const TF_Output,
            output_values: *mut *mut TF_Tensor,
            noutputs: ::std::os::raw::c_int,
            target_opers: *const *const TF_Operation,
            ntargets: ::std::os::raw::c_int,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_DeletePRunHandle(handle: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn TF_NewDeprecatedSession(
            arg1: *const TF_SessionOptions,
            status: *mut TF_Status,
        ) -> *mut TF_DeprecatedSession;
    }
    extern "C" {
        pub fn TF_CloseDeprecatedSession(arg1: *mut TF_DeprecatedSession, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_DeleteDeprecatedSession(arg1: *mut TF_DeprecatedSession, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_Reset(
            opt: *const TF_SessionOptions,
            containers: *mut *const ::std::os::raw::c_char,
            ncontainers: ::std::os::raw::c_int,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_ExtendGraph(
            arg1: *mut TF_DeprecatedSession,
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_Run(
            arg1: *mut TF_DeprecatedSession,
            run_options: *const TF_Buffer,
            input_names: *mut *const ::std::os::raw::c_char,
            inputs: *mut *mut TF_Tensor,
            ninputs: ::std::os::raw::c_int,
            output_names: *mut *const ::std::os::raw::c_char,
            outputs: *mut *mut TF_Tensor,
            noutputs: ::std::os::raw::c_int,
            target_oper_names: *mut *const ::std::os::raw::c_char,
            ntargets: ::std::os::raw::c_int,
            run_metadata: *mut TF_Buffer,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_PRunSetup(
            arg1: *mut TF_DeprecatedSession,
            input_names: *mut *const ::std::os::raw::c_char,
            ninputs: ::std::os::raw::c_int,
            output_names: *mut *const ::std::os::raw::c_char,
            noutputs: ::std::os::raw::c_int,
            target_oper_names: *mut *const ::std::os::raw::c_char,
            ntargets: ::std::os::raw::c_int,
            handle: *mut *const ::std::os::raw::c_char,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_PRun(
            arg1: *mut TF_DeprecatedSession,
            handle: *const ::std::os::raw::c_char,
            input_names: *mut *const ::std::os::raw::c_char,
            inputs: *mut *mut TF_Tensor,
            ninputs: ::std::os::raw::c_int,
            output_names: *mut *const ::std::os::raw::c_char,
            outputs: *mut *mut TF_Tensor,
            noutputs: ::std::os::raw::c_int,
            target_oper_names: *mut *const ::std::os::raw::c_char,
            ntargets: ::std::os::raw::c_int,
            arg2: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_SessionListDevices(
            session: *mut TF_Session,
            status: *mut TF_Status,
        ) -> *mut TF_DeviceList;
    }
    extern "C" {
        pub fn TF_DeprecatedSessionListDevices(
            session: *mut TF_DeprecatedSession,
            status: *mut TF_Status,
        ) -> *mut TF_DeviceList;
    }
    extern "C" {
        pub fn TF_DeleteDeviceList(list: *mut TF_DeviceList);
    }
    extern "C" {
        pub fn TF_DeviceListCount(list: *const TF_DeviceList) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn TF_DeviceListName(
            list: *const TF_DeviceList,
            index: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_DeviceListType(
            list: *const TF_DeviceList,
            index: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_DeviceListMemoryBytes(
            list: *const TF_DeviceList,
            index: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> i64;
    }
    extern "C" {
        pub fn TF_DeviceListIncarnation(
            list: *const TF_DeviceList,
            index: ::std::os::raw::c_int,
            status: *mut TF_Status,
        ) -> u64;
    }
    extern "C" {
        pub fn TF_LoadLibrary(
            library_filename: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        ) -> *mut TF_Library;
    }
    extern "C" {
        pub fn TF_GetOpList(lib_handle: *mut TF_Library) -> TF_Buffer;
    }
    extern "C" {
        pub fn TF_DeleteLibraryHandle(lib_handle: *mut TF_Library);
    }
    extern "C" {
        pub fn TF_GetAllOpList() -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_NewApiDefMap(
            op_list_buffer: *mut TF_Buffer,
            status: *mut TF_Status,
        ) -> *mut TF_ApiDefMap;
    }
    extern "C" {
        pub fn TF_DeleteApiDefMap(apimap: *mut TF_ApiDefMap);
    }
    extern "C" {
        pub fn TF_ApiDefMapPut(
            api_def_map: *mut TF_ApiDefMap,
            text: *const ::std::os::raw::c_char,
            text_len: usize,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_ApiDefMapGet(
            api_def_map: *mut TF_ApiDefMap,
            name: *const ::std::os::raw::c_char,
            name_len: usize,
            status: *mut TF_Status,
        ) -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_GetAllRegisteredKernels(status: *mut TF_Status) -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_GetRegisteredKernelsForOp(
            name: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        ) -> *mut TF_Buffer;
    }
    extern "C" {
        pub fn TF_UpdateEdge(
            graph: *mut TF_Graph,
            new_src: TF_Output,
            dst: TF_Input,
            status: *mut TF_Status,
        );
    }
    extern "C" {
        pub fn TF_NewServer(
            proto: *const ::std::os::raw::c_void,
            proto_len: usize,
            status: *mut TF_Status,
        ) -> *mut TF_Server;
    }
    extern "C" {
        pub fn TF_ServerStart(server: *mut TF_Server, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_ServerStop(server: *mut TF_Server, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_ServerJoin(server: *mut TF_Server, status: *mut TF_Status);
    }
    extern "C" {
        pub fn TF_ServerTarget(server: *mut TF_Server) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn TF_DeleteServer(server: *mut TF_Server);
    }
    extern "C" {
        pub fn TF_RegisterLogListener(
            listener: ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
        );
    }
    extern "C" {
        pub fn TF_RegisterFilesystemPlugin(
            plugin_filename: *const ::std::os::raw::c_char,
            status: *mut TF_Status,
        );
    }
}