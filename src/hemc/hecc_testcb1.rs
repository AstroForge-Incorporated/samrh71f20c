#[doc = "Register `HECC_TESTCB1` reader"]
pub type R = crate::R<HeccTestcb1Spec>;
#[doc = "Register `HECC_TESTCB1` writer"]
pub type W = crate::W<HeccTestcb1Spec>;
#[doc = "Field `TCB1` reader - test check bit (16 bit)"]
pub type Tcb1R = crate::FieldReader<u16>;
#[doc = "Field `TCB1` writer - test check bit (16 bit)"]
pub type Tcb1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - test check bit (16 bit)"]
    #[inline(always)]
    pub fn tcb1(&self) -> Tcb1R {
        Tcb1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - test check bit (16 bit)"]
    #[inline(always)]
    pub fn tcb1(&mut self) -> Tcb1W<HeccTestcb1Spec> {
        Tcb1W::new(self, 0)
    }
}
#[doc = "HECC Test mode Register Channel 1 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccTestcb1Spec;
impl crate::RegisterSpec for HeccTestcb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_testcb1::R`](R) reader structure"]
impl crate::Readable for HeccTestcb1Spec {}
#[doc = "`write(|w| ..)` method takes [`hecc_testcb1::W`](W) writer structure"]
impl crate::Writable for HeccTestcb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HECC_TESTCB1 to value 0"]
impl crate::Resettable for HeccTestcb1Spec {}
