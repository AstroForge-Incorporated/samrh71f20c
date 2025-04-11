#[doc = "Register `HECC_TESTCB5` reader"]
pub type R = crate::R<HeccTestcb5Spec>;
#[doc = "Register `HECC_TESTCB5` writer"]
pub type W = crate::W<HeccTestcb5Spec>;
#[doc = "Field `TCB` reader - test check bit (16 bit)"]
pub type TcbR = crate::FieldReader<u16>;
#[doc = "Field `TCB` writer - test check bit (16 bit)"]
pub type TcbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - test check bit (16 bit)"]
    #[inline(always)]
    pub fn tcb(&self) -> TcbR {
        TcbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - test check bit (16 bit)"]
    #[inline(always)]
    pub fn tcb(&mut self) -> TcbW<HeccTestcb5Spec> {
        TcbW::new(self, 0)
    }
}
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccTestcb5Spec;
impl crate::RegisterSpec for HeccTestcb5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_testcb5::R`](R) reader structure"]
impl crate::Readable for HeccTestcb5Spec {}
#[doc = "`write(|w| ..)` method takes [`hecc_testcb5::W`](W) writer structure"]
impl crate::Writable for HeccTestcb5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HECC_TESTCB5 to value 0"]
impl crate::Resettable for HeccTestcb5Spec {}
