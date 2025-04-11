#[doc = "Register `PKTTX1_CURSENDCFG` reader"]
pub type R = crate::R<Pkttx1CursendcfgSpec>;
#[doc = "Register `PKTTX1_CURSENDCFG` writer"]
pub type W = crate::W<Pkttx1CursendcfgSpec>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader<u16>;
#[doc = "Field `LEN` writer - Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ABORT` reader - Abort"]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<Pkttx1CursendcfgSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 31 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<Pkttx1CursendcfgSpec> {
        AbortW::new(self, 31)
    }
}
#[doc = "PktTx Current Send List Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_cursendcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1CursendcfgSpec;
impl crate::RegisterSpec for Pkttx1CursendcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_cursendcfg::R`](R) reader structure"]
impl crate::Readable for Pkttx1CursendcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_cursendcfg::W`](W) writer structure"]
impl crate::Writable for Pkttx1CursendcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_CURSENDCFG to value 0"]
impl crate::Resettable for Pkttx1CursendcfgSpec {}
