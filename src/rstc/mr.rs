#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `SCKSW` reader - Slow Clock Switching"]
pub type SckswR = crate::BitReader;
#[doc = "Field `SCKSW` writer - Slow Clock Switching"]
pub type SckswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUFEN` reader - CPU Fail Enable"]
pub type CpufenR = crate::BitReader;
#[doc = "Field `CPUFEN` writer - CPU Fail Enable"]
pub type CpufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "165: Writing any other value in this field aborts the write operation. Always reads as 0."]
    Passwd = 165,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            165 => Some(Keyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Keyselect::Passwd
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl R {
    #[doc = "Bit 1 - Slow Clock Switching"]
    #[inline(always)]
    pub fn scksw(&self) -> SckswR {
        SckswR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Fail Enable"]
    #[inline(always)]
    pub fn cpufen(&self) -> CpufenR {
        CpufenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Slow Clock Switching"]
    #[inline(always)]
    pub fn scksw(&mut self) -> SckswW<MrSpec> {
        SckswW::new(self, 1)
    }
    #[doc = "Bit 3 - CPU Fail Enable"]
    #[inline(always)]
    pub fn cpufen(&mut self) -> CpufenW<MrSpec> {
        CpufenW::new(self, 3)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
