// This file is generated by rust-protobuf 2.4.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ChecksumRequest {
    // message fields
    start_ts: ::std::option::Option<u64>,
    scan_on: ::std::option::Option<ChecksumScanOn>,
    algorithm: ::std::option::Option<ChecksumAlgorithm>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ChecksumRequest {
    pub fn new() -> ChecksumRequest {
        ::std::default::Default::default()
    }

    // optional uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts.unwrap_or(0)
    }

    // optional .tipb.ChecksumScanOn scan_on = 2;

    pub fn clear_scan_on(&mut self) {
        self.scan_on = ::std::option::Option::None;
    }

    pub fn has_scan_on(&self) -> bool {
        self.scan_on.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scan_on(&mut self, v: ChecksumScanOn) {
        self.scan_on = ::std::option::Option::Some(v);
    }

    pub fn get_scan_on(&self) -> ChecksumScanOn {
        self.scan_on.unwrap_or(ChecksumScanOn::Table)
    }

    // optional .tipb.ChecksumAlgorithm algorithm = 3;

    pub fn clear_algorithm(&mut self) {
        self.algorithm = ::std::option::Option::None;
    }

    pub fn has_algorithm(&self) -> bool {
        self.algorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_algorithm(&mut self, v: ChecksumAlgorithm) {
        self.algorithm = ::std::option::Option::Some(v);
    }

    pub fn get_algorithm(&self) -> ChecksumAlgorithm {
        self.algorithm.unwrap_or(ChecksumAlgorithm::Crc64_Xor)
    }
}

impl ::protobuf::Message for ChecksumRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.scan_on, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.algorithm, 3, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_ts {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scan_on {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.algorithm {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.scan_on {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.algorithm {
            os.write_enum(3, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ChecksumRequest {
        ChecksumRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    |m: &ChecksumRequest| { &m.start_ts },
                    |m: &mut ChecksumRequest| { &mut m.start_ts },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumScanOn>>(
                    "scan_on",
                    |m: &ChecksumRequest| { &m.scan_on },
                    |m: &mut ChecksumRequest| { &mut m.scan_on },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumAlgorithm>>(
                    "algorithm",
                    |m: &ChecksumRequest| { &m.algorithm },
                    |m: &mut ChecksumRequest| { &mut m.algorithm },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumRequest>(
                    "ChecksumRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ChecksumRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumRequest,
        };
        unsafe {
            instance.get(ChecksumRequest::new)
        }
    }
}

impl ::protobuf::Clear for ChecksumRequest {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_scan_on();
        self.clear_algorithm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChecksumResponse {
    // message fields
    checksum: ::std::option::Option<u64>,
    total_kvs: ::std::option::Option<u64>,
    total_bytes: ::std::option::Option<u64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ChecksumResponse {
    pub fn new() -> ChecksumResponse {
        ::std::default::Default::default()
    }

    // optional uint64 checksum = 1;

    pub fn clear_checksum(&mut self) {
        self.checksum = ::std::option::Option::None;
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: u64) {
        self.checksum = ::std::option::Option::Some(v);
    }

    pub fn get_checksum(&self) -> u64 {
        self.checksum.unwrap_or(0)
    }

    // optional uint64 total_kvs = 2;

    pub fn clear_total_kvs(&mut self) {
        self.total_kvs = ::std::option::Option::None;
    }

    pub fn has_total_kvs(&self) -> bool {
        self.total_kvs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_kvs(&mut self, v: u64) {
        self.total_kvs = ::std::option::Option::Some(v);
    }

    pub fn get_total_kvs(&self) -> u64 {
        self.total_kvs.unwrap_or(0)
    }

    // optional uint64 total_bytes = 3;

    pub fn clear_total_bytes(&mut self) {
        self.total_bytes = ::std::option::Option::None;
    }

    pub fn has_total_bytes(&self) -> bool {
        self.total_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_bytes(&mut self, v: u64) {
        self.total_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.total_bytes.unwrap_or(0)
    }
}

impl ::protobuf::Message for ChecksumResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.checksum = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_kvs = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_bytes = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.checksum {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_kvs {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_bytes {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.checksum {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.total_kvs {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.total_bytes {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ChecksumResponse {
        ChecksumResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "checksum",
                    |m: &ChecksumResponse| { &m.checksum },
                    |m: &mut ChecksumResponse| { &mut m.checksum },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_kvs",
                    |m: &ChecksumResponse| { &m.total_kvs },
                    |m: &mut ChecksumResponse| { &mut m.total_kvs },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_bytes",
                    |m: &ChecksumResponse| { &m.total_bytes },
                    |m: &mut ChecksumResponse| { &mut m.total_bytes },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumResponse>(
                    "ChecksumResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ChecksumResponse {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumResponse,
        };
        unsafe {
            instance.get(ChecksumResponse::new)
        }
    }
}

impl ::protobuf::Clear for ChecksumResponse {
    fn clear(&mut self) {
        self.clear_checksum();
        self.clear_total_kvs();
        self.clear_total_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumScanOn {
    Table = 0,
    Index = 1,
}

impl ::protobuf::ProtobufEnum for ChecksumScanOn {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumScanOn> {
        match value {
            0 => ::std::option::Option::Some(ChecksumScanOn::Table),
            1 => ::std::option::Option::Some(ChecksumScanOn::Index),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumScanOn] = &[
            ChecksumScanOn::Table,
            ChecksumScanOn::Index,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumScanOn", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumScanOn {
}

impl ::std::default::Default for ChecksumScanOn {
    fn default() -> Self {
        ChecksumScanOn::Table
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumScanOn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumAlgorithm {
    Crc64_Xor = 0,
}

impl ::protobuf::ProtobufEnum for ChecksumAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(ChecksumAlgorithm::Crc64_Xor),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumAlgorithm] = &[
            ChecksumAlgorithm::Crc64_Xor,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumAlgorithm {
}

impl ::std::default::Default for ChecksumAlgorithm {
    fn default() -> Self {
        ChecksumAlgorithm::Crc64_Xor
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumAlgorithm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0echecksum.proto\x12\x04tipb\x1a\x14gogoproto/gogo.proto\"\xa4\x01\n\
    \x0fChecksumRequest\x12\x1f\n\x08start_ts\x18\x01\x20\x01(\x04R\x07start\
    TsB\x04\xc8\xde\x1f\0\x123\n\x07scan_on\x18\x02\x20\x01(\x0e2\x14.tipb.C\
    hecksumScanOnR\x06scanOnB\x04\xc8\xde\x1f\0\x12;\n\talgorithm\x18\x03\
    \x20\x01(\x0e2\x17.tipb.ChecksumAlgorithmR\talgorithmB\x04\xc8\xde\x1f\0\
    \"~\n\x10ChecksumResponse\x12\x20\n\x08checksum\x18\x01\x20\x01(\x04R\
    \x08checksumB\x04\xc8\xde\x1f\0\x12!\n\ttotal_kvs\x18\x02\x20\x01(\x04R\
    \x08totalKvsB\x04\xc8\xde\x1f\0\x12%\n\x0btotal_bytes\x18\x03\x20\x01(\
    \x04R\ntotalBytesB\x04\xc8\xde\x1f\0*&\n\x0eChecksumScanOn\x12\t\n\x05Ta\
    ble\x10\0\x12\t\n\x05Index\x10\x01*\"\n\x11ChecksumAlgorithm\x12\r\n\tCr\
    c64_Xor\x10\0B%\n\x15com.pingcap.tidb.tipbP\x01\xc8\xe2\x1e\x01\xd0\xe2\
    \x1e\x01\xe0\xe2\x1e\x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
