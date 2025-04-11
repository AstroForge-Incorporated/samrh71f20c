#[doc = "Register `CR_NCS0` reader"]
pub type R = crate::R<CrNcs0Spec>;
#[doc = "Register `CR_NCS0` writer"]
pub type W = crate::W<CrNcs0Spec>;
#[doc = "Field `ZERO` reader - fixed to 0"]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - fixed to 0"]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bank Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Banksizeselect {
    #[doc = "0: 8Kbytes"]
    _8kb = 0,
    #[doc = "1: 16Kbytes"]
    _16kb = 1,
    #[doc = "2: 32Kbytes"]
    _32kb = 2,
    #[doc = "3: 64Kbytes"]
    _64kb = 3,
    #[doc = "4: 128Kbytes"]
    _128kb = 4,
    #[doc = "5: 256Kbytes"]
    _256kb = 5,
    #[doc = "6: 512Kbytes"]
    _512kb = 6,
    #[doc = "7: 1Mbytes"]
    _1mb = 7,
    #[doc = "8: 2Mbytes"]
    _2mb = 8,
    #[doc = "9: 4Mbytes"]
    _4mb = 9,
    #[doc = "10: 8Mbytes"]
    _8mb = 10,
    #[doc = "11: 16Mbytes"]
    _16mb = 11,
    #[doc = "12: 32Mbytes"]
    _32mb = 12,
    #[doc = "13: 64Mbytes"]
    _64mb = 13,
    #[doc = "14: 128Mbytes"]
    _128mb = 14,
    #[doc = "15: 256Mbytes (Default)"]
    _256mb = 15,
    #[doc = "16: 512Mbytes"]
    _512mb = 16,
    #[doc = "31: NOT_USED"]
    NotUsed = 31,
}
impl From<Banksizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Banksizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Banksizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Banksizeselect {}
#[doc = "Field `BANKSIZE` reader - Bank Size"]
pub type BanksizeR = crate::FieldReader<Banksizeselect>;
impl BanksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Banksizeselect> {
        match self.bits {
            0 => Some(Banksizeselect::_8kb),
            1 => Some(Banksizeselect::_16kb),
            2 => Some(Banksizeselect::_32kb),
            3 => Some(Banksizeselect::_64kb),
            4 => Some(Banksizeselect::_128kb),
            5 => Some(Banksizeselect::_256kb),
            6 => Some(Banksizeselect::_512kb),
            7 => Some(Banksizeselect::_1mb),
            8 => Some(Banksizeselect::_2mb),
            9 => Some(Banksizeselect::_4mb),
            10 => Some(Banksizeselect::_8mb),
            11 => Some(Banksizeselect::_16mb),
            12 => Some(Banksizeselect::_32mb),
            13 => Some(Banksizeselect::_64mb),
            14 => Some(Banksizeselect::_128mb),
            15 => Some(Banksizeselect::_256mb),
            16 => Some(Banksizeselect::_512mb),
            31 => Some(Banksizeselect::NotUsed),
            _ => None,
        }
    }
    #[doc = "8Kbytes"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == Banksizeselect::_8kb
    }
    #[doc = "16Kbytes"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Banksizeselect::_16kb
    }
    #[doc = "32Kbytes"]
    #[inline(always)]
    pub fn is_32kb(&self) -> bool {
        *self == Banksizeselect::_32kb
    }
    #[doc = "64Kbytes"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Banksizeselect::_64kb
    }
    #[doc = "128Kbytes"]
    #[inline(always)]
    pub fn is_128kb(&self) -> bool {
        *self == Banksizeselect::_128kb
    }
    #[doc = "256Kbytes"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == Banksizeselect::_256kb
    }
    #[doc = "512Kbytes"]
    #[inline(always)]
    pub fn is_512kb(&self) -> bool {
        *self == Banksizeselect::_512kb
    }
    #[doc = "1Mbytes"]
    #[inline(always)]
    pub fn is_1mb(&self) -> bool {
        *self == Banksizeselect::_1mb
    }
    #[doc = "2Mbytes"]
    #[inline(always)]
    pub fn is_2mb(&self) -> bool {
        *self == Banksizeselect::_2mb
    }
    #[doc = "4Mbytes"]
    #[inline(always)]
    pub fn is_4mb(&self) -> bool {
        *self == Banksizeselect::_4mb
    }
    #[doc = "8Mbytes"]
    #[inline(always)]
    pub fn is_8mb(&self) -> bool {
        *self == Banksizeselect::_8mb
    }
    #[doc = "16Mbytes"]
    #[inline(always)]
    pub fn is_16mb(&self) -> bool {
        *self == Banksizeselect::_16mb
    }
    #[doc = "32Mbytes"]
    #[inline(always)]
    pub fn is_32mb(&self) -> bool {
        *self == Banksizeselect::_32mb
    }
    #[doc = "64Mbytes"]
    #[inline(always)]
    pub fn is_64mb(&self) -> bool {
        *self == Banksizeselect::_64mb
    }
    #[doc = "128Mbytes"]
    #[inline(always)]
    pub fn is_128mb(&self) -> bool {
        *self == Banksizeselect::_128mb
    }
    #[doc = "256Mbytes (Default)"]
    #[inline(always)]
    pub fn is_256mb(&self) -> bool {
        *self == Banksizeselect::_256mb
    }
    #[doc = "512Mbytes"]
    #[inline(always)]
    pub fn is_512mb(&self) -> bool {
        *self == Banksizeselect::_512mb
    }
    #[doc = "NOT_USED"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Banksizeselect::NotUsed
    }
}
#[doc = "Field `BANKSIZE` writer - Bank Size"]
pub type BanksizeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Banksizeselect>;
impl<'a, REG> BanksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8Kbytes"]
    #[inline(always)]
    pub fn _8kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_8kb)
    }
    #[doc = "16Kbytes"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_16kb)
    }
    #[doc = "32Kbytes"]
    #[inline(always)]
    pub fn _32kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_32kb)
    }
    #[doc = "64Kbytes"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_64kb)
    }
    #[doc = "128Kbytes"]
    #[inline(always)]
    pub fn _128kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_128kb)
    }
    #[doc = "256Kbytes"]
    #[inline(always)]
    pub fn _256kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_256kb)
    }
    #[doc = "512Kbytes"]
    #[inline(always)]
    pub fn _512kb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_512kb)
    }
    #[doc = "1Mbytes"]
    #[inline(always)]
    pub fn _1mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_1mb)
    }
    #[doc = "2Mbytes"]
    #[inline(always)]
    pub fn _2mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_2mb)
    }
    #[doc = "4Mbytes"]
    #[inline(always)]
    pub fn _4mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_4mb)
    }
    #[doc = "8Mbytes"]
    #[inline(always)]
    pub fn _8mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_8mb)
    }
    #[doc = "16Mbytes"]
    #[inline(always)]
    pub fn _16mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_16mb)
    }
    #[doc = "32Mbytes"]
    #[inline(always)]
    pub fn _32mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_32mb)
    }
    #[doc = "64Mbytes"]
    #[inline(always)]
    pub fn _64mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_64mb)
    }
    #[doc = "128Mbytes"]
    #[inline(always)]
    pub fn _128mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_128mb)
    }
    #[doc = "256Mbytes (Default)"]
    #[inline(always)]
    pub fn _256mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_256mb)
    }
    #[doc = "512Mbytes"]
    #[inline(always)]
    pub fn _512mb(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::_512mb)
    }
    #[doc = "NOT_USED"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Banksizeselect::NotUsed)
    }
}
#[doc = "Field `TYPE` reader - type of memory used"]
pub type TypeR = crate::BitReader;
#[doc = "Field `TYPE` writer - type of memory used"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDBASE` reader - relative base address of NCS area"]
pub type AddbaseR = crate::FieldReader<u32>;
#[doc = "Field `ADDBASE` writer - relative base address of NCS area"]
pub type AddbaseW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `WRITE_ECC_CONF` reader - ECC Configuration Protection Enable"]
pub type WriteEccConfR = crate::BitReader;
#[doc = "Field `WRITE_ECC_CONF` writer - ECC Configuration Protection Enable"]
pub type WriteEccConfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ENABLE` reader - ECC Protection Enable"]
pub type EccEnableR = crate::BitReader;
#[doc = "Field `ECC_ENABLE` writer - ECC Protection Enable"]
pub type EccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC12_ENABLE` reader - BCH ECC Enable"]
pub type Ecc12EnableR = crate::BitReader;
#[doc = "Field `ECC12_ENABLE` writer - BCH ECC Enable"]
pub type Ecc12EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - fixed to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Bank Size"]
    #[inline(always)]
    pub fn banksize(&self) -> BanksizeR {
        BanksizeR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - type of memory used"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:24 - relative base address of NCS area"]
    #[inline(always)]
    pub fn addbase(&self) -> AddbaseR {
        AddbaseR::new((self.bits >> 7) & 0x0003_ffff)
    }
    #[doc = "Bit 29 - ECC Configuration Protection Enable"]
    #[inline(always)]
    pub fn write_ecc_conf(&self) -> WriteEccConfR {
        WriteEccConfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC Protection Enable"]
    #[inline(always)]
    pub fn ecc_enable(&self) -> EccEnableR {
        EccEnableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BCH ECC Enable"]
    #[inline(always)]
    pub fn ecc12_enable(&self) -> Ecc12EnableR {
        Ecc12EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - fixed to 0"]
    #[inline(always)]
    pub fn zero(&mut self) -> ZeroW<CrNcs0Spec> {
        ZeroW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Bank Size"]
    #[inline(always)]
    pub fn banksize(&mut self) -> BanksizeW<CrNcs0Spec> {
        BanksizeW::new(self, 1)
    }
    #[doc = "Bit 6 - type of memory used"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<CrNcs0Spec> {
        TypeW::new(self, 6)
    }
    #[doc = "Bits 7:24 - relative base address of NCS area"]
    #[inline(always)]
    pub fn addbase(&mut self) -> AddbaseW<CrNcs0Spec> {
        AddbaseW::new(self, 7)
    }
    #[doc = "Bit 29 - ECC Configuration Protection Enable"]
    #[inline(always)]
    pub fn write_ecc_conf(&mut self) -> WriteEccConfW<CrNcs0Spec> {
        WriteEccConfW::new(self, 29)
    }
    #[doc = "Bit 30 - ECC Protection Enable"]
    #[inline(always)]
    pub fn ecc_enable(&mut self) -> EccEnableW<CrNcs0Spec> {
        EccEnableW::new(self, 30)
    }
    #[doc = "Bit 31 - BCH ECC Enable"]
    #[inline(always)]
    pub fn ecc12_enable(&mut self) -> Ecc12EnableW<CrNcs0Spec> {
        Ecc12EnableW::new(self, 31)
    }
}
#[doc = "HEMC Control Register NCS 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrNcs0Spec;
impl crate::RegisterSpec for CrNcs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_ncs0::R`](R) reader structure"]
impl crate::Readable for CrNcs0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_ncs0::W`](W) writer structure"]
impl crate::Writable for CrNcs0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_NCS0 to value 0"]
impl crate::Resettable for CrNcs0Spec {}
