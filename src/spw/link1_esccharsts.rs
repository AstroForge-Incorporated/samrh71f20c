#[doc = "Register `LINK1_ESCCHARSTS` reader"]
pub type R = crate::R<Link1EsccharstsSpec>;
#[doc = "Field `CHAR1` reader - Esc Char 1"]
pub type Char1R = crate::FieldReader;
#[doc = "Field `CHAR2` reader - Esc Char 2"]
pub type Char2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Esc Char 1"]
    #[inline(always)]
    pub fn char1(&self) -> Char1R {
        Char1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Esc Char 2"]
    #[inline(always)]
    pub fn char2(&self) -> Char2R {
        Char2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "SpW Link 1 Escape Character Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_esccharsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1EsccharstsSpec;
impl crate::RegisterSpec for Link1EsccharstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_esccharsts::R`](R) reader structure"]
impl crate::Readable for Link1EsccharstsSpec {}
#[doc = "`reset()` method sets LINK1_ESCCHARSTS to value 0"]
impl crate::Resettable for Link1EsccharstsSpec {}
