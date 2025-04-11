#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `OUTOFRANGE` writer - out of range"]
pub type OutofrangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERRORACCESS` writer - read access error"]
pub type RderroraccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRERRORACCESS` writer - write access error"]
pub type WrerroraccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USERERRORACCESS` writer - user or superuser access error"]
pub type UsererroraccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - out of range"]
    #[inline(always)]
    pub fn outofrange(&mut self) -> OutofrangeW<IdrSpec> {
        OutofrangeW::new(self, 1)
    }
    #[doc = "Bit 2 - read access error"]
    #[inline(always)]
    pub fn rderroraccess(&mut self) -> RderroraccessW<IdrSpec> {
        RderroraccessW::new(self, 2)
    }
    #[doc = "Bit 3 - write access error"]
    #[inline(always)]
    pub fn wrerroraccess(&mut self) -> WrerroraccessW<IdrSpec> {
        WrerroraccessW::new(self, 3)
    }
    #[doc = "Bit 4 - user or superuser access error"]
    #[inline(always)]
    pub fn usererroraccess(&mut self) -> UsererroraccessW<IdrSpec> {
        UsererroraccessW::new(self, 4)
    }
}
#[doc = "HEMC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
