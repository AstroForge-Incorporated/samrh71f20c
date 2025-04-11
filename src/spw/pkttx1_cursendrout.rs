#[doc = "Register `PKTTX1_CURSENDROUT` reader"]
pub type R = crate::R<Pkttx1CursendroutSpec>;
#[doc = "Field `BYTE4` reader - Byte4"]
pub type Byte4R = crate::FieldReader;
#[doc = "Field `BYTE3` reader - Byte3"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE2` reader - Byte2"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE1` reader - Byte1"]
pub type Byte1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte4"]
    #[inline(always)]
    pub fn byte4(&self) -> Byte4R {
        Byte4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte3"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PktTx Current Send List Router Bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendrout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1CursendroutSpec;
impl crate::RegisterSpec for Pkttx1CursendroutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_cursendrout::R`](R) reader structure"]
impl crate::Readable for Pkttx1CursendroutSpec {}
#[doc = "`reset()` method sets PKTTX1_CURSENDROUT to value 0"]
impl crate::Resettable for Pkttx1CursendroutSpec {}
