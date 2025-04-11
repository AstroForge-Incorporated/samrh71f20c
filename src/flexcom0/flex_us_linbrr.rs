#[doc = "Register `FLEX_US_LINBRR` reader"]
pub type R = crate::R<FlexUsLinbrrSpec>;
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub type LincdR = crate::FieldReader<u16>;
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub type LinfpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LincdR {
        LincdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LinfpR {
        LinfpR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "USART LIN Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_linbrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLinbrrSpec;
impl crate::RegisterSpec for FlexUsLinbrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_linbrr::R`](R) reader structure"]
impl crate::Readable for FlexUsLinbrrSpec {}
#[doc = "`reset()` method sets FLEX_US_LINBRR to value 0"]
impl crate::Resettable for FlexUsLinbrrSpec {}
