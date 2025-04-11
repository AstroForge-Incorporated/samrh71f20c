#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Field `IP1553DATA1` reader - IP1553 data value 1"]
pub type Ip1553data1R = crate::FieldReader<u16>;
#[doc = "Field `IP1553DATA2` reader - IP1553 data value 2"]
pub type Ip1553data2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IP1553 data value 1"]
    #[inline(always)]
    pub fn ip1553data1(&self) -> Ip1553data1R {
        Ip1553data1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IP1553 data value 2"]
    #[inline(always)]
    pub fn ip1553data2(&self) -> Ip1553data2R {
        Ip1553data2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
