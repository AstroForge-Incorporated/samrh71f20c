#[doc = "Register `PKTRX1_CURBUFCFG` reader"]
pub type R = crate::R<Pktrx1CurbufcfgSpec>;
#[doc = "Register `PKTRX1_CURBUFCFG` writer"]
pub type W = crate::W<Pktrx1CurbufcfgSpec>;
#[doc = "Field `MAXCNT` reader - Max Count"]
pub type MaxcntR = crate::FieldReader<u16>;
#[doc = "Field `MAXCNT` writer - Max Count"]
pub type MaxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPLIT` reader - Split"]
pub type SplitR = crate::BitReader;
#[doc = "Field `SPLIT` writer - Split"]
pub type SplitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Abort"]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Max Count"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MaxcntR {
        MaxcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Split"]
    #[inline(always)]
    pub fn split(&self) -> SplitR {
        SplitR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Max Count"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MaxcntW<Pktrx1CurbufcfgSpec> {
        MaxcntW::new(self, 0)
    }
    #[doc = "Bit 30 - Split"]
    #[inline(always)]
    pub fn split(&mut self) -> SplitW<Pktrx1CurbufcfgSpec> {
        SplitW::new(self, 30)
    }
    #[doc = "Bit 31 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<Pktrx1CurbufcfgSpec> {
        AbortW::new(self, 31)
    }
}
#[doc = "PktRx Current Buffer Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_curbufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1CurbufcfgSpec;
impl crate::RegisterSpec for Pktrx1CurbufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_curbufcfg::R`](R) reader structure"]
impl crate::Readable for Pktrx1CurbufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_curbufcfg::W`](W) writer structure"]
impl crate::Writable for Pktrx1CurbufcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_CURBUFCFG to value 0"]
impl crate::Resettable for Pktrx1CurbufcfgSpec {}
