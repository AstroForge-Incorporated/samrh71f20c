#[doc = "Register `GCSR1` reader"]
pub type R = crate::R<Gcsr1Spec>;
#[doc = "Field `GPID32` reader - Generic Clock 32 Status"]
pub type Gpid32R = crate::BitReader;
#[doc = "Field `GPID33` reader - Generic Clock 33 Status"]
pub type Gpid33R = crate::BitReader;
#[doc = "Field `GPID34` reader - Generic Clock 34 Status"]
pub type Gpid34R = crate::BitReader;
#[doc = "Field `GPID35` reader - Generic Clock 35 Status"]
pub type Gpid35R = crate::BitReader;
#[doc = "Field `GPID36` reader - Generic Clock 36 Status"]
pub type Gpid36R = crate::BitReader;
#[doc = "Field `GPID37` reader - Generic Clock 37 Status"]
pub type Gpid37R = crate::BitReader;
#[doc = "Field `GPID38` reader - Generic Clock 38 Status"]
pub type Gpid38R = crate::BitReader;
#[doc = "Field `GPID39` reader - Generic Clock 39 Status"]
pub type Gpid39R = crate::BitReader;
#[doc = "Field `GPID40` reader - Generic Clock 40 Status"]
pub type Gpid40R = crate::BitReader;
#[doc = "Field `GPID41` reader - Generic Clock 41 Status"]
pub type Gpid41R = crate::BitReader;
#[doc = "Field `GPID42` reader - Generic Clock 42 Status"]
pub type Gpid42R = crate::BitReader;
#[doc = "Field `GPID43` reader - Generic Clock 43 Status"]
pub type Gpid43R = crate::BitReader;
#[doc = "Field `GPID44` reader - Generic Clock 44 Status"]
pub type Gpid44R = crate::BitReader;
#[doc = "Field `GPID45` reader - Generic Clock 45 Status"]
pub type Gpid45R = crate::BitReader;
#[doc = "Field `GPID46` reader - Generic Clock 46 Status"]
pub type Gpid46R = crate::BitReader;
#[doc = "Field `GPID47` reader - Generic Clock 47 Status"]
pub type Gpid47R = crate::BitReader;
#[doc = "Field `GPID48` reader - Generic Clock 48 Status"]
pub type Gpid48R = crate::BitReader;
#[doc = "Field `GPID49` reader - Generic Clock 49 Status"]
pub type Gpid49R = crate::BitReader;
#[doc = "Field `GPID50` reader - Generic Clock 50 Status"]
pub type Gpid50R = crate::BitReader;
#[doc = "Field `GPID51` reader - Generic Clock 51 Status"]
pub type Gpid51R = crate::BitReader;
#[doc = "Field `GPID52` reader - Generic Clock 52 Status"]
pub type Gpid52R = crate::BitReader;
#[doc = "Field `GPID53` reader - Generic Clock 53 Status"]
pub type Gpid53R = crate::BitReader;
#[doc = "Field `GPID54` reader - Generic Clock 54 Status"]
pub type Gpid54R = crate::BitReader;
#[doc = "Field `GPID55` reader - Generic Clock 55 Status"]
pub type Gpid55R = crate::BitReader;
#[doc = "Field `GPID56` reader - Generic Clock 56 Status"]
pub type Gpid56R = crate::BitReader;
#[doc = "Field `GPID57` reader - Generic Clock 57 Status"]
pub type Gpid57R = crate::BitReader;
#[doc = "Field `GPID58` reader - Generic Clock 58 Status"]
pub type Gpid58R = crate::BitReader;
#[doc = "Field `GPID59` reader - Generic Clock 59 Status"]
pub type Gpid59R = crate::BitReader;
#[doc = "Field `GPID60` reader - Generic Clock 60 Status"]
pub type Gpid60R = crate::BitReader;
#[doc = "Field `GPID61` reader - Generic Clock 61 Status"]
pub type Gpid61R = crate::BitReader;
#[doc = "Field `GPID62` reader - Generic Clock 62 Status"]
pub type Gpid62R = crate::BitReader;
#[doc = "Field `GPID63` reader - Generic Clock 63 Status"]
pub type Gpid63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic Clock 32 Status"]
    #[inline(always)]
    pub fn gpid32(&self) -> Gpid32R {
        Gpid32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic Clock 33 Status"]
    #[inline(always)]
    pub fn gpid33(&self) -> Gpid33R {
        Gpid33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock 34 Status"]
    #[inline(always)]
    pub fn gpid34(&self) -> Gpid34R {
        Gpid34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock 35 Status"]
    #[inline(always)]
    pub fn gpid35(&self) -> Gpid35R {
        Gpid35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock 36 Status"]
    #[inline(always)]
    pub fn gpid36(&self) -> Gpid36R {
        Gpid36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock 37 Status"]
    #[inline(always)]
    pub fn gpid37(&self) -> Gpid37R {
        Gpid37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock 38 Status"]
    #[inline(always)]
    pub fn gpid38(&self) -> Gpid38R {
        Gpid38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock 39 Status"]
    #[inline(always)]
    pub fn gpid39(&self) -> Gpid39R {
        Gpid39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock 40 Status"]
    #[inline(always)]
    pub fn gpid40(&self) -> Gpid40R {
        Gpid40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock 41 Status"]
    #[inline(always)]
    pub fn gpid41(&self) -> Gpid41R {
        Gpid41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock 42 Status"]
    #[inline(always)]
    pub fn gpid42(&self) -> Gpid42R {
        Gpid42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic Clock 43 Status"]
    #[inline(always)]
    pub fn gpid43(&self) -> Gpid43R {
        Gpid43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic Clock 44 Status"]
    #[inline(always)]
    pub fn gpid44(&self) -> Gpid44R {
        Gpid44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic Clock 45 Status"]
    #[inline(always)]
    pub fn gpid45(&self) -> Gpid45R {
        Gpid45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generic Clock 46 Status"]
    #[inline(always)]
    pub fn gpid46(&self) -> Gpid46R {
        Gpid46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock 47 Status"]
    #[inline(always)]
    pub fn gpid47(&self) -> Gpid47R {
        Gpid47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Generic Clock 48 Status"]
    #[inline(always)]
    pub fn gpid48(&self) -> Gpid48R {
        Gpid48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic Clock 49 Status"]
    #[inline(always)]
    pub fn gpid49(&self) -> Gpid49R {
        Gpid49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic Clock 50 Status"]
    #[inline(always)]
    pub fn gpid50(&self) -> Gpid50R {
        Gpid50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Generic Clock 51 Status"]
    #[inline(always)]
    pub fn gpid51(&self) -> Gpid51R {
        Gpid51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Generic Clock 52 Status"]
    #[inline(always)]
    pub fn gpid52(&self) -> Gpid52R {
        Gpid52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Generic Clock 53 Status"]
    #[inline(always)]
    pub fn gpid53(&self) -> Gpid53R {
        Gpid53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Generic Clock 54 Status"]
    #[inline(always)]
    pub fn gpid54(&self) -> Gpid54R {
        Gpid54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Generic Clock 55 Status"]
    #[inline(always)]
    pub fn gpid55(&self) -> Gpid55R {
        Gpid55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Generic Clock 56 Status"]
    #[inline(always)]
    pub fn gpid56(&self) -> Gpid56R {
        Gpid56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Generic Clock 57 Status"]
    #[inline(always)]
    pub fn gpid57(&self) -> Gpid57R {
        Gpid57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Generic Clock 58 Status"]
    #[inline(always)]
    pub fn gpid58(&self) -> Gpid58R {
        Gpid58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Generic Clock 59 Status"]
    #[inline(always)]
    pub fn gpid59(&self) -> Gpid59R {
        Gpid59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Generic Clock 60 Status"]
    #[inline(always)]
    pub fn gpid60(&self) -> Gpid60R {
        Gpid60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock 61 Status"]
    #[inline(always)]
    pub fn gpid61(&self) -> Gpid61R {
        Gpid61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Generic Clock 62 Status"]
    #[inline(always)]
    pub fn gpid62(&self) -> Gpid62R {
        Gpid62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Generic Clock 63 Status"]
    #[inline(always)]
    pub fn gpid63(&self) -> Gpid63R {
        Gpid63R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Generic Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcsr1Spec;
impl crate::RegisterSpec for Gcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcsr1::R`](R) reader structure"]
impl crate::Readable for Gcsr1Spec {}
#[doc = "`reset()` method sets GCSR1 to value 0"]
impl crate::Resettable for Gcsr1Spec {}
