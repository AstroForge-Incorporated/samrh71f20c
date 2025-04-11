#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SmmrSpec>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SmmrSpec>;
#[doc = "Field `CORESMUSEL` reader - Core Supply Monitor User Selection"]
pub type CoresmuselR = crate::BitReader;
#[doc = "Field `CORESMUSEL` writer - Core Supply Monitor User Selection"]
pub type CoresmuselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - Core Supply Monitor User Selection"]
    #[inline(always)]
    pub fn coresmusel(&self) -> CoresmuselR {
        CoresmuselR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Core Supply Monitor User Selection"]
    #[inline(always)]
    pub fn coresmusel(&mut self) -> CoresmuselW<SmmrSpec> {
        CoresmuselW::new(self, 23)
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmmrSpec;
impl crate::RegisterSpec for SmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SmmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SmmrSpec {}
