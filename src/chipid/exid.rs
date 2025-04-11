#[doc = "Register `EXID` reader"]
pub type R = crate::R<ExidSpec>;
#[doc = "Chip ID Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Exidselect {
    #[doc = "1: 256-lead package"]
    PackageType256 = 1,
}
impl From<Exidselect> for u32 {
    #[inline(always)]
    fn from(variant: Exidselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exidselect {
    type Ux = u32;
}
impl crate::IsEnum for Exidselect {}
#[doc = "Field `EXID` reader - Chip ID Extension"]
pub type ExidR = crate::FieldReader<Exidselect>;
impl ExidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exidselect> {
        match self.bits {
            1 => Some(Exidselect::PackageType256),
            _ => None,
        }
    }
    #[doc = "256-lead package"]
    #[inline(always)]
    pub fn is_package_type_256(&self) -> bool {
        *self == Exidselect::PackageType256
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> ExidR {
        ExidR::new(self.bits)
    }
}
#[doc = "Chip ID Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`exid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExidSpec;
impl crate::RegisterSpec for ExidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exid::R`](R) reader structure"]
impl crate::Readable for ExidSpec {}
#[doc = "`reset()` method sets EXID to value 0"]
impl crate::Resettable for ExidSpec {}
