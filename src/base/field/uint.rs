pub const UINT1_SIZE: usize = 1; // 255
pub const UINT2_SIZE: usize = 2; // 65535
pub const UINT3_SIZE: usize = 3; // 16777215
pub const UINT4_SIZE: usize = 4; // 4294967295
pub const UINT5_SIZE: usize = 5; // 1099511627775
pub const UINT6_SIZE: usize = 6; // 
pub const UINT7_SIZE: usize = 7;
pub const UINT8_SIZE: usize = 8;

pub const UINT1_SIZE_VL: usize = 1;
pub const UINT2_SIZE_VL: usize = 2;
pub const UINT3_SIZE_VL: usize = 4;
pub const UINT4_SIZE_VL: usize = 4;
pub const UINT5_SIZE_VL: usize = 8;
pub const UINT6_SIZE_VL: usize = 8;
pub const UINT7_SIZE_VL: usize = 8;
pub const UINT8_SIZE_VL: usize = 8;

/////////////////////////////////




// create struct
StructFieldUint!(Uint1,  u8, UINT1_SIZE, UINT1_SIZE_VL);
StructFieldUint!(Uint2, u16, UINT2_SIZE, UINT2_SIZE_VL);
StructFieldUint!(Uint3, u32, UINT3_SIZE, UINT3_SIZE_VL);
StructFieldUint!(Uint4, u32, UINT4_SIZE, UINT4_SIZE_VL);
StructFieldUint!(Uint5, u64, UINT5_SIZE, UINT5_SIZE_VL);
StructFieldUint!(Uint6, u64, UINT6_SIZE, UINT6_SIZE_VL);
StructFieldUint!(Uint7, u64, UINT7_SIZE, UINT7_SIZE_VL);
StructFieldUint!(Uint8, u64, UINT8_SIZE, UINT8_SIZE_VL);

