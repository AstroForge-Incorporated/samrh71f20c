#[doc = "Register `IODATAR11` reader"]
pub type R = crate::R<Iodatar11Spec>;
#[doc = "Register `IODATAR11` writer"]
pub type W = crate::W<Iodatar11Spec>;
#[doc = "Field `IODATA` reader - Input/Output Data"]
pub type IodataR = crate::FieldReader<u32>;
#[doc = "Field `IODATA` writer - Input/Output Data"]
pub type IodataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input/Output Data"]
    #[inline(always)]
    pub fn iodata(&self) -> IodataR {
        IodataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input/Output Data"]
    #[inline(always)]
    pub fn iodata(&mut self) -> IodataW<Iodatar11Spec> {
        IodataW::new(self, 0)
    }
}
#[doc = "Input/Output Data 0 Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iodatar11Spec;
impl crate::RegisterSpec for Iodatar11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iodatar11::R`](R) reader structure"]
impl crate::Readable for Iodatar11Spec {}
#[doc = "`write(|w| ..)` method takes [`iodatar11::W`](W) writer structure"]
impl crate::Writable for Iodatar11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IODATAR11 to value 0"]
impl crate::Resettable for Iodatar11Spec {}
