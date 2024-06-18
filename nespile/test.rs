pub mod address_mode {
    use binrw::{io, BinRead};
    use nespile_macros::{parse_byte_with, BinReadAddressMode};
    use subenum::subenum;
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddressMode {
        /// Operand is implied Accumulator register.
        Accumulator,
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Immediate (8-bit) value.
        Immediate(u8),
        /// Implied (empty) value.
        Implied,
        /// Absolute address, the value in memory at the given absolute address.
        Indirect(u16),
        /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
        IndirectX(u8),
        /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
        IndirectY(u8),
        /// Branch target is PC plus the signed offset byte.
        Relative(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
        /// Zeropage (8-bit) address, incremented by Y without carry.
        ZeroPageY(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddressMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddressMode::Accumulator => {
                    ::core::fmt::Formatter::write_str(f, "Accumulator")
                }
                AddressMode::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddressMode::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddressMode::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddressMode::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
                AddressMode::Implied => ::core::fmt::Formatter::write_str(f, "Implied"),
                AddressMode::Indirect(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Indirect",
                        &__self_0,
                    )
                }
                AddressMode::IndirectX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectX",
                        &__self_0,
                    )
                }
                AddressMode::IndirectY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectY",
                        &__self_0,
                    )
                }
                AddressMode::Relative(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Relative",
                        &__self_0,
                    )
                }
                AddressMode::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddressMode::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
                AddressMode::ZeroPageY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddressMode {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x0a | 0x2a | 0x4a | 0x6a => Ok(AddressMode::Accumulator),
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddressMode::Absolute(u16::read_options(reader, endian, ())?))
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(AddressMode::AbsoluteX(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(AddressMode::AbsoluteY(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(AddressMode::Immediate(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1f == 0x08) || (b & 0x1f == 0x0a) || (b & 0x1f == 0x12)
                    || (b & 0x1f == 0x18) || (b & 0x1f == 0x0a) || (b & 0x9f == 0x02)
                    || (b & 0x9f == 0x00 && b != 0x20) => Ok(AddressMode::Implied),
                0x6c => Ok(AddressMode::Indirect(u16::read_options(reader, endian, ())?)),
                b if (b & 0x1d == 0x01) => {
                    Ok(AddressMode::IndirectX(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x11) => {
                    Ok(AddressMode::IndirectY(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1f == 0x10) => {
                    Ok(AddressMode::Relative(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddressMode::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(AddressMode::ZeroPageX(u8::read_options(reader, endian, ())?))
                }
                0x97 | 0x97 | 0xb6 | 0xb7 => {
                    Ok(AddressMode::ZeroPageY(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeAHX {
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
        IndirectY(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAHX {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeAHX::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddrModeAHX::IndirectY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeAHX {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(AddrModeAHX::AbsoluteY(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x11) => {
                    Ok(AddrModeAHX::IndirectY(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeAHX`].
    pub struct AddrModeAHXConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeAHXConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeAHXConvertError {
        #[inline]
        fn clone(&self) -> AddrModeAHXConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAHXConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeAHXConvertError")
        }
    }
    impl core::fmt::Display for AddrModeAHXConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeAHXConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeAHX> for AddressMode {
        fn from(child: AddrModeAHX) -> Self {
            match child {
                AddrModeAHX::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
                AddrModeAHX::IndirectY(var0) => AddressMode::IndirectY(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeAHX {
        type Error = AddrModeAHXConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::AbsoluteY(var0) => Ok(AddrModeAHX::AbsoluteY(var0)),
                AddressMode::IndirectY(var0) => Ok(AddrModeAHX::IndirectY(var0)),
                _ => Err(AddrModeAHXConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeAbs {
        /// Absolute (16-bit) address.
        Absolute(u16),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbs {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeAbs::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeAbs {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeAbs::Absolute(u16::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeAbs`].
    pub struct AddrModeAbsConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeAbsConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeAbsConvertError {
        #[inline]
        fn clone(&self) -> AddrModeAbsConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeAbsConvertError")
        }
    }
    impl core::fmt::Display for AddrModeAbsConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeAbsConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeAbs> for AddressMode {
        fn from(child: AddrModeAbs) -> Self {
            match child {
                AddrModeAbs::Absolute(var0) => AddressMode::Absolute(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeAbs {
        type Error = AddrModeAbsConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeAbs::Absolute(var0)),
                _ => Err(AddrModeAbsConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeAbsInd {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute address, the value in memory at the given absolute address.
        Indirect(u16),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsInd {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeAbsInd::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeAbsInd::Indirect(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Indirect",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeAbsInd {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeAbsInd::Absolute(u16::read_options(reader, endian, ())?))
                }
                0x6c => {
                    Ok(AddrModeAbsInd::Indirect(u16::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeAbsInd`].
    pub struct AddrModeAbsIndConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeAbsIndConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeAbsIndConvertError {
        #[inline]
        fn clone(&self) -> AddrModeAbsIndConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsIndConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeAbsIndConvertError")
        }
    }
    impl core::fmt::Display for AddrModeAbsIndConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeAbsIndConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeAbsInd> for AddressMode {
        fn from(child: AddrModeAbsInd) -> Self {
            match child {
                AddrModeAbsInd::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeAbsInd::Indirect(var0) => AddressMode::Indirect(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeAbsInd {
        type Error = AddrModeAbsIndConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeAbsInd::Absolute(var0)),
                AddressMode::Indirect(var0) => Ok(AddrModeAbsInd::Indirect(var0)),
                _ => Err(AddrModeAbsIndConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeAbsX {
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsX {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeAbsX::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeAbsX {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(AddrModeAbsX::AbsoluteX(u16::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeAbsX`].
    pub struct AddrModeAbsXConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeAbsXConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeAbsXConvertError {
        #[inline]
        fn clone(&self) -> AddrModeAbsXConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsXConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeAbsXConvertError")
        }
    }
    impl core::fmt::Display for AddrModeAbsXConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeAbsXConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeAbsX> for AddressMode {
        fn from(child: AddrModeAbsX) -> Self {
            match child {
                AddrModeAbsX::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeAbsX {
        type Error = AddrModeAbsXConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::AbsoluteX(var0) => Ok(AddrModeAbsX::AbsoluteX(var0)),
                _ => Err(AddrModeAbsXConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeAbsY {
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsY {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeAbsY::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeAbsY {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(AddrModeAbsY::AbsoluteY(u16::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeAbsY`].
    pub struct AddrModeAbsYConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeAbsYConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeAbsYConvertError {
        #[inline]
        fn clone(&self) -> AddrModeAbsYConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeAbsYConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeAbsYConvertError")
        }
    }
    impl core::fmt::Display for AddrModeAbsYConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeAbsYConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeAbsY> for AddressMode {
        fn from(child: AddrModeAbsY) -> Self {
            match child {
                AddrModeAbsY::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeAbsY {
        type Error = AddrModeAbsYConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::AbsoluteY(var0) => Ok(AddrModeAbsY::AbsoluteY(var0)),
                _ => Err(AddrModeAbsYConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeImmediate {
        /// Immediate (8-bit) value.
        Immediate(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeImmediate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeImmediate::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeImmediate {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(
                        AddrModeImmediate::Immediate(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeImmediate`].
    pub struct AddrModeImmediateConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeImmediateConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeImmediateConvertError {
        #[inline]
        fn clone(&self) -> AddrModeImmediateConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeImmediateConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeImmediateConvertError")
        }
    }
    impl core::fmt::Display for AddrModeImmediateConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeImmediateConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeImmediate> for AddressMode {
        fn from(child: AddrModeImmediate) -> Self {
            match child {
                AddrModeImmediate::Immediate(var0) => AddressMode::Immediate(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeImmediate {
        type Error = AddrModeImmediateConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Immediate(var0) => Ok(AddrModeImmediate::Immediate(var0)),
                _ => Err(AddrModeImmediateConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeLAX {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
        IndirectX(u8),
        /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
        IndirectY(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by Y without carry.
        ZeroPageY(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeLAX {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeLAX::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeLAX::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddrModeLAX::IndirectX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectX",
                        &__self_0,
                    )
                }
                AddrModeLAX::IndirectY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectY",
                        &__self_0,
                    )
                }
                AddrModeLAX::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeLAX::ZeroPageY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeLAX {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeLAX::Absolute(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(AddrModeLAX::AbsoluteY(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x01) => {
                    Ok(AddrModeLAX::IndirectX(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1d == 0x11) => {
                    Ok(AddrModeLAX::IndirectY(u8::read_options(reader, endian, ())?))
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddrModeLAX::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                0x97 | 0x97 | 0xb6 | 0xb7 => {
                    Ok(AddrModeLAX::ZeroPageY(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeLAX`].
    pub struct AddrModeLAXConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeLAXConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeLAXConvertError {
        #[inline]
        fn clone(&self) -> AddrModeLAXConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeLAXConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeLAXConvertError")
        }
    }
    impl core::fmt::Display for AddrModeLAXConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeLAXConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeLAX> for AddressMode {
        fn from(child: AddrModeLAX) -> Self {
            match child {
                AddrModeLAX::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeLAX::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
                AddrModeLAX::IndirectX(var0) => AddressMode::IndirectX(var0),
                AddrModeLAX::IndirectY(var0) => AddressMode::IndirectY(var0),
                AddrModeLAX::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeLAX::ZeroPageY(var0) => AddressMode::ZeroPageY(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeLAX {
        type Error = AddrModeLAXConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeLAX::Absolute(var0)),
                AddressMode::AbsoluteY(var0) => Ok(AddrModeLAX::AbsoluteY(var0)),
                AddressMode::IndirectX(var0) => Ok(AddrModeLAX::IndirectX(var0)),
                AddressMode::IndirectY(var0) => Ok(AddrModeLAX::IndirectY(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeLAX::ZeroPage(var0)),
                AddressMode::ZeroPageY(var0) => Ok(AddrModeLAX::ZeroPageY(var0)),
                _ => Err(AddrModeLAXConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeNoZeropageY {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Immediate (8-bit) value.
        Immediate(u8),
        /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
        IndirectX(u8),
        /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
        IndirectY(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeNoZeropageY {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeNoZeropageY::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::IndirectX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectX",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::IndirectY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectY",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageY::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeNoZeropageY {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeNoZeropageY::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(
                        AddrModeNoZeropageY::AbsoluteX(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(
                        AddrModeNoZeropageY::AbsoluteY(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(
                        AddrModeNoZeropageY::Immediate(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x01) => {
                    Ok(
                        AddrModeNoZeropageY::IndirectX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x11) => {
                    Ok(
                        AddrModeNoZeropageY::IndirectY(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeNoZeropageY::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(
                        AddrModeNoZeropageY::ZeroPageX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeNoZeropageY`].
    pub struct AddrModeNoZeropageYConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeNoZeropageYConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeNoZeropageYConvertError {
        #[inline]
        fn clone(&self) -> AddrModeNoZeropageYConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeNoZeropageYConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeNoZeropageYConvertError")
        }
    }
    impl core::fmt::Display for AddrModeNoZeropageYConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeNoZeropageYConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeNoZeropageY> for AddressMode {
        fn from(child: AddrModeNoZeropageY) -> Self {
            match child {
                AddrModeNoZeropageY::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeNoZeropageY::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
                AddrModeNoZeropageY::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
                AddrModeNoZeropageY::Immediate(var0) => AddressMode::Immediate(var0),
                AddrModeNoZeropageY::IndirectX(var0) => AddressMode::IndirectX(var0),
                AddrModeNoZeropageY::IndirectY(var0) => AddressMode::IndirectY(var0),
                AddrModeNoZeropageY::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeNoZeropageY::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeNoZeropageY {
        type Error = AddrModeNoZeropageYConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeNoZeropageY::Absolute(var0)),
                AddressMode::AbsoluteX(var0) => Ok(AddrModeNoZeropageY::AbsoluteX(var0)),
                AddressMode::AbsoluteY(var0) => Ok(AddrModeNoZeropageY::AbsoluteY(var0)),
                AddressMode::Immediate(var0) => Ok(AddrModeNoZeropageY::Immediate(var0)),
                AddressMode::IndirectX(var0) => Ok(AddrModeNoZeropageY::IndirectX(var0)),
                AddressMode::IndirectY(var0) => Ok(AddrModeNoZeropageY::IndirectY(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeNoZeropageY::ZeroPage(var0)),
                AddressMode::ZeroPageX(var0) => Ok(AddrModeNoZeropageY::ZeroPageX(var0)),
                _ => Err(AddrModeNoZeropageYConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeNoZeropageYNoImm {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
        IndirectX(u8),
        /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
        IndirectY(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeNoZeropageYNoImm {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeNoZeropageYNoImm::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::IndirectX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectX",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::IndirectY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IndirectY",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeNoZeropageYNoImm::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeNoZeropageYNoImm {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeNoZeropageYNoImm::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(
                        AddrModeNoZeropageYNoImm::AbsoluteX(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(
                        AddrModeNoZeropageYNoImm::AbsoluteY(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x01) => {
                    Ok(
                        AddrModeNoZeropageYNoImm::IndirectX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x11) => {
                    Ok(
                        AddrModeNoZeropageYNoImm::IndirectY(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeNoZeropageYNoImm::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(
                        AddrModeNoZeropageYNoImm::ZeroPageX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeNoZeropageYNoImm`].
    pub struct AddrModeNoZeropageYNoImmConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeNoZeropageYNoImmConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeNoZeropageYNoImmConvertError {
        #[inline]
        fn clone(&self) -> AddrModeNoZeropageYNoImmConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeNoZeropageYNoImmConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeNoZeropageYNoImmConvertError")
        }
    }
    impl core::fmt::Display for AddrModeNoZeropageYNoImmConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeNoZeropageYNoImmConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeNoZeropageYNoImm> for AddressMode {
        fn from(child: AddrModeNoZeropageYNoImm) -> Self {
            match child {
                AddrModeNoZeropageYNoImm::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeNoZeropageYNoImm::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
                AddrModeNoZeropageYNoImm::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
                AddrModeNoZeropageYNoImm::IndirectX(var0) => AddressMode::IndirectX(var0),
                AddrModeNoZeropageYNoImm::IndirectY(var0) => AddressMode::IndirectY(var0),
                AddrModeNoZeropageYNoImm::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeNoZeropageYNoImm::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeNoZeropageYNoImm {
        type Error = AddrModeNoZeropageYNoImmConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::Absolute(var0))
                }
                AddressMode::AbsoluteX(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::AbsoluteX(var0))
                }
                AddressMode::AbsoluteY(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::AbsoluteY(var0))
                }
                AddressMode::IndirectX(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::IndirectX(var0))
                }
                AddressMode::IndirectY(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::IndirectY(var0))
                }
                AddressMode::ZeroPage(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::ZeroPage(var0))
                }
                AddressMode::ZeroPageX(var0) => {
                    Ok(AddrModeNoZeropageYNoImm::ZeroPageX(var0))
                }
                _ => Err(AddrModeNoZeropageYNoImmConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeRelative {
        /// Branch target is PC plus the signed offset byte.
        Relative(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeRelative {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeRelative::Relative(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Relative",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeRelative {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                b if (b & 0x1f == 0x10) => {
                    Ok(AddrModeRelative::Relative(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeRelative`].
    pub struct AddrModeRelativeConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeRelativeConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeRelativeConvertError {
        #[inline]
        fn clone(&self) -> AddrModeRelativeConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeRelativeConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeRelativeConvertError")
        }
    }
    impl core::fmt::Display for AddrModeRelativeConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeRelativeConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeRelative> for AddressMode {
        fn from(child: AddrModeRelative) -> Self {
            match child {
                AddrModeRelative::Relative(var0) => AddressMode::Relative(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeRelative {
        type Error = AddrModeRelativeConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Relative(var0) => Ok(AddrModeRelative::Relative(var0)),
                _ => Err(AddrModeRelativeConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSTX {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by Y without carry.
        ZeroPageY(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSTX {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSTX::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSTX::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSTX::ZeroPageY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSTX {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeSTX::Absolute(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddrModeSTX::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                0x97 | 0x97 | 0xb6 | 0xb7 => {
                    Ok(AddrModeSTX::ZeroPageY(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSTX`].
    pub struct AddrModeSTXConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSTXConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSTXConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSTXConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSTXConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSTXConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSTXConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSTXConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSTX> for AddressMode {
        fn from(child: AddrModeSTX) -> Self {
            match child {
                AddrModeSTX::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSTX::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSTX::ZeroPageY(var0) => AddressMode::ZeroPageY(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSTX {
        type Error = AddrModeSTXConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSTX::Absolute(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSTX::ZeroPage(var0)),
                AddressMode::ZeroPageY(var0) => Ok(AddrModeSTX::ZeroPageY(var0)),
                _ => Err(AddrModeSTXConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSTY {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSTY {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSTY::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSTY::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSTY::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSTY {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeSTY::Absolute(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddrModeSTY::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(AddrModeSTY::ZeroPageX(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSTY`].
    pub struct AddrModeSTYConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSTYConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSTYConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSTYConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSTYConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSTYConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSTYConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSTYConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSTY> for AddressMode {
        fn from(child: AddrModeSTY) -> Self {
            match child {
                AddrModeSTY::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSTY::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSTY::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSTY {
        type Error = AddrModeSTYConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSTY::Absolute(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSTY::ZeroPage(var0)),
                AddressMode::ZeroPageX(var0) => Ok(AddrModeSTY::ZeroPageX(var0)),
                _ => Err(AddrModeSTYConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimple {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimple {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimple::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimple::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimple {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeSimple::Absolute(u16::read_options(reader, endian, ())?))
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddrModeSimple::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimple`].
    pub struct AddrModeSimpleConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimple> for AddressMode {
        fn from(child: AddrModeSimple) -> Self {
            match child {
                AddrModeSimple::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimple::ZeroPage(var0) => AddressMode::ZeroPage(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimple {
        type Error = AddrModeSimpleConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSimple::Absolute(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimple::ZeroPage(var0)),
                _ => Err(AddrModeSimpleConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimpleOrImm {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Immediate (8-bit) value.
        Immediate(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleOrImm {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimpleOrImm::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimpleOrImm::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
                AddrModeSimpleOrImm::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimpleOrImm {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeSimpleOrImm::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(
                        AddrModeSimpleOrImm::Immediate(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeSimpleOrImm::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimpleOrImm`].
    pub struct AddrModeSimpleOrImmConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleOrImmConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleOrImmConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleOrImmConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleOrImmConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleOrImmConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleOrImmConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleOrImmConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimpleOrImm> for AddressMode {
        fn from(child: AddrModeSimpleOrImm) -> Self {
            match child {
                AddrModeSimpleOrImm::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimpleOrImm::Immediate(var0) => AddressMode::Immediate(var0),
                AddrModeSimpleOrImm::ZeroPage(var0) => AddressMode::ZeroPage(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimpleOrImm {
        type Error = AddrModeSimpleOrImmConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSimpleOrImm::Absolute(var0)),
                AddressMode::Immediate(var0) => Ok(AddrModeSimpleOrImm::Immediate(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimpleOrImm::ZeroPage(var0)),
                _ => Err(AddrModeSimpleOrImmConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimpleX {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleX {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimpleX::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimpleX::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddrModeSimpleX::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSimpleX::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimpleX {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(AddrModeSimpleX::Absolute(u16::read_options(reader, endian, ())?))
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(
                        AddrModeSimpleX::AbsoluteX(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(AddrModeSimpleX::ZeroPage(u8::read_options(reader, endian, ())?))
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(AddrModeSimpleX::ZeroPageX(u8::read_options(reader, endian, ())?))
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimpleX`].
    pub struct AddrModeSimpleXConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleXConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleXConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleXConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleXConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleXConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleXConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleXConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimpleX> for AddressMode {
        fn from(child: AddrModeSimpleX) -> Self {
            match child {
                AddrModeSimpleX::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimpleX::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
                AddrModeSimpleX::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSimpleX::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimpleX {
        type Error = AddrModeSimpleXConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSimpleX::Absolute(var0)),
                AddressMode::AbsoluteX(var0) => Ok(AddrModeSimpleX::AbsoluteX(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimpleX::ZeroPage(var0)),
                AddressMode::ZeroPageX(var0) => Ok(AddrModeSimpleX::ZeroPageX(var0)),
                _ => Err(AddrModeSimpleXConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimpleXAcc {
        /// Operand is implied Accumulator register.
        Accumulator,
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleXAcc {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimpleXAcc::Accumulator => {
                    ::core::fmt::Formatter::write_str(f, "Accumulator")
                }
                AddrModeSimpleXAcc::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimpleXAcc::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddrModeSimpleXAcc::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSimpleXAcc::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimpleXAcc {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x0a | 0x2a | 0x4a | 0x6a => Ok(AddrModeSimpleXAcc::Accumulator),
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeSimpleXAcc::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(
                        AddrModeSimpleXAcc::AbsoluteX(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeSimpleXAcc::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(
                        AddrModeSimpleXAcc::ZeroPageX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimpleXAcc`].
    pub struct AddrModeSimpleXAccConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleXAccConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleXAccConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleXAccConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleXAccConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleXAccConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleXAccConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleXAccConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimpleXAcc> for AddressMode {
        fn from(child: AddrModeSimpleXAcc) -> Self {
            match child {
                AddrModeSimpleXAcc::Accumulator => AddressMode::Accumulator,
                AddrModeSimpleXAcc::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimpleXAcc::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
                AddrModeSimpleXAcc::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSimpleXAcc::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimpleXAcc {
        type Error = AddrModeSimpleXAccConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Accumulator => Ok(AddrModeSimpleXAcc::Accumulator),
                AddressMode::Absolute(var0) => Ok(AddrModeSimpleXAcc::Absolute(var0)),
                AddressMode::AbsoluteX(var0) => Ok(AddrModeSimpleXAcc::AbsoluteX(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimpleXAcc::ZeroPage(var0)),
                AddressMode::ZeroPageX(var0) => Ok(AddrModeSimpleXAcc::ZeroPageX(var0)),
                _ => Err(AddrModeSimpleXAccConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimpleXImm {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by X with carry.
        AbsoluteX(u16),
        /// Immediate (8-bit) value.
        Immediate(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by X without carry.
        ZeroPageX(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleXImm {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimpleXImm::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimpleXImm::AbsoluteX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteX",
                        &__self_0,
                    )
                }
                AddrModeSimpleXImm::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
                AddrModeSimpleXImm::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSimpleXImm::ZeroPageX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageX",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimpleXImm {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeSimpleXImm::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x1c..=0x1f
                | 0x3c..=0x3f
                | 0x5c..=0x5f
                | 0x7d..=0x7f
                | 0x9c
                | 0x9d
                | 0xbc
                | 0xbd
                | 0xdc..=0xdf
                | 0xfc..=0xff => {
                    Ok(
                        AddrModeSimpleXImm::AbsoluteX(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(
                        AddrModeSimpleXImm::Immediate(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeSimpleXImm::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x14..=0x17
                | 0x34..=0x37
                | 0x54..=0x57
                | 0x74..=0x77
                | 0x94
                | 0x95
                | 0xb4
                | 0xb5
                | 0xd4..=0xd7
                | 0xf4..=0xf7 => {
                    Ok(
                        AddrModeSimpleXImm::ZeroPageX(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimpleXImm`].
    pub struct AddrModeSimpleXImmConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleXImmConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleXImmConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleXImmConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleXImmConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleXImmConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleXImmConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleXImmConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimpleXImm> for AddressMode {
        fn from(child: AddrModeSimpleXImm) -> Self {
            match child {
                AddrModeSimpleXImm::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimpleXImm::AbsoluteX(var0) => AddressMode::AbsoluteX(var0),
                AddrModeSimpleXImm::Immediate(var0) => AddressMode::Immediate(var0),
                AddrModeSimpleXImm::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSimpleXImm::ZeroPageX(var0) => AddressMode::ZeroPageX(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimpleXImm {
        type Error = AddrModeSimpleXImmConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSimpleXImm::Absolute(var0)),
                AddressMode::AbsoluteX(var0) => Ok(AddrModeSimpleXImm::AbsoluteX(var0)),
                AddressMode::Immediate(var0) => Ok(AddrModeSimpleXImm::Immediate(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimpleXImm::ZeroPage(var0)),
                AddressMode::ZeroPageX(var0) => Ok(AddrModeSimpleXImm::ZeroPageX(var0)),
                _ => Err(AddrModeSimpleXImmConvertError),
            }
        }
    }
    /// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
    pub enum AddrModeSimpleYImm {
        /// Absolute (16-bit) address.
        Absolute(u16),
        /// Absolute (16-bit) address, incremented by Y with carry.
        AbsoluteY(u16),
        /// Immediate (8-bit) value.
        Immediate(u8),
        /// Zeropage (8-bit) address.
        ZeroPage(u8),
        /// Zeropage (8-bit) address, incremented by Y without carry.
        ZeroPageY(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleYImm {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AddrModeSimpleYImm::Absolute(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Absolute",
                        &__self_0,
                    )
                }
                AddrModeSimpleYImm::AbsoluteY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AbsoluteY",
                        &__self_0,
                    )
                }
                AddrModeSimpleYImm::Immediate(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Immediate",
                        &__self_0,
                    )
                }
                AddrModeSimpleYImm::ZeroPage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPage",
                        &__self_0,
                    )
                }
                AddrModeSimpleYImm::ZeroPageY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ZeroPageY",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl BinRead for AddrModeSimpleYImm {
        type Args<'a> = u8;
        /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
        fn read_options<R: io::Read + io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            match args {
                0x20
                | 0x0c..=0x0f
                | 0x2c..=0x2f
                | 0x4c..=0x4f
                | 0x6d..=0x6f
                | 0x8c..=0x8f
                | 0xac..=0xaf
                | 0xcc..=0xcf
                | 0xec..=0xef => {
                    Ok(
                        AddrModeSimpleYImm::Absolute(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) => {
                    Ok(
                        AddrModeSimpleYImm::AbsoluteY(
                            u16::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) => {
                    Ok(
                        AddrModeSimpleYImm::Immediate(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                b if (b & 0x1c == 0x04) => {
                    Ok(
                        AddrModeSimpleYImm::ZeroPage(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                0x97 | 0x97 | 0xb6 | 0xb7 => {
                    Ok(
                        AddrModeSimpleYImm::ZeroPageY(
                            u8::read_options(reader, endian, ())?,
                        ),
                    )
                }
                _ => {
                    Err(binrw::Error::AssertFail {
                        pos: reader.stream_position()?,
                        message: {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unexpected byte for address mode: {0:02x}",
                                    args,
                                ),
                            );
                            res
                        },
                    })
                }
            }
        }
    }
    ///An error type used for converting from [`AddressMode`] to [`AddrModeSimpleYImm`].
    pub struct AddrModeSimpleYImmConvertError;
    #[automatically_derived]
    impl ::core::marker::Copy for AddrModeSimpleYImmConvertError {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddrModeSimpleYImmConvertError {
        #[inline]
        fn clone(&self) -> AddrModeSimpleYImmConvertError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddrModeSimpleYImmConvertError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AddrModeSimpleYImmConvertError")
        }
    }
    impl core::fmt::Display for AddrModeSimpleYImmConvertError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Debug::fmt(self, f)
        }
    }
    impl std::error::Error for AddrModeSimpleYImmConvertError {}
    #[automatically_derived]
    impl core::convert::From<AddrModeSimpleYImm> for AddressMode {
        fn from(child: AddrModeSimpleYImm) -> Self {
            match child {
                AddrModeSimpleYImm::Absolute(var0) => AddressMode::Absolute(var0),
                AddrModeSimpleYImm::AbsoluteY(var0) => AddressMode::AbsoluteY(var0),
                AddrModeSimpleYImm::Immediate(var0) => AddressMode::Immediate(var0),
                AddrModeSimpleYImm::ZeroPage(var0) => AddressMode::ZeroPage(var0),
                AddrModeSimpleYImm::ZeroPageY(var0) => AddressMode::ZeroPageY(var0),
            }
        }
    }
    #[automatically_derived]
    impl core::convert::TryFrom<AddressMode> for AddrModeSimpleYImm {
        type Error = AddrModeSimpleYImmConvertError;
        fn try_from(
            parent: AddressMode,
        ) -> Result<Self, <Self as core::convert::TryFrom<AddressMode>>::Error> {
            match parent {
                AddressMode::Absolute(var0) => Ok(AddrModeSimpleYImm::Absolute(var0)),
                AddressMode::AbsoluteY(var0) => Ok(AddrModeSimpleYImm::AbsoluteY(var0)),
                AddressMode::Immediate(var0) => Ok(AddrModeSimpleYImm::Immediate(var0)),
                AddressMode::ZeroPage(var0) => Ok(AddrModeSimpleYImm::ZeroPage(var0)),
                AddressMode::ZeroPageY(var0) => Ok(AddrModeSimpleYImm::ZeroPageY(var0)),
                _ => Err(AddrModeSimpleYImmConvertError),
            }
        }
    }
    impl AddressMode {
        pub fn size(&self) -> usize {
            match self {
                &AddressMode::Accumulator | &AddressMode::Implied => 0,
                &AddressMode::Immediate(_)
                | &AddressMode::IndirectX(_)
                | &AddressMode::IndirectY(_)
                | &AddressMode::Relative(_)
                | &AddressMode::ZeroPage(_)
                | &AddressMode::ZeroPageX(_)
                | &AddressMode::ZeroPageY(_) => 1,
                &AddressMode::Absolute(_)
                | &AddressMode::AbsoluteX(_)
                | &AddressMode::AbsoluteY(_)
                | &AddressMode::Indirect(_) => 2,
            }
        }
    }
    impl std::fmt::Display for AddressMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                &AddressMode::Accumulator => f.write_fmt(format_args!("A")),
                &AddressMode::Absolute(addr) => {
                    f.write_fmt(format_args!("${0:04x}", addr))
                }
                &AddressMode::AbsoluteX(addr) => {
                    f.write_fmt(format_args!("${0:04x},X", addr))
                }
                &AddressMode::AbsoluteY(addr) => {
                    f.write_fmt(format_args!("${0:04x},Y", addr))
                }
                &AddressMode::Immediate(addr) => {
                    f.write_fmt(format_args!("#${0:02x}", addr))
                }
                &AddressMode::Implied => Ok(()),
                &AddressMode::Indirect(addr) => {
                    f.write_fmt(format_args!("(${0:04x})", addr))
                }
                &AddressMode::IndirectX(addr) => {
                    f.write_fmt(format_args!("(${0:02x},X)", addr))
                }
                &AddressMode::IndirectY(addr) => {
                    f.write_fmt(format_args!("(${0:02x}),Y", addr))
                }
                &AddressMode::Relative(addr) => {
                    f.write_fmt(format_args!("${0:02x}", addr))
                }
                &AddressMode::ZeroPage(addr) => {
                    f.write_fmt(format_args!("${0:02x}", addr))
                }
                &AddressMode::ZeroPageX(addr) => {
                    f.write_fmt(format_args!("${0:02x},X", addr))
                }
                &AddressMode::ZeroPageY(addr) => {
                    f.write_fmt(format_args!("${0:02x},Y", addr))
                }
            }
        }
    }
    pub enum Opcode {
        /// add with carry
        ADC(AddrModeNoZeropageY),
        /// and (with accumulator)
        AND(AddrModeNoZeropageY),
        /// arithmetic shift left
        ASL(AddrModeSimpleXAcc),
        /// branch on carry clear
        BCC(AddrModeRelative),
        /// branch on carry set
        BCS(AddrModeRelative),
        /// branch on equal (zero set)
        BEQ(AddrModeRelative),
        /// bit test
        BIT(AddrModeSimple),
        /// branch on minus (negative set)
        BMI(AddrModeRelative),
        /// branch on not equal (zero clear)
        BNE(AddrModeRelative),
        /// branch on plus (negative clear)
        BPL(AddrModeRelative),
        /// break / interrupt
        BRK,
        /// branch on overflow clear
        BVC(AddrModeRelative),
        /// branch on overflow set
        BVS(AddrModeRelative),
        /// clear carry
        CLC,
        /// clear decimal
        CLD,
        /// clear interrupt disable
        CLI,
        /// clear overflow
        CLV,
        /// compare (with accumulator)
        CMP(AddrModeNoZeropageY),
        /// compare with X
        CPX(AddrModeSimpleOrImm),
        /// compare with Y
        CPY(AddrModeSimpleOrImm),
        /// decrement
        DEC(AddrModeSimpleX),
        /// decrement X
        DEX,
        /// decrement Y
        DEY,
        /// exclusive or (with accumulator)
        EOR(AddrModeNoZeropageY),
        /// increment
        INC(AddrModeSimpleX),
        /// increment X
        INX,
        /// increment Y
        INY,
        /// jump
        JMP(AddrModeAbsInd),
        /// jump subroutine
        JSR(AddrModeAbs),
        /// load accumulator
        LDA(AddrModeNoZeropageY),
        /// load X
        LDX(AddrModeSimpleYImm),
        /// load Y
        LDY(AddrModeSimpleXImm),
        /// logical shift right
        LSR(AddrModeSimpleXAcc),
        /// no operation
        NOP,
        /// or with accumulator
        ORA(AddrModeNoZeropageY),
        /// push accumulator
        PHA,
        /// push processor status (SR)
        PHP,
        /// pull accumulator
        PLA,
        /// pull processor status (SR)
        PLP,
        /// rotate left
        ROL(AddrModeSimpleXAcc),
        /// rotate right
        ROR(AddrModeSimpleXAcc),
        /// return from interrupt
        RTI,
        /// return from subroutine
        RTS,
        /// subtract with carry
        SBC(AddrModeNoZeropageY),
        /// set carry
        SEC,
        /// set decimal
        SED,
        /// set interrupt disable
        SEI,
        /// store accumulator
        STA(AddrModeNoZeropageYNoImm),
        /// store X
        STX(AddrModeSTX),
        /// store Y
        STY(AddrModeSTY),
        /// transfer accumulator to X
        TAX,
        /// transfer accumulator to Y
        TAY,
        /// transfer stack pointer to X
        TSX,
        /// transfer X to accumulator
        TXA,
        /// transfer X to stack pointer
        TXS,
        /// transfer Y to accumulator
        TYA,
        /// a.k.a SHA or AXA
        ///
        /// Stores A AND X AND (high-byte of addr. + 1) at addr.
        AHX(AddrModeAHX),
        /// ALR = AND + LSR
        ALR(AddrModeImmediate),
        /// ANC = AND, bit(7) -> Carry
        ANC(AddrModeImmediate),
        /// ARR = AND + ROR
        ARR(AddrModeImmediate),
        /// a.k.a SBX or SAX
        ///
        /// CMP and DEX at once, sets flags like CMP
        AXS(AddrModeImmediate),
        /// DCP = DEC + CMP
        DCP(AddrModeNoZeropageYNoImm),
        /// ISC = INC + SBC
        ISC(AddrModeNoZeropageYNoImm),
        /// LSA/TSX oper
        ///
        /// M AND SP -> A, X, SP
        LAS(AddrModeAbsY),
        /// LAZ = LDA + LDX
        LAX(AddrModeLAX),
        /// RLA = ROL + AND
        RLA(AddrModeNoZeropageYNoImm),
        /// RRA = ROR + ADC
        RRA(AddrModeNoZeropageYNoImm),
        /// a.k.a. SBX, AXS
        ///
        /// (A AND X) - oper -> X
        SAX(AddrModeImmediate),
        /// a.k.a A11, SXA, XAS
        ///
        /// Stores X AND (high-byte of addr. + 1) at addr.
        SHX(AddrModeAbsY),
        /// a.k.a A11, SYA, SAY
        ///
        /// Stores Y AND (high-byte of addr. + 1) at addr.
        SHY(AddrModeAbsX),
        /// SLO = ASL + ORA
        SLO(AddrModeNoZeropageYNoImm),
        /// SRE = LSR + EOR
        SRE(AddrModeNoZeropageYNoImm),
        /// Puts A AND X in SP and stores A AND X AND (high-byte of addr. + 1) at addr.
        TAS(AddrModeAbsY),
        /// a.k.a ANE
        ///
        /// `(A OR CONST) AND X AND oper -> A`
        XAA(AddrModeImmediate),
        /// NES Stop (?)
        STP,
    }
    impl BinRead for Opcode {
        type Args<'a> = ();
        fn read_options<R: std::io::Read + std::io::Seek>(
            reader: &mut R,
            endian: binrw::Endian,
            args: Self::Args<'_>,
        ) -> binrw::BinResult<Self> {
            let byte = u8::read_options(reader, endian, ())?;
            match byte {
                0u8 => Ok(Opcode::BRK),
                1u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                2u8 => Ok(Opcode::STP),
                3u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                4u8 => Ok(Opcode::NOP),
                5u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                6u8 => {
                    Ok(
                        Opcode::ASL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                7u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                8u8 => Ok(Opcode::PHP),
                9u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                10u8 => {
                    Ok(
                        Opcode::ASL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                11u8 => {
                    Ok(
                        Opcode::ANC(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                12u8 => Ok(Opcode::NOP),
                13u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                14u8 => {
                    Ok(
                        Opcode::ASL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                15u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                16u8 => {
                    Ok(
                        Opcode::BPL(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                17u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                18u8 => Ok(Opcode::STP),
                19u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                20u8 => Ok(Opcode::NOP),
                21u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                22u8 => {
                    Ok(
                        Opcode::ASL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                23u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                24u8 => Ok(Opcode::CLC),
                25u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                26u8 => Ok(Opcode::NOP),
                27u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                28u8 => Ok(Opcode::NOP),
                29u8 => {
                    Ok(
                        Opcode::ORA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                30u8 => {
                    Ok(
                        Opcode::ASL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                31u8 => {
                    Ok(
                        Opcode::SLO(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                32u8 => Ok(Opcode::JSR(AddrModeAbs::read_options(reader, endian, byte)?)),
                33u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                34u8 => Ok(Opcode::STP),
                35u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                36u8 => {
                    Ok(Opcode::BIT(AddrModeSimple::read_options(reader, endian, byte)?))
                }
                37u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                38u8 => {
                    Ok(
                        Opcode::ROL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                39u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                40u8 => Ok(Opcode::PLP),
                41u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                42u8 => {
                    Ok(
                        Opcode::ROL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                43u8 => {
                    Ok(
                        Opcode::ANC(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                44u8 => {
                    Ok(Opcode::BIT(AddrModeSimple::read_options(reader, endian, byte)?))
                }
                45u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                46u8 => {
                    Ok(
                        Opcode::ROL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                47u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                48u8 => {
                    Ok(
                        Opcode::BMI(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                49u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                50u8 => Ok(Opcode::STP),
                51u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                52u8 => Ok(Opcode::NOP),
                53u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                54u8 => {
                    Ok(
                        Opcode::ROL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                55u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                56u8 => Ok(Opcode::SEC),
                57u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                58u8 => Ok(Opcode::NOP),
                59u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                60u8 => Ok(Opcode::NOP),
                61u8 => {
                    Ok(
                        Opcode::AND(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                62u8 => {
                    Ok(
                        Opcode::ROL(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                63u8 => {
                    Ok(
                        Opcode::RLA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                64u8 => Ok(Opcode::RTI),
                65u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                66u8 => Ok(Opcode::STP),
                67u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                68u8 => Ok(Opcode::NOP),
                69u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                70u8 => {
                    Ok(
                        Opcode::LSR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                71u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                72u8 => Ok(Opcode::PHA),
                73u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                74u8 => {
                    Ok(
                        Opcode::LSR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                75u8 => {
                    Ok(
                        Opcode::ALR(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                76u8 => {
                    Ok(Opcode::JMP(AddrModeAbsInd::read_options(reader, endian, byte)?))
                }
                77u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                78u8 => {
                    Ok(
                        Opcode::LSR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                79u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                80u8 => {
                    Ok(
                        Opcode::BVC(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                81u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                82u8 => Ok(Opcode::STP),
                83u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                84u8 => Ok(Opcode::NOP),
                85u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                86u8 => {
                    Ok(
                        Opcode::LSR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                87u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                88u8 => Ok(Opcode::CLI),
                89u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                90u8 => Ok(Opcode::NOP),
                91u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                92u8 => Ok(Opcode::NOP),
                93u8 => {
                    Ok(
                        Opcode::EOR(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                94u8 => {
                    Ok(
                        Opcode::LSR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                95u8 => {
                    Ok(
                        Opcode::SRE(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                96u8 => Ok(Opcode::RTS),
                97u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                98u8 => Ok(Opcode::STP),
                99u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                100u8 => Ok(Opcode::NOP),
                101u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                102u8 => {
                    Ok(
                        Opcode::ROR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                103u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                104u8 => Ok(Opcode::PLA),
                105u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                106u8 => {
                    Ok(
                        Opcode::ROR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                107u8 => {
                    Ok(
                        Opcode::ARR(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                108u8 => {
                    Ok(Opcode::JMP(AddrModeAbsInd::read_options(reader, endian, byte)?))
                }
                109u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                110u8 => {
                    Ok(
                        Opcode::ROR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                111u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                112u8 => {
                    Ok(
                        Opcode::BVS(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                113u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                114u8 => Ok(Opcode::STP),
                115u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                116u8 => Ok(Opcode::NOP),
                117u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                118u8 => {
                    Ok(
                        Opcode::ROR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                119u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                120u8 => Ok(Opcode::SEI),
                121u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                122u8 => Ok(Opcode::NOP),
                123u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                124u8 => Ok(Opcode::NOP),
                125u8 => {
                    Ok(
                        Opcode::ADC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                126u8 => {
                    Ok(
                        Opcode::ROR(
                            AddrModeSimpleXAcc::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                127u8 => {
                    Ok(
                        Opcode::RRA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                128u8 => Ok(Opcode::NOP),
                129u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                130u8 => Ok(Opcode::NOP),
                131u8 => {
                    Ok(
                        Opcode::SAX(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                132u8 => {
                    Ok(Opcode::STY(AddrModeSTY::read_options(reader, endian, byte)?))
                }
                133u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                134u8 => {
                    Ok(Opcode::STX(AddrModeSTX::read_options(reader, endian, byte)?))
                }
                135u8 => {
                    Ok(
                        Opcode::SAX(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                136u8 => Ok(Opcode::DEY),
                137u8 => Ok(Opcode::NOP),
                138u8 => Ok(Opcode::TXA),
                139u8 => {
                    Ok(
                        Opcode::XAA(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                140u8 => {
                    Ok(Opcode::STY(AddrModeSTY::read_options(reader, endian, byte)?))
                }
                141u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                142u8 => {
                    Ok(Opcode::STX(AddrModeSTX::read_options(reader, endian, byte)?))
                }
                143u8 => {
                    Ok(
                        Opcode::SAX(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                144u8 => {
                    Ok(
                        Opcode::BCC(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                145u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                146u8 => Ok(Opcode::STP),
                147u8 => {
                    Ok(Opcode::AHX(AddrModeAHX::read_options(reader, endian, byte)?))
                }
                148u8 => {
                    Ok(Opcode::STY(AddrModeSTY::read_options(reader, endian, byte)?))
                }
                149u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                150u8 => {
                    Ok(Opcode::STX(AddrModeSTX::read_options(reader, endian, byte)?))
                }
                151u8 => {
                    Ok(
                        Opcode::SAX(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                152u8 => Ok(Opcode::TYA),
                153u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                154u8 => Ok(Opcode::TXS),
                155u8 => {
                    Ok(Opcode::TAS(AddrModeAbsY::read_options(reader, endian, byte)?))
                }
                156u8 => {
                    Ok(Opcode::SHY(AddrModeAbsX::read_options(reader, endian, byte)?))
                }
                157u8 => {
                    Ok(
                        Opcode::STA(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                158u8 => {
                    Ok(Opcode::SHX(AddrModeAbsY::read_options(reader, endian, byte)?))
                }
                159u8 => {
                    Ok(Opcode::AHX(AddrModeAHX::read_options(reader, endian, byte)?))
                }
                160u8 => {
                    Ok(
                        Opcode::LDY(
                            AddrModeSimpleXImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                161u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                162u8 => {
                    Ok(
                        Opcode::LDX(
                            AddrModeSimpleYImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                163u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                164u8 => {
                    Ok(
                        Opcode::LDY(
                            AddrModeSimpleXImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                165u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                166u8 => {
                    Ok(
                        Opcode::LDX(
                            AddrModeSimpleYImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                167u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                168u8 => Ok(Opcode::TAY),
                169u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                170u8 => Ok(Opcode::TAX),
                171u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                172u8 => {
                    Ok(
                        Opcode::LDY(
                            AddrModeSimpleXImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                173u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                174u8 => {
                    Ok(
                        Opcode::LDX(
                            AddrModeSimpleYImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                175u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                176u8 => {
                    Ok(
                        Opcode::BCS(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                177u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                178u8 => Ok(Opcode::STP),
                179u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                180u8 => {
                    Ok(
                        Opcode::LDY(
                            AddrModeSimpleXImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                181u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                182u8 => {
                    Ok(
                        Opcode::LDX(
                            AddrModeSimpleYImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                183u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                184u8 => Ok(Opcode::CLV),
                185u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                186u8 => Ok(Opcode::TSX),
                187u8 => {
                    Ok(Opcode::LAS(AddrModeAbsY::read_options(reader, endian, byte)?))
                }
                188u8 => {
                    Ok(
                        Opcode::LDY(
                            AddrModeSimpleXImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                189u8 => {
                    Ok(
                        Opcode::LDA(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                190u8 => {
                    Ok(
                        Opcode::LDX(
                            AddrModeSimpleYImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                191u8 => {
                    Ok(Opcode::LAX(AddrModeLAX::read_options(reader, endian, byte)?))
                }
                192u8 => {
                    Ok(
                        Opcode::CPY(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                193u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                194u8 => Ok(Opcode::NOP),
                195u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                196u8 => {
                    Ok(
                        Opcode::CPY(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                197u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                198u8 => {
                    Ok(Opcode::DEC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                199u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                200u8 => Ok(Opcode::INY),
                201u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                202u8 => Ok(Opcode::DEX),
                203u8 => {
                    Ok(
                        Opcode::AXS(
                            AddrModeImmediate::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                204u8 => {
                    Ok(
                        Opcode::CPY(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                205u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                206u8 => {
                    Ok(Opcode::DEC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                207u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                208u8 => {
                    Ok(
                        Opcode::BNE(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                209u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                210u8 => Ok(Opcode::STP),
                211u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                212u8 => Ok(Opcode::NOP),
                213u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                214u8 => {
                    Ok(Opcode::DEC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                215u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                216u8 => Ok(Opcode::CLD),
                217u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                218u8 => Ok(Opcode::NOP),
                219u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                220u8 => Ok(Opcode::NOP),
                221u8 => {
                    Ok(
                        Opcode::CMP(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                222u8 => {
                    Ok(Opcode::DEC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                223u8 => {
                    Ok(
                        Opcode::DCP(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                224u8 => {
                    Ok(
                        Opcode::CPX(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                225u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                226u8 => Ok(Opcode::NOP),
                227u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                228u8 => {
                    Ok(
                        Opcode::CPX(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                229u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                230u8 => {
                    Ok(Opcode::INC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                231u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                232u8 => Ok(Opcode::INX),
                233u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                234u8 => Ok(Opcode::NOP),
                235u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                236u8 => {
                    Ok(
                        Opcode::CPX(
                            AddrModeSimpleOrImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                237u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                238u8 => {
                    Ok(Opcode::INC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                239u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                240u8 => {
                    Ok(
                        Opcode::BEQ(
                            AddrModeRelative::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                241u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                242u8 => Ok(Opcode::STP),
                243u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                244u8 => Ok(Opcode::NOP),
                245u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                246u8 => {
                    Ok(Opcode::INC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                247u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                248u8 => Ok(Opcode::SED),
                249u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                250u8 => Ok(Opcode::NOP),
                251u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                252u8 => Ok(Opcode::NOP),
                253u8 => {
                    Ok(
                        Opcode::SBC(
                            AddrModeNoZeropageY::read_options(reader, endian, byte)?,
                        ),
                    )
                }
                254u8 => {
                    Ok(Opcode::INC(AddrModeSimpleX::read_options(reader, endian, byte)?))
                }
                255u8 => {
                    Ok(
                        Opcode::ISC(
                            AddrModeNoZeropageYNoImm::read_options(reader, endian, byte)?,
                        ),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Opcode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Opcode::ADC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ADC",
                        &__self_0,
                    )
                }
                Opcode::AND(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AND",
                        &__self_0,
                    )
                }
                Opcode::ASL(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ASL",
                        &__self_0,
                    )
                }
                Opcode::BCC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BCC",
                        &__self_0,
                    )
                }
                Opcode::BCS(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BCS",
                        &__self_0,
                    )
                }
                Opcode::BEQ(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BEQ",
                        &__self_0,
                    )
                }
                Opcode::BIT(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BIT",
                        &__self_0,
                    )
                }
                Opcode::BMI(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BMI",
                        &__self_0,
                    )
                }
                Opcode::BNE(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BNE",
                        &__self_0,
                    )
                }
                Opcode::BPL(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BPL",
                        &__self_0,
                    )
                }
                Opcode::BRK => ::core::fmt::Formatter::write_str(f, "BRK"),
                Opcode::BVC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BVC",
                        &__self_0,
                    )
                }
                Opcode::BVS(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BVS",
                        &__self_0,
                    )
                }
                Opcode::CLC => ::core::fmt::Formatter::write_str(f, "CLC"),
                Opcode::CLD => ::core::fmt::Formatter::write_str(f, "CLD"),
                Opcode::CLI => ::core::fmt::Formatter::write_str(f, "CLI"),
                Opcode::CLV => ::core::fmt::Formatter::write_str(f, "CLV"),
                Opcode::CMP(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CMP",
                        &__self_0,
                    )
                }
                Opcode::CPX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CPX",
                        &__self_0,
                    )
                }
                Opcode::CPY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CPY",
                        &__self_0,
                    )
                }
                Opcode::DEC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DEC",
                        &__self_0,
                    )
                }
                Opcode::DEX => ::core::fmt::Formatter::write_str(f, "DEX"),
                Opcode::DEY => ::core::fmt::Formatter::write_str(f, "DEY"),
                Opcode::EOR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EOR",
                        &__self_0,
                    )
                }
                Opcode::INC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "INC",
                        &__self_0,
                    )
                }
                Opcode::INX => ::core::fmt::Formatter::write_str(f, "INX"),
                Opcode::INY => ::core::fmt::Formatter::write_str(f, "INY"),
                Opcode::JMP(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JMP",
                        &__self_0,
                    )
                }
                Opcode::JSR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "JSR",
                        &__self_0,
                    )
                }
                Opcode::LDA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LDA",
                        &__self_0,
                    )
                }
                Opcode::LDX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LDX",
                        &__self_0,
                    )
                }
                Opcode::LDY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LDY",
                        &__self_0,
                    )
                }
                Opcode::LSR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LSR",
                        &__self_0,
                    )
                }
                Opcode::NOP => ::core::fmt::Formatter::write_str(f, "NOP"),
                Opcode::ORA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ORA",
                        &__self_0,
                    )
                }
                Opcode::PHA => ::core::fmt::Formatter::write_str(f, "PHA"),
                Opcode::PHP => ::core::fmt::Formatter::write_str(f, "PHP"),
                Opcode::PLA => ::core::fmt::Formatter::write_str(f, "PLA"),
                Opcode::PLP => ::core::fmt::Formatter::write_str(f, "PLP"),
                Opcode::ROL(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ROL",
                        &__self_0,
                    )
                }
                Opcode::ROR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ROR",
                        &__self_0,
                    )
                }
                Opcode::RTI => ::core::fmt::Formatter::write_str(f, "RTI"),
                Opcode::RTS => ::core::fmt::Formatter::write_str(f, "RTS"),
                Opcode::SBC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SBC",
                        &__self_0,
                    )
                }
                Opcode::SEC => ::core::fmt::Formatter::write_str(f, "SEC"),
                Opcode::SED => ::core::fmt::Formatter::write_str(f, "SED"),
                Opcode::SEI => ::core::fmt::Formatter::write_str(f, "SEI"),
                Opcode::STA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "STA",
                        &__self_0,
                    )
                }
                Opcode::STX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "STX",
                        &__self_0,
                    )
                }
                Opcode::STY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "STY",
                        &__self_0,
                    )
                }
                Opcode::TAX => ::core::fmt::Formatter::write_str(f, "TAX"),
                Opcode::TAY => ::core::fmt::Formatter::write_str(f, "TAY"),
                Opcode::TSX => ::core::fmt::Formatter::write_str(f, "TSX"),
                Opcode::TXA => ::core::fmt::Formatter::write_str(f, "TXA"),
                Opcode::TXS => ::core::fmt::Formatter::write_str(f, "TXS"),
                Opcode::TYA => ::core::fmt::Formatter::write_str(f, "TYA"),
                Opcode::AHX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AHX",
                        &__self_0,
                    )
                }
                Opcode::ALR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ALR",
                        &__self_0,
                    )
                }
                Opcode::ANC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ANC",
                        &__self_0,
                    )
                }
                Opcode::ARR(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ARR",
                        &__self_0,
                    )
                }
                Opcode::AXS(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AXS",
                        &__self_0,
                    )
                }
                Opcode::DCP(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DCP",
                        &__self_0,
                    )
                }
                Opcode::ISC(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ISC",
                        &__self_0,
                    )
                }
                Opcode::LAS(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LAS",
                        &__self_0,
                    )
                }
                Opcode::LAX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LAX",
                        &__self_0,
                    )
                }
                Opcode::RLA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RLA",
                        &__self_0,
                    )
                }
                Opcode::RRA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RRA",
                        &__self_0,
                    )
                }
                Opcode::SAX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SAX",
                        &__self_0,
                    )
                }
                Opcode::SHX(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SHX",
                        &__self_0,
                    )
                }
                Opcode::SHY(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SHY",
                        &__self_0,
                    )
                }
                Opcode::SLO(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SLO",
                        &__self_0,
                    )
                }
                Opcode::SRE(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SRE",
                        &__self_0,
                    )
                }
                Opcode::TAS(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TAS",
                        &__self_0,
                    )
                }
                Opcode::XAA(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "XAA",
                        &__self_0,
                    )
                }
                Opcode::STP => ::core::fmt::Formatter::write_str(f, "STP"),
            }
        }
    }
}
