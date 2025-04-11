#[doc = "Register `TCH_CFGTCEVENT` reader"]
pub type R = crate::R<TchCfgtceventSpec>;
#[doc = "Register `TCH_CFGTCEVENT` writer"]
pub type W = crate::W<TchCfgtceventSpec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MASK` reader - Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<TchCfgtceventSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<TchCfgtceventSpec> {
        MaskW::new(self, 8)
    }
}
#[doc = "SpW Tch Config Tc Event\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgtcevent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgtcevent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfgtceventSpec;
impl crate::RegisterSpec for TchCfgtceventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfgtcevent::R`](R) reader structure"]
impl crate::Readable for TchCfgtceventSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfgtcevent::W`](W) writer structure"]
impl crate::Writable for TchCfgtceventSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFGTCEVENT to value 0"]
impl crate::Resettable for TchCfgtceventSpec {}
