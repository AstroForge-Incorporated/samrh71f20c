#[doc = "Register `ROUTER_STS` reader"]
pub type R = crate::R<RouterStsSpec>;
#[doc = "Field `DEST` reader - Destination addr"]
pub type DestR = crate::FieldReader;
#[doc = "Field `SOURCE` reader - Source address"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `BYTE` reader - Router byte"]
pub type ByteR = crate::FieldReader;
#[doc = "Field `COUNT` reader - Packet Count"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Destination addr"]
    #[inline(always)]
    pub fn dest(&self) -> DestR {
        DestR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Source address"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Router byte"]
    #[inline(always)]
    pub fn byte(&self) -> ByteR {
        ByteR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Packet Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "SpW Router Status\n\nYou can [`read`](crate::Reg::read) this register and get [`router_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouterStsSpec;
impl crate::RegisterSpec for RouterStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`router_sts::R`](R) reader structure"]
impl crate::Readable for RouterStsSpec {}
#[doc = "`reset()` method sets ROUTER_STS to value 0"]
impl crate::Resettable for RouterStsSpec {}
