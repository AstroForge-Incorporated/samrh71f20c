#[doc = "Register `TR` reader"]
pub type R = crate::R<TrSpec>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TrSpec>;
#[doc = "Field `COUNT` reader - SDRAMC Refresh Timer Count"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - SDRAMC Refresh Timer Count"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<TrSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Refresh Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrSpec;
impl crate::RegisterSpec for TrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TrSpec {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TrSpec {}
