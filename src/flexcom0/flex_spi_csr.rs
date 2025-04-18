#[doc = "Register `FLEX_SPI_CSR[%s]` reader"]
pub type R = crate::R<FlexSpiCsrSpec>;
#[doc = "Register `FLEX_SPI_CSR[%s]` writer"]
pub type W = crate::W<FlexSpiCsrSpec>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NcphaR = crate::BitReader;
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NcphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CsnaatR = crate::BitReader;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CsnaatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CsaatR = crate::BitReader;
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CsaatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bitsselect {
    #[doc = "0: 8 bits for transfer"]
    _8Bit = 0,
    #[doc = "1: 9 bits for transfer"]
    _9Bit = 1,
    #[doc = "2: 10 bits for transfer"]
    _10Bit = 2,
    #[doc = "3: 11 bits for transfer"]
    _11Bit = 3,
    #[doc = "4: 12 bits for transfer"]
    _12Bit = 4,
    #[doc = "5: 13 bits for transfer"]
    _13Bit = 5,
    #[doc = "6: 14 bits for transfer"]
    _14Bit = 6,
    #[doc = "7: 15 bits for transfer"]
    _15Bit = 7,
    #[doc = "8: 16 bits for transfer"]
    _16Bit = 8,
}
impl From<Bitsselect> for u8 {
    #[inline(always)]
    fn from(variant: Bitsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bitsselect {
    type Ux = u8;
}
impl crate::IsEnum for Bitsselect {}
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BitsR = crate::FieldReader<Bitsselect>;
impl BitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bitsselect> {
        match self.bits {
            0 => Some(Bitsselect::_8Bit),
            1 => Some(Bitsselect::_9Bit),
            2 => Some(Bitsselect::_10Bit),
            3 => Some(Bitsselect::_11Bit),
            4 => Some(Bitsselect::_12Bit),
            5 => Some(Bitsselect::_13Bit),
            6 => Some(Bitsselect::_14Bit),
            7 => Some(Bitsselect::_15Bit),
            8 => Some(Bitsselect::_16Bit),
            _ => None,
        }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Bitsselect::_8Bit
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Bitsselect::_9Bit
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == Bitsselect::_10Bit
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == Bitsselect::_11Bit
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == Bitsselect::_12Bit
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == Bitsselect::_13Bit
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == Bitsselect::_14Bit
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == Bitsselect::_15Bit
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Bitsselect::_16Bit
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bitsselect>;
impl<'a, REG> BitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_8Bit)
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_9Bit)
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_10Bit)
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_11Bit)
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_12Bit)
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_13Bit)
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_14Bit)
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_15Bit)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Bitsselect::_16Bit)
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Bit Rate"]
pub type ScbrR = crate::FieldReader;
#[doc = "Field `SCBR` writer - Serial Clock Bit Rate"]
pub type ScbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DlybsR = crate::FieldReader;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DlybsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DlybctR = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DlybctW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NcphaR {
        NcphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CsnaatR {
        CsnaatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CsaatR {
        CsaatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> ScbrR {
        ScbrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DlybsR {
        DlybsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DlybctR {
        DlybctR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<FlexSpiCsrSpec> {
        CpolW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&mut self) -> NcphaW<FlexSpiCsrSpec> {
        NcphaW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&mut self) -> CsnaatW<FlexSpiCsrSpec> {
        CsnaatW::new(self, 2)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&mut self) -> CsaatW<FlexSpiCsrSpec> {
        CsaatW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BitsW<FlexSpiCsrSpec> {
        BitsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> ScbrW<FlexSpiCsrSpec> {
        ScbrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DlybsW<FlexSpiCsrSpec> {
        DlybsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DlybctW<FlexSpiCsrSpec> {
        DlybctW::new(self, 24)
    }
}
#[doc = "SPI Chip Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiCsrSpec;
impl crate::RegisterSpec for FlexSpiCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_csr::R`](R) reader structure"]
impl crate::Readable for FlexSpiCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_spi_csr::W`](W) writer structure"]
impl crate::Writable for FlexSpiCsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_CSR[%s] to value 0"]
impl crate::Resettable for FlexSpiCsrSpec {}
