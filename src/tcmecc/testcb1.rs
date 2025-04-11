#[doc = "Register `TESTCB1` reader"]
pub type R = crate::R<Testcb1Spec>;
#[doc = "Register `TESTCB1` writer"]
pub type W = crate::W<Testcb1Spec>;
#[doc = "Field `TCB1` reader - test check bit (8 bit)"]
pub type Tcb1R = crate::FieldReader;
#[doc = "Field `TCB1` writer - test check bit (8 bit)"]
pub type Tcb1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCB2` reader - test check bit (8 bit)"]
pub type Tcb2R = crate::FieldReader;
#[doc = "Field `TCB2` writer - test check bit (8 bit)"]
pub type Tcb2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - test check bit (8 bit)"]
    #[inline(always)]
    pub fn tcb1(&self) -> Tcb1R {
        Tcb1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - test check bit (8 bit)"]
    #[inline(always)]
    pub fn tcb2(&self) -> Tcb2R {
        Tcb2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - test check bit (8 bit)"]
    #[inline(always)]
    pub fn tcb1(&mut self) -> Tcb1W<Testcb1Spec> {
        Tcb1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - test check bit (8 bit)"]
    #[inline(always)]
    pub fn tcb2(&mut self) -> Tcb2W<Testcb1Spec> {
        Tcb2W::new(self, 8)
    }
}
#[doc = "TCMECC Test mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`testcb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testcb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testcb1Spec;
impl crate::RegisterSpec for Testcb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testcb1::R`](R) reader structure"]
impl crate::Readable for Testcb1Spec {}
#[doc = "`write(|w| ..)` method takes [`testcb1::W`](W) writer structure"]
impl crate::Writable for Testcb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTCB1 to value 0"]
impl crate::Resettable for Testcb1Spec {}
