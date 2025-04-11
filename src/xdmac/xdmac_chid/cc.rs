#[doc = "Register `CC` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Channel x Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typeselect {
    #[doc = "0: Self-triggered mode (memory-to-memory transfer)."]
    MemTran = 0,
    #[doc = "1: Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PerTran = 1,
}
impl From<Typeselect> for bool {
    #[inline(always)]
    fn from(variant: Typeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - Channel x Transfer Type"]
pub type TypeR = crate::BitReader<Typeselect>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Typeselect {
        match self.bits {
            false => Typeselect::MemTran,
            true => Typeselect::PerTran,
        }
    }
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == Typeselect::MemTran
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == Typeselect::PerTran
    }
}
#[doc = "Field `TYPE` writer - Channel x Transfer Type"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG, Typeselect>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut crate::W<REG> {
        self.variant(Typeselect::MemTran)
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut crate::W<REG> {
        self.variant(Typeselect::PerTran)
    }
}
#[doc = "Channel x Memory Burst Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbsizeselect {
    #[doc = "0: The memory burst size is set to one."]
    Single = 0,
    #[doc = "1: The memory burst size is set to four."]
    Four = 1,
    #[doc = "2: The memory burst size is set to eight."]
    Eight = 2,
    #[doc = "3: The memory burst size is set to sixteen."]
    Sixteen = 3,
}
impl From<Mbsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Mbsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Mbsizeselect {}
#[doc = "Field `MBSIZE` reader - Channel x Memory Burst Size"]
pub type MbsizeR = crate::FieldReader<Mbsizeselect>;
impl MbsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbsizeselect {
        match self.bits {
            0 => Mbsizeselect::Single,
            1 => Mbsizeselect::Four,
            2 => Mbsizeselect::Eight,
            3 => Mbsizeselect::Sixteen,
            _ => unreachable!(),
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Mbsizeselect::Single
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Mbsizeselect::Four
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Mbsizeselect::Eight
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Mbsizeselect::Sixteen
    }
}
#[doc = "Field `MBSIZE` writer - Channel x Memory Burst Size"]
pub type MbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mbsizeselect, crate::Safe>;
impl<'a, REG> MbsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Single)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Four)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Eight)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsizeselect::Sixteen)
    }
}
#[doc = "Channel x Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsyncselect {
    #[doc = "0: Peripheral-to-memory transfer."]
    Per2mem = 0,
    #[doc = "1: Memory-to-peripheral transfer."]
    Mem2per = 1,
}
impl From<Dsyncselect> for bool {
    #[inline(always)]
    fn from(variant: Dsyncselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSYNC` reader - Channel x Synchronization"]
pub type DsyncR = crate::BitReader<Dsyncselect>;
impl DsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsyncselect {
        match self.bits {
            false => Dsyncselect::Per2mem,
            true => Dsyncselect::Mem2per,
        }
    }
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == Dsyncselect::Per2mem
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == Dsyncselect::Mem2per
    }
}
#[doc = "Field `DSYNC` writer - Channel x Synchronization"]
pub type DsyncW<'a, REG> = crate::BitWriter<'a, REG, Dsyncselect>;
impl<'a, REG> DsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut crate::W<REG> {
        self.variant(Dsyncselect::Per2mem)
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut crate::W<REG> {
        self.variant(Dsyncselect::Mem2per)
    }
}
#[doc = "Channel x Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protselect {
    #[doc = "0: Channel uses Privileged mode."]
    Privileged = 0,
    #[doc = "1: Channel uses User mode."]
    User = 1,
}
impl From<Protselect> for bool {
    #[inline(always)]
    fn from(variant: Protselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROT` reader - Channel x Protection"]
pub type ProtR = crate::BitReader<Protselect>;
impl ProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protselect {
        match self.bits {
            false => Protselect::Privileged,
            true => Protselect::User,
        }
    }
    #[doc = "Channel uses Privileged mode."]
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == Protselect::Privileged
    }
    #[doc = "Channel uses User mode."]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == Protselect::User
    }
}
#[doc = "Field `PROT` writer - Channel x Protection"]
pub type ProtW<'a, REG> = crate::BitWriter<'a, REG, Protselect>;
impl<'a, REG> ProtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel uses Privileged mode."]
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(Protselect::Privileged)
    }
    #[doc = "Channel uses User mode."]
    #[inline(always)]
    pub fn user(self) -> &'a mut crate::W<REG> {
        self.variant(Protselect::User)
    }
}
#[doc = "Channel x Software Request Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swreqselect {
    #[doc = "0: Hardware request line is connected to the peripheral request line."]
    HwrConnected = 0,
    #[doc = "1: Software request is connected to the peripheral request line."]
    SwrConnected = 1,
}
impl From<Swreqselect> for bool {
    #[inline(always)]
    fn from(variant: Swreqselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - Channel x Software Request Trigger"]
pub type SwreqR = crate::BitReader<Swreqselect>;
impl SwreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swreqselect {
        match self.bits {
            false => Swreqselect::HwrConnected,
            true => Swreqselect::SwrConnected,
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == Swreqselect::HwrConnected
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == Swreqselect::SwrConnected
    }
}
#[doc = "Field `SWREQ` writer - Channel x Software Request Trigger"]
pub type SwreqW<'a, REG> = crate::BitWriter<'a, REG, Swreqselect>;
impl<'a, REG> SwreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(Swreqselect::HwrConnected)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(Swreqselect::SwrConnected)
    }
}
#[doc = "Channel x Fill Block of Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memsetselect {
    #[doc = "0: Memset is not activated."]
    NormalMode = 0,
    #[doc = "1: Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HwMode = 1,
}
impl From<Memsetselect> for bool {
    #[inline(always)]
    fn from(variant: Memsetselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMSET` reader - Channel x Fill Block of Memory"]
pub type MemsetR = crate::BitReader<Memsetselect>;
impl MemsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memsetselect {
        match self.bits {
            false => Memsetselect::NormalMode,
            true => Memsetselect::HwMode,
        }
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Memsetselect::NormalMode
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == Memsetselect::HwMode
    }
}
#[doc = "Field `MEMSET` writer - Channel x Fill Block of Memory"]
pub type MemsetW<'a, REG> = crate::BitWriter<'a, REG, Memsetselect>;
impl<'a, REG> MemsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Memsetselect::NormalMode)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Memsetselect::HwMode)
    }
}
#[doc = "Channel x Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csizeselect {
    #[doc = "0: 1 data transferred"]
    Chk1 = 0,
    #[doc = "1: 2 data transferred"]
    Chk2 = 1,
    #[doc = "2: 4 data transferred"]
    Chk4 = 2,
    #[doc = "3: 8 data transferred"]
    Chk8 = 3,
    #[doc = "4: 16 data transferred"]
    Chk16 = 4,
}
impl From<Csizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Csizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Csizeselect {}
#[doc = "Field `CSIZE` reader - Channel x Chunk Size"]
pub type CsizeR = crate::FieldReader<Csizeselect>;
impl CsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csizeselect> {
        match self.bits {
            0 => Some(Csizeselect::Chk1),
            1 => Some(Csizeselect::Chk2),
            2 => Some(Csizeselect::Chk4),
            3 => Some(Csizeselect::Chk8),
            4 => Some(Csizeselect::Chk16),
            _ => None,
        }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == Csizeselect::Chk1
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == Csizeselect::Chk2
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == Csizeselect::Chk4
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == Csizeselect::Chk8
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == Csizeselect::Chk16
    }
}
#[doc = "Field `CSIZE` writer - Channel x Chunk Size"]
pub type CsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csizeselect>;
impl<'a, REG> CsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut crate::W<REG> {
        self.variant(Csizeselect::Chk16)
    }
}
#[doc = "Channel x Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwidthselect {
    #[doc = "0: The data size is set to 8 bits"]
    Byte = 0,
    #[doc = "1: The data size is set to 16 bits"]
    Halfword = 1,
    #[doc = "2: The data size is set to 32 bits"]
    Word = 2,
}
impl From<Dwidthselect> for u8 {
    #[inline(always)]
    fn from(variant: Dwidthselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwidthselect {
    type Ux = u8;
}
impl crate::IsEnum for Dwidthselect {}
#[doc = "Field `DWIDTH` reader - Channel x Data Width"]
pub type DwidthR = crate::FieldReader<Dwidthselect>;
impl DwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dwidthselect> {
        match self.bits {
            0 => Some(Dwidthselect::Byte),
            1 => Some(Dwidthselect::Halfword),
            2 => Some(Dwidthselect::Word),
            _ => None,
        }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dwidthselect::Byte
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Dwidthselect::Halfword
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dwidthselect::Word
    }
}
#[doc = "Field `DWIDTH` writer - Channel x Data Width"]
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dwidthselect>;
impl<'a, REG> DwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Byte)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Halfword)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidthselect::Word)
    }
}
#[doc = "Channel x Source Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sifselect {
    #[doc = "0: The data is read through system bus interface 0."]
    AhbIf0 = 0,
    #[doc = "1: The data is read through system bus interface 1."]
    AhbIf1 = 1,
}
impl From<Sifselect> for bool {
    #[inline(always)]
    fn from(variant: Sifselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIF` reader - Channel x Source Interface Identifier"]
pub type SifR = crate::BitReader<Sifselect>;
impl SifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sifselect {
        match self.bits {
            false => Sifselect::AhbIf0,
            true => Sifselect::AhbIf1,
        }
    }
    #[doc = "The data is read through system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == Sifselect::AhbIf0
    }
    #[doc = "The data is read through system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == Sifselect::AhbIf1
    }
}
#[doc = "Field `SIF` writer - Channel x Source Interface Identifier"]
pub type SifW<'a, REG> = crate::BitWriter<'a, REG, Sifselect>;
impl<'a, REG> SifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is read through system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(Sifselect::AhbIf0)
    }
    #[doc = "The data is read through system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(Sifselect::AhbIf1)
    }
}
#[doc = "Channel x Destination Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difselect {
    #[doc = "0: The data is written through system bus interface 0."]
    AhbIf0 = 0,
    #[doc = "1: The data is written though system bus interface 1."]
    AhbIf1 = 1,
}
impl From<Difselect> for bool {
    #[inline(always)]
    fn from(variant: Difselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIF` reader - Channel x Destination Interface Identifier"]
pub type DifR = crate::BitReader<Difselect>;
impl DifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Difselect {
        match self.bits {
            false => Difselect::AhbIf0,
            true => Difselect::AhbIf1,
        }
    }
    #[doc = "The data is written through system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == Difselect::AhbIf0
    }
    #[doc = "The data is written though system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == Difselect::AhbIf1
    }
}
#[doc = "Field `DIF` writer - Channel x Destination Interface Identifier"]
pub type DifW<'a, REG> = crate::BitWriter<'a, REG, Difselect>;
impl<'a, REG> DifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is written through system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(Difselect::AhbIf0)
    }
    #[doc = "The data is written though system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(Difselect::AhbIf1)
    }
}
#[doc = "Channel x Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samselect {
    #[doc = "0: The address remains unchanged."]
    FixedAm = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    IncrementedAm = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UbsAm = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UbsDsAm = 3,
}
impl From<Samselect> for u8 {
    #[inline(always)]
    fn from(variant: Samselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samselect {
    type Ux = u8;
}
impl crate::IsEnum for Samselect {}
#[doc = "Field `SAM` reader - Channel x Source Addressing Mode"]
pub type SamR = crate::FieldReader<Samselect>;
impl SamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samselect {
        match self.bits {
            0 => Samselect::FixedAm,
            1 => Samselect::IncrementedAm,
            2 => Samselect::UbsAm,
            3 => Samselect::UbsDsAm,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == Samselect::FixedAm
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == Samselect::IncrementedAm
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == Samselect::UbsAm
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == Samselect::UbsDsAm
    }
}
#[doc = "Field `SAM` writer - Channel x Source Addressing Mode"]
pub type SamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Samselect, crate::Safe>;
impl<'a, REG> SamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::FixedAm)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::IncrementedAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::UbsAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(Samselect::UbsDsAm)
    }
}
#[doc = "Channel x Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Damselect {
    #[doc = "0: The address remains unchanged."]
    FixedAm = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    IncrementedAm = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UbsAm = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UbsDsAm = 3,
}
impl From<Damselect> for u8 {
    #[inline(always)]
    fn from(variant: Damselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Damselect {
    type Ux = u8;
}
impl crate::IsEnum for Damselect {}
#[doc = "Field `DAM` reader - Channel x Destination Addressing Mode"]
pub type DamR = crate::FieldReader<Damselect>;
impl DamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Damselect {
        match self.bits {
            0 => Damselect::FixedAm,
            1 => Damselect::IncrementedAm,
            2 => Damselect::UbsAm,
            3 => Damselect::UbsDsAm,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == Damselect::FixedAm
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == Damselect::IncrementedAm
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == Damselect::UbsAm
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == Damselect::UbsDsAm
    }
}
#[doc = "Field `DAM` writer - Channel x Destination Addressing Mode"]
pub type DamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Damselect, crate::Safe>;
impl<'a, REG> DamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::FixedAm)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::IncrementedAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::UbsAm)
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(Damselect::UbsDsAm)
    }
}
#[doc = "Channel Initialization Done (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initdselect {
    #[doc = "0: Channel initialization is in progress."]
    InProgress = 0,
    #[doc = "1: Channel initialization is completed."]
    Terminated = 1,
}
impl From<Initdselect> for bool {
    #[inline(always)]
    fn from(variant: Initdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITD` reader - Channel Initialization Done (this bit is read-only)"]
pub type InitdR = crate::BitReader<Initdselect>;
impl InitdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initdselect {
        match self.bits {
            false => Initdselect::InProgress,
            true => Initdselect::Terminated,
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Initdselect::InProgress
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == Initdselect::Terminated
    }
}
#[doc = "Field `INITD` writer - Channel Initialization Done (this bit is read-only)"]
pub type InitdW<'a, REG> = crate::BitWriter<'a, REG, Initdselect>;
impl<'a, REG> InitdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Initdselect::InProgress)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut crate::W<REG> {
        self.variant(Initdselect::Terminated)
    }
}
#[doc = "Read in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdipselect {
    #[doc = "0: No active read transaction on the bus."]
    Done = 0,
    #[doc = "1: A read transaction is in progress."]
    InProgress = 1,
}
impl From<Rdipselect> for bool {
    #[inline(always)]
    fn from(variant: Rdipselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIP` reader - Read in Progress (this bit is read-only)"]
pub type RdipR = crate::BitReader<Rdipselect>;
impl RdipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdipselect {
        match self.bits {
            false => Rdipselect::Done,
            true => Rdipselect::InProgress,
        }
    }
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Rdipselect::Done
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Rdipselect::InProgress
    }
}
#[doc = "Field `RDIP` writer - Read in Progress (this bit is read-only)"]
pub type RdipW<'a, REG> = crate::BitWriter<'a, REG, Rdipselect>;
impl<'a, REG> RdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(Rdipselect::Done)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Rdipselect::InProgress)
    }
}
#[doc = "Write in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wripselect {
    #[doc = "0: No active write transaction on the bus."]
    Done = 0,
    #[doc = "1: A write transaction is in progress."]
    InProgress = 1,
}
impl From<Wripselect> for bool {
    #[inline(always)]
    fn from(variant: Wripselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRIP` reader - Write in Progress (this bit is read-only)"]
pub type WripR = crate::BitReader<Wripselect>;
impl WripR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wripselect {
        match self.bits {
            false => Wripselect::Done,
            true => Wripselect::InProgress,
        }
    }
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Wripselect::Done
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Wripselect::InProgress
    }
}
#[doc = "Field `WRIP` writer - Write in Progress (this bit is read-only)"]
pub type WripW<'a, REG> = crate::BitWriter<'a, REG, Wripselect>;
impl<'a, REG> WripW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(Wripselect::Done)
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Wripselect::InProgress)
    }
}
#[doc = "Channel x Peripheral Hardware Request Line Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peridselect {
    #[doc = "0: FLEXCOM0_TX"]
    Flexcom0Tx = 0,
    #[doc = "1: FLEXCOM0_RX"]
    Flexcom0Rx = 1,
    #[doc = "2: FLEXCOM1_TX"]
    Flexcom1Tx = 2,
    #[doc = "3: FLEXCOM1_RX"]
    Flexcom1Rx = 3,
    #[doc = "4: FLEXCOM2_TX"]
    Flexcom2Tx = 4,
    #[doc = "5: FLEXCOM2_RX"]
    Flexcom2Rx = 5,
    #[doc = "6: FLEXCOM3_TX"]
    Flexcom3Tx = 6,
    #[doc = "7: FLEXCOM3_RX"]
    Flexcom3Rx = 7,
    #[doc = "8: FLEXCOM4_TX"]
    Flexcom4Tx = 8,
    #[doc = "9: FLEXCOM4_RX"]
    Flexcom4Rx = 9,
    #[doc = "10: FLEXCOM5_TX"]
    Flexcom5Tx = 10,
    #[doc = "11: FLEXCOM5_RX"]
    Flexcom5Rx = 11,
    #[doc = "12: FLEXCOM6_TX"]
    Flexcom6Tx = 12,
    #[doc = "13: FLEXCOM6_RX"]
    Flexcom6Rx = 13,
    #[doc = "14: FLEXCOM7_TX"]
    Flexcom7Tx = 14,
    #[doc = "15: FLEXCOM7_RX"]
    Flexcom7Rx = 15,
    #[doc = "16: FLEXCOM8_TX"]
    Flexcom8Tx = 16,
    #[doc = "17: FLEXCOM8_RX"]
    Flexcom8Rx = 17,
    #[doc = "18: FLEXCOM9_TX"]
    Flexcom9Tx = 18,
    #[doc = "19: FLEXCOM9_RX"]
    Flexcom9Rx = 19,
    #[doc = "20: QSPI_TX"]
    QspiTx = 20,
    #[doc = "21: QSPI_RX"]
    QspiRx = 21,
    #[doc = "22: PWM0"]
    Pwm0 = 22,
    #[doc = "23: PWM1"]
    Pwm1 = 23,
    #[doc = "24: TC0"]
    Tc0 = 24,
    #[doc = "25: TC1"]
    Tc1 = 25,
    #[doc = "26: TC2"]
    Tc2 = 26,
    #[doc = "27: TC3"]
    Tc3 = 27,
    #[doc = "28: SHA"]
    Sha = 28,
    #[doc = "29: TC1_CPA"]
    Tc1Cpa = 29,
    #[doc = "30: TC4_CPA"]
    Tc4Cpa = 30,
    #[doc = "31: TC7_CPA"]
    Tc7Cpa = 31,
    #[doc = "32: TC10_CPA"]
    Tc10Cpa = 32,
    #[doc = "33: TC1_CPB"]
    Tc1Cpb = 33,
    #[doc = "34: TC4_CPB"]
    Tc4Cpb = 34,
    #[doc = "35: TC7_CPB"]
    Tc7Cpb = 35,
    #[doc = "36: TC10_CPB"]
    Tc10Cpb = 36,
    #[doc = "37: TC1_CPC"]
    Tc1Cpc = 37,
    #[doc = "38: TC4_CPC"]
    Tc4Cpc = 38,
    #[doc = "39: TC7_CPC"]
    Tc7Cpc = 39,
    #[doc = "40: TC10_CPC"]
    Tc10Cpc = 40,
    #[doc = "41: TC1_ETRG"]
    Tc1Etrg = 41,
    #[doc = "42: TC4_ETRG"]
    Tc4Etrg = 42,
    #[doc = "43: TC7_ETRG"]
    Tc7Etrg = 43,
    #[doc = "44: TC10_ETRG"]
    Tc10Etrg = 44,
}
impl From<Peridselect> for u8 {
    #[inline(always)]
    fn from(variant: Peridselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peridselect {
    type Ux = u8;
}
impl crate::IsEnum for Peridselect {}
#[doc = "Field `PERID` reader - Channel x Peripheral Hardware Request Line Identifier"]
pub type PeridR = crate::FieldReader<Peridselect>;
impl PeridR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Peridselect> {
        match self.bits {
            0 => Some(Peridselect::Flexcom0Tx),
            1 => Some(Peridselect::Flexcom0Rx),
            2 => Some(Peridselect::Flexcom1Tx),
            3 => Some(Peridselect::Flexcom1Rx),
            4 => Some(Peridselect::Flexcom2Tx),
            5 => Some(Peridselect::Flexcom2Rx),
            6 => Some(Peridselect::Flexcom3Tx),
            7 => Some(Peridselect::Flexcom3Rx),
            8 => Some(Peridselect::Flexcom4Tx),
            9 => Some(Peridselect::Flexcom4Rx),
            10 => Some(Peridselect::Flexcom5Tx),
            11 => Some(Peridselect::Flexcom5Rx),
            12 => Some(Peridselect::Flexcom6Tx),
            13 => Some(Peridselect::Flexcom6Rx),
            14 => Some(Peridselect::Flexcom7Tx),
            15 => Some(Peridselect::Flexcom7Rx),
            16 => Some(Peridselect::Flexcom8Tx),
            17 => Some(Peridselect::Flexcom8Rx),
            18 => Some(Peridselect::Flexcom9Tx),
            19 => Some(Peridselect::Flexcom9Rx),
            20 => Some(Peridselect::QspiTx),
            21 => Some(Peridselect::QspiRx),
            22 => Some(Peridselect::Pwm0),
            23 => Some(Peridselect::Pwm1),
            24 => Some(Peridselect::Tc0),
            25 => Some(Peridselect::Tc1),
            26 => Some(Peridselect::Tc2),
            27 => Some(Peridselect::Tc3),
            28 => Some(Peridselect::Sha),
            29 => Some(Peridselect::Tc1Cpa),
            30 => Some(Peridselect::Tc4Cpa),
            31 => Some(Peridselect::Tc7Cpa),
            32 => Some(Peridselect::Tc10Cpa),
            33 => Some(Peridselect::Tc1Cpb),
            34 => Some(Peridselect::Tc4Cpb),
            35 => Some(Peridselect::Tc7Cpb),
            36 => Some(Peridselect::Tc10Cpb),
            37 => Some(Peridselect::Tc1Cpc),
            38 => Some(Peridselect::Tc4Cpc),
            39 => Some(Peridselect::Tc7Cpc),
            40 => Some(Peridselect::Tc10Cpc),
            41 => Some(Peridselect::Tc1Etrg),
            42 => Some(Peridselect::Tc4Etrg),
            43 => Some(Peridselect::Tc7Etrg),
            44 => Some(Peridselect::Tc10Etrg),
            _ => None,
        }
    }
    #[doc = "FLEXCOM0_TX"]
    #[inline(always)]
    pub fn is_flexcom0_tx(&self) -> bool {
        *self == Peridselect::Flexcom0Tx
    }
    #[doc = "FLEXCOM0_RX"]
    #[inline(always)]
    pub fn is_flexcom0_rx(&self) -> bool {
        *self == Peridselect::Flexcom0Rx
    }
    #[doc = "FLEXCOM1_TX"]
    #[inline(always)]
    pub fn is_flexcom1_tx(&self) -> bool {
        *self == Peridselect::Flexcom1Tx
    }
    #[doc = "FLEXCOM1_RX"]
    #[inline(always)]
    pub fn is_flexcom1_rx(&self) -> bool {
        *self == Peridselect::Flexcom1Rx
    }
    #[doc = "FLEXCOM2_TX"]
    #[inline(always)]
    pub fn is_flexcom2_tx(&self) -> bool {
        *self == Peridselect::Flexcom2Tx
    }
    #[doc = "FLEXCOM2_RX"]
    #[inline(always)]
    pub fn is_flexcom2_rx(&self) -> bool {
        *self == Peridselect::Flexcom2Rx
    }
    #[doc = "FLEXCOM3_TX"]
    #[inline(always)]
    pub fn is_flexcom3_tx(&self) -> bool {
        *self == Peridselect::Flexcom3Tx
    }
    #[doc = "FLEXCOM3_RX"]
    #[inline(always)]
    pub fn is_flexcom3_rx(&self) -> bool {
        *self == Peridselect::Flexcom3Rx
    }
    #[doc = "FLEXCOM4_TX"]
    #[inline(always)]
    pub fn is_flexcom4_tx(&self) -> bool {
        *self == Peridselect::Flexcom4Tx
    }
    #[doc = "FLEXCOM4_RX"]
    #[inline(always)]
    pub fn is_flexcom4_rx(&self) -> bool {
        *self == Peridselect::Flexcom4Rx
    }
    #[doc = "FLEXCOM5_TX"]
    #[inline(always)]
    pub fn is_flexcom5_tx(&self) -> bool {
        *self == Peridselect::Flexcom5Tx
    }
    #[doc = "FLEXCOM5_RX"]
    #[inline(always)]
    pub fn is_flexcom5_rx(&self) -> bool {
        *self == Peridselect::Flexcom5Rx
    }
    #[doc = "FLEXCOM6_TX"]
    #[inline(always)]
    pub fn is_flexcom6_tx(&self) -> bool {
        *self == Peridselect::Flexcom6Tx
    }
    #[doc = "FLEXCOM6_RX"]
    #[inline(always)]
    pub fn is_flexcom6_rx(&self) -> bool {
        *self == Peridselect::Flexcom6Rx
    }
    #[doc = "FLEXCOM7_TX"]
    #[inline(always)]
    pub fn is_flexcom7_tx(&self) -> bool {
        *self == Peridselect::Flexcom7Tx
    }
    #[doc = "FLEXCOM7_RX"]
    #[inline(always)]
    pub fn is_flexcom7_rx(&self) -> bool {
        *self == Peridselect::Flexcom7Rx
    }
    #[doc = "FLEXCOM8_TX"]
    #[inline(always)]
    pub fn is_flexcom8_tx(&self) -> bool {
        *self == Peridselect::Flexcom8Tx
    }
    #[doc = "FLEXCOM8_RX"]
    #[inline(always)]
    pub fn is_flexcom8_rx(&self) -> bool {
        *self == Peridselect::Flexcom8Rx
    }
    #[doc = "FLEXCOM9_TX"]
    #[inline(always)]
    pub fn is_flexcom9_tx(&self) -> bool {
        *self == Peridselect::Flexcom9Tx
    }
    #[doc = "FLEXCOM9_RX"]
    #[inline(always)]
    pub fn is_flexcom9_rx(&self) -> bool {
        *self == Peridselect::Flexcom9Rx
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == Peridselect::QspiTx
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == Peridselect::QspiRx
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == Peridselect::Pwm0
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Peridselect::Pwm1
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == Peridselect::Tc0
    }
    #[doc = "TC1"]
    #[inline(always)]
    pub fn is_tc1(&self) -> bool {
        *self == Peridselect::Tc1
    }
    #[doc = "TC2"]
    #[inline(always)]
    pub fn is_tc2(&self) -> bool {
        *self == Peridselect::Tc2
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn is_tc3(&self) -> bool {
        *self == Peridselect::Tc3
    }
    #[doc = "SHA"]
    #[inline(always)]
    pub fn is_sha(&self) -> bool {
        *self == Peridselect::Sha
    }
    #[doc = "TC1_CPA"]
    #[inline(always)]
    pub fn is_tc1_cpa(&self) -> bool {
        *self == Peridselect::Tc1Cpa
    }
    #[doc = "TC4_CPA"]
    #[inline(always)]
    pub fn is_tc4_cpa(&self) -> bool {
        *self == Peridselect::Tc4Cpa
    }
    #[doc = "TC7_CPA"]
    #[inline(always)]
    pub fn is_tc7_cpa(&self) -> bool {
        *self == Peridselect::Tc7Cpa
    }
    #[doc = "TC10_CPA"]
    #[inline(always)]
    pub fn is_tc10_cpa(&self) -> bool {
        *self == Peridselect::Tc10Cpa
    }
    #[doc = "TC1_CPB"]
    #[inline(always)]
    pub fn is_tc1_cpb(&self) -> bool {
        *self == Peridselect::Tc1Cpb
    }
    #[doc = "TC4_CPB"]
    #[inline(always)]
    pub fn is_tc4_cpb(&self) -> bool {
        *self == Peridselect::Tc4Cpb
    }
    #[doc = "TC7_CPB"]
    #[inline(always)]
    pub fn is_tc7_cpb(&self) -> bool {
        *self == Peridselect::Tc7Cpb
    }
    #[doc = "TC10_CPB"]
    #[inline(always)]
    pub fn is_tc10_cpb(&self) -> bool {
        *self == Peridselect::Tc10Cpb
    }
    #[doc = "TC1_CPC"]
    #[inline(always)]
    pub fn is_tc1_cpc(&self) -> bool {
        *self == Peridselect::Tc1Cpc
    }
    #[doc = "TC4_CPC"]
    #[inline(always)]
    pub fn is_tc4_cpc(&self) -> bool {
        *self == Peridselect::Tc4Cpc
    }
    #[doc = "TC7_CPC"]
    #[inline(always)]
    pub fn is_tc7_cpc(&self) -> bool {
        *self == Peridselect::Tc7Cpc
    }
    #[doc = "TC10_CPC"]
    #[inline(always)]
    pub fn is_tc10_cpc(&self) -> bool {
        *self == Peridselect::Tc10Cpc
    }
    #[doc = "TC1_ETRG"]
    #[inline(always)]
    pub fn is_tc1_etrg(&self) -> bool {
        *self == Peridselect::Tc1Etrg
    }
    #[doc = "TC4_ETRG"]
    #[inline(always)]
    pub fn is_tc4_etrg(&self) -> bool {
        *self == Peridselect::Tc4Etrg
    }
    #[doc = "TC7_ETRG"]
    #[inline(always)]
    pub fn is_tc7_etrg(&self) -> bool {
        *self == Peridselect::Tc7Etrg
    }
    #[doc = "TC10_ETRG"]
    #[inline(always)]
    pub fn is_tc10_etrg(&self) -> bool {
        *self == Peridselect::Tc10Etrg
    }
}
#[doc = "Field `PERID` writer - Channel x Peripheral Hardware Request Line Identifier"]
pub type PeridW<'a, REG> = crate::FieldWriter<'a, REG, 7, Peridselect>;
impl<'a, REG> PeridW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLEXCOM0_TX"]
    #[inline(always)]
    pub fn flexcom0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom0Tx)
    }
    #[doc = "FLEXCOM0_RX"]
    #[inline(always)]
    pub fn flexcom0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom0Rx)
    }
    #[doc = "FLEXCOM1_TX"]
    #[inline(always)]
    pub fn flexcom1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom1Tx)
    }
    #[doc = "FLEXCOM1_RX"]
    #[inline(always)]
    pub fn flexcom1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom1Rx)
    }
    #[doc = "FLEXCOM2_TX"]
    #[inline(always)]
    pub fn flexcom2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom2Tx)
    }
    #[doc = "FLEXCOM2_RX"]
    #[inline(always)]
    pub fn flexcom2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom2Rx)
    }
    #[doc = "FLEXCOM3_TX"]
    #[inline(always)]
    pub fn flexcom3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom3Tx)
    }
    #[doc = "FLEXCOM3_RX"]
    #[inline(always)]
    pub fn flexcom3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom3Rx)
    }
    #[doc = "FLEXCOM4_TX"]
    #[inline(always)]
    pub fn flexcom4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom4Tx)
    }
    #[doc = "FLEXCOM4_RX"]
    #[inline(always)]
    pub fn flexcom4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom4Rx)
    }
    #[doc = "FLEXCOM5_TX"]
    #[inline(always)]
    pub fn flexcom5_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom5Tx)
    }
    #[doc = "FLEXCOM5_RX"]
    #[inline(always)]
    pub fn flexcom5_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom5Rx)
    }
    #[doc = "FLEXCOM6_TX"]
    #[inline(always)]
    pub fn flexcom6_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom6Tx)
    }
    #[doc = "FLEXCOM6_RX"]
    #[inline(always)]
    pub fn flexcom6_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom6Rx)
    }
    #[doc = "FLEXCOM7_TX"]
    #[inline(always)]
    pub fn flexcom7_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom7Tx)
    }
    #[doc = "FLEXCOM7_RX"]
    #[inline(always)]
    pub fn flexcom7_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom7Rx)
    }
    #[doc = "FLEXCOM8_TX"]
    #[inline(always)]
    pub fn flexcom8_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom8Tx)
    }
    #[doc = "FLEXCOM8_RX"]
    #[inline(always)]
    pub fn flexcom8_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom8Rx)
    }
    #[doc = "FLEXCOM9_TX"]
    #[inline(always)]
    pub fn flexcom9_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom9Tx)
    }
    #[doc = "FLEXCOM9_RX"]
    #[inline(always)]
    pub fn flexcom9_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Flexcom9Rx)
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::QspiTx)
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::QspiRx)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pwm0)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Pwm1)
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc0)
    }
    #[doc = "TC1"]
    #[inline(always)]
    pub fn tc1(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1)
    }
    #[doc = "TC2"]
    #[inline(always)]
    pub fn tc2(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc2)
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn tc3(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc3)
    }
    #[doc = "SHA"]
    #[inline(always)]
    pub fn sha(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Sha)
    }
    #[doc = "TC1_CPA"]
    #[inline(always)]
    pub fn tc1_cpa(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1Cpa)
    }
    #[doc = "TC4_CPA"]
    #[inline(always)]
    pub fn tc4_cpa(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc4Cpa)
    }
    #[doc = "TC7_CPA"]
    #[inline(always)]
    pub fn tc7_cpa(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc7Cpa)
    }
    #[doc = "TC10_CPA"]
    #[inline(always)]
    pub fn tc10_cpa(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc10Cpa)
    }
    #[doc = "TC1_CPB"]
    #[inline(always)]
    pub fn tc1_cpb(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1Cpb)
    }
    #[doc = "TC4_CPB"]
    #[inline(always)]
    pub fn tc4_cpb(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc4Cpb)
    }
    #[doc = "TC7_CPB"]
    #[inline(always)]
    pub fn tc7_cpb(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc7Cpb)
    }
    #[doc = "TC10_CPB"]
    #[inline(always)]
    pub fn tc10_cpb(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc10Cpb)
    }
    #[doc = "TC1_CPC"]
    #[inline(always)]
    pub fn tc1_cpc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1Cpc)
    }
    #[doc = "TC4_CPC"]
    #[inline(always)]
    pub fn tc4_cpc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc4Cpc)
    }
    #[doc = "TC7_CPC"]
    #[inline(always)]
    pub fn tc7_cpc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc7Cpc)
    }
    #[doc = "TC10_CPC"]
    #[inline(always)]
    pub fn tc10_cpc(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc10Cpc)
    }
    #[doc = "TC1_ETRG"]
    #[inline(always)]
    pub fn tc1_etrg(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc1Etrg)
    }
    #[doc = "TC4_ETRG"]
    #[inline(always)]
    pub fn tc4_etrg(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc4Etrg)
    }
    #[doc = "TC7_ETRG"]
    #[inline(always)]
    pub fn tc7_etrg(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc7Etrg)
    }
    #[doc = "TC10_ETRG"]
    #[inline(always)]
    pub fn tc10_etrg(self) -> &'a mut crate::W<REG> {
        self.variant(Peridselect::Tc10Etrg)
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MbsizeR {
        MbsizeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DsyncR {
        DsyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel x Protection"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SwreqR {
        SwreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of Memory"]
    #[inline(always)]
    pub fn memset(&self) -> MemsetR {
        MemsetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CsizeR {
        CsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SifR {
        SifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DifR {
        DifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DamR {
        DamR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Done (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> InitdR {
        InitdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RdipR {
        RdipR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WripR {
        WripR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PeridR {
        PeridR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<CcSpec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&mut self) -> MbsizeW<CcSpec> {
        MbsizeW::new(self, 1)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&mut self) -> DsyncW<CcSpec> {
        DsyncW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel x Protection"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<CcSpec> {
        ProtW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<CcSpec> {
        SwreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel x Fill Block of Memory"]
    #[inline(always)]
    pub fn memset(&mut self) -> MemsetW<CcSpec> {
        MemsetW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&mut self) -> CsizeW<CcSpec> {
        CsizeW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<CcSpec> {
        DwidthW::new(self, 11)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&mut self) -> SifW<CcSpec> {
        SifW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&mut self) -> DifW<CcSpec> {
        DifW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<CcSpec> {
        SamW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&mut self) -> DamW<CcSpec> {
        DamW::new(self, 18)
    }
    #[doc = "Bit 21 - Channel Initialization Done (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&mut self) -> InitdW<CcSpec> {
        InitdW::new(self, 21)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&mut self) -> RdipW<CcSpec> {
        RdipW::new(self, 22)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&mut self) -> WripW<CcSpec> {
        WripW::new(self, 23)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> PeridW<CcSpec> {
        PeridW::new(self, 24)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CcSpec {}
