#[doc = "Register `GCSR2` reader"]
pub type R = crate::R<Gcsr2Spec>;
#[doc = "Field `GPID64` reader - Generic Clock 64 Status"]
pub type Gpid64R = crate::BitReader;
#[doc = "Field `GPID65` reader - Generic Clock 65 Status"]
pub type Gpid65R = crate::BitReader;
#[doc = "Field `GPID66` reader - Generic Clock 66 Status"]
pub type Gpid66R = crate::BitReader;
#[doc = "Field `GPID67` reader - Generic Clock 67 Status"]
pub type Gpid67R = crate::BitReader;
#[doc = "Field `GPID68` reader - Generic Clock 68 Status"]
pub type Gpid68R = crate::BitReader;
#[doc = "Field `GPID69` reader - Generic Clock 69 Status"]
pub type Gpid69R = crate::BitReader;
#[doc = "Field `GPID70` reader - Generic Clock 70 Status"]
pub type Gpid70R = crate::BitReader;
#[doc = "Field `GPID71` reader - Generic Clock 71 Status"]
pub type Gpid71R = crate::BitReader;
#[doc = "Field `GPID72` reader - Generic Clock 72 Status"]
pub type Gpid72R = crate::BitReader;
#[doc = "Field `GPID73` reader - Generic Clock 73 Status"]
pub type Gpid73R = crate::BitReader;
#[doc = "Field `GPID74` reader - Generic Clock 74 Status"]
pub type Gpid74R = crate::BitReader;
#[doc = "Field `GPID75` reader - Generic Clock 75 Status"]
pub type Gpid75R = crate::BitReader;
#[doc = "Field `GPID76` reader - Generic Clock 76 Status"]
pub type Gpid76R = crate::BitReader;
#[doc = "Field `GPID77` reader - Generic Clock 77 Status"]
pub type Gpid77R = crate::BitReader;
#[doc = "Field `GPID78` reader - Generic Clock 78 Status"]
pub type Gpid78R = crate::BitReader;
#[doc = "Field `GPID79` reader - Generic Clock 79 Status"]
pub type Gpid79R = crate::BitReader;
#[doc = "Field `GPID80` reader - Generic Clock 80 Status"]
pub type Gpid80R = crate::BitReader;
#[doc = "Field `GPID81` reader - Generic Clock 81 Status"]
pub type Gpid81R = crate::BitReader;
#[doc = "Field `GPID82` reader - Generic Clock 82 Status"]
pub type Gpid82R = crate::BitReader;
#[doc = "Field `GPID83` reader - Generic Clock 83 Status"]
pub type Gpid83R = crate::BitReader;
#[doc = "Field `GPID84` reader - Generic Clock 84 Status"]
pub type Gpid84R = crate::BitReader;
#[doc = "Field `GPID85` reader - Generic Clock 85 Status"]
pub type Gpid85R = crate::BitReader;
#[doc = "Field `GPID86` reader - Generic Clock 86 Status"]
pub type Gpid86R = crate::BitReader;
#[doc = "Field `GPID87` reader - Generic Clock 87 Status"]
pub type Gpid87R = crate::BitReader;
#[doc = "Field `GPID88` reader - Generic Clock 88 Status"]
pub type Gpid88R = crate::BitReader;
#[doc = "Field `GPID89` reader - Generic Clock 89 Status"]
pub type Gpid89R = crate::BitReader;
#[doc = "Field `GPID90` reader - Generic Clock 90 Status"]
pub type Gpid90R = crate::BitReader;
#[doc = "Field `GPID91` reader - Generic Clock 91 Status"]
pub type Gpid91R = crate::BitReader;
#[doc = "Field `GPID92` reader - Generic Clock 92 Status"]
pub type Gpid92R = crate::BitReader;
#[doc = "Field `GPID93` reader - Generic Clock 93 Status"]
pub type Gpid93R = crate::BitReader;
#[doc = "Field `GPID94` reader - Generic Clock 94 Status"]
pub type Gpid94R = crate::BitReader;
#[doc = "Field `GPID95` reader - Generic Clock 95 Status"]
pub type Gpid95R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic Clock 64 Status"]
    #[inline(always)]
    pub fn gpid64(&self) -> Gpid64R {
        Gpid64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic Clock 65 Status"]
    #[inline(always)]
    pub fn gpid65(&self) -> Gpid65R {
        Gpid65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock 66 Status"]
    #[inline(always)]
    pub fn gpid66(&self) -> Gpid66R {
        Gpid66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock 67 Status"]
    #[inline(always)]
    pub fn gpid67(&self) -> Gpid67R {
        Gpid67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock 68 Status"]
    #[inline(always)]
    pub fn gpid68(&self) -> Gpid68R {
        Gpid68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock 69 Status"]
    #[inline(always)]
    pub fn gpid69(&self) -> Gpid69R {
        Gpid69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock 70 Status"]
    #[inline(always)]
    pub fn gpid70(&self) -> Gpid70R {
        Gpid70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock 71 Status"]
    #[inline(always)]
    pub fn gpid71(&self) -> Gpid71R {
        Gpid71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock 72 Status"]
    #[inline(always)]
    pub fn gpid72(&self) -> Gpid72R {
        Gpid72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock 73 Status"]
    #[inline(always)]
    pub fn gpid73(&self) -> Gpid73R {
        Gpid73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock 74 Status"]
    #[inline(always)]
    pub fn gpid74(&self) -> Gpid74R {
        Gpid74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic Clock 75 Status"]
    #[inline(always)]
    pub fn gpid75(&self) -> Gpid75R {
        Gpid75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic Clock 76 Status"]
    #[inline(always)]
    pub fn gpid76(&self) -> Gpid76R {
        Gpid76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic Clock 77 Status"]
    #[inline(always)]
    pub fn gpid77(&self) -> Gpid77R {
        Gpid77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generic Clock 78 Status"]
    #[inline(always)]
    pub fn gpid78(&self) -> Gpid78R {
        Gpid78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock 79 Status"]
    #[inline(always)]
    pub fn gpid79(&self) -> Gpid79R {
        Gpid79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Generic Clock 80 Status"]
    #[inline(always)]
    pub fn gpid80(&self) -> Gpid80R {
        Gpid80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic Clock 81 Status"]
    #[inline(always)]
    pub fn gpid81(&self) -> Gpid81R {
        Gpid81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic Clock 82 Status"]
    #[inline(always)]
    pub fn gpid82(&self) -> Gpid82R {
        Gpid82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Generic Clock 83 Status"]
    #[inline(always)]
    pub fn gpid83(&self) -> Gpid83R {
        Gpid83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Generic Clock 84 Status"]
    #[inline(always)]
    pub fn gpid84(&self) -> Gpid84R {
        Gpid84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Generic Clock 85 Status"]
    #[inline(always)]
    pub fn gpid85(&self) -> Gpid85R {
        Gpid85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Generic Clock 86 Status"]
    #[inline(always)]
    pub fn gpid86(&self) -> Gpid86R {
        Gpid86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Generic Clock 87 Status"]
    #[inline(always)]
    pub fn gpid87(&self) -> Gpid87R {
        Gpid87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Generic Clock 88 Status"]
    #[inline(always)]
    pub fn gpid88(&self) -> Gpid88R {
        Gpid88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Generic Clock 89 Status"]
    #[inline(always)]
    pub fn gpid89(&self) -> Gpid89R {
        Gpid89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Generic Clock 90 Status"]
    #[inline(always)]
    pub fn gpid90(&self) -> Gpid90R {
        Gpid90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Generic Clock 91 Status"]
    #[inline(always)]
    pub fn gpid91(&self) -> Gpid91R {
        Gpid91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Generic Clock 92 Status"]
    #[inline(always)]
    pub fn gpid92(&self) -> Gpid92R {
        Gpid92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock 93 Status"]
    #[inline(always)]
    pub fn gpid93(&self) -> Gpid93R {
        Gpid93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Generic Clock 94 Status"]
    #[inline(always)]
    pub fn gpid94(&self) -> Gpid94R {
        Gpid94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Generic Clock 95 Status"]
    #[inline(always)]
    pub fn gpid95(&self) -> Gpid95R {
        Gpid95R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Generic Clock Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcsr2Spec;
impl crate::RegisterSpec for Gcsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcsr2::R`](R) reader structure"]
impl crate::Readable for Gcsr2Spec {}
#[doc = "`reset()` method sets GCSR2 to value 0"]
impl crate::Resettable for Gcsr2Spec {}
