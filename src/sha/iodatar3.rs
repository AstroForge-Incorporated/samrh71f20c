#[doc = "Register `IODATAR3` reader"]
pub type R = crate::R<Iodatar3Spec>;
#[doc = "Register `IODATAR3` writer"]
pub type W = crate::W<Iodatar3Spec>;
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
    pub fn iodata(&mut self) -> IodataW<Iodatar3Spec> {
        IodataW::new(self, 0)
    }
}
#[doc = "Input/Output Data 0 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iodatar3Spec;
impl crate::RegisterSpec for Iodatar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iodatar3::R`](R) reader structure"]
impl crate::Readable for Iodatar3Spec {}
#[doc = "`write(|w| ..)` method takes [`iodatar3::W`](W) writer structure"]
impl crate::Writable for Iodatar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IODATAR3 to value 0"]
impl crate::Resettable for Iodatar3Spec {}
