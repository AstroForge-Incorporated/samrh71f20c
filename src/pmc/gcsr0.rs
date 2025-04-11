#[doc = "Register `GCSR0` reader"]
pub type R = crate::R<Gcsr0Spec>;
#[doc = "Field `GPID0` reader - Generic Clock 0 Status"]
pub type Gpid0R = crate::BitReader;
#[doc = "Field `GPID1` reader - Generic Clock 1 Status"]
pub type Gpid1R = crate::BitReader;
#[doc = "Field `GPID2` reader - Generic Clock 2 Status"]
pub type Gpid2R = crate::BitReader;
#[doc = "Field `GPID3` reader - Generic Clock 3 Status"]
pub type Gpid3R = crate::BitReader;
#[doc = "Field `GPID4` reader - Generic Clock 4 Status"]
pub type Gpid4R = crate::BitReader;
#[doc = "Field `GPID5` reader - Generic Clock 5 Status"]
pub type Gpid5R = crate::BitReader;
#[doc = "Field `GPID6` reader - Generic Clock 6 Status"]
pub type Gpid6R = crate::BitReader;
#[doc = "Field `GPID7` reader - Generic Clock 7 Status"]
pub type Gpid7R = crate::BitReader;
#[doc = "Field `GPID8` reader - Generic Clock 8 Status"]
pub type Gpid8R = crate::BitReader;
#[doc = "Field `GPID9` reader - Generic Clock 9 Status"]
pub type Gpid9R = crate::BitReader;
#[doc = "Field `GPID10` reader - Generic Clock 10 Status"]
pub type Gpid10R = crate::BitReader;
#[doc = "Field `GPID11` reader - Generic Clock 11 Status"]
pub type Gpid11R = crate::BitReader;
#[doc = "Field `GPID12` reader - Generic Clock 12 Status"]
pub type Gpid12R = crate::BitReader;
#[doc = "Field `GPID13` reader - Generic Clock 13 Status"]
pub type Gpid13R = crate::BitReader;
#[doc = "Field `GPID14` reader - Generic Clock 14 Status"]
pub type Gpid14R = crate::BitReader;
#[doc = "Field `GPID15` reader - Generic Clock 15 Status"]
pub type Gpid15R = crate::BitReader;
#[doc = "Field `GPID16` reader - Generic Clock 16 Status"]
pub type Gpid16R = crate::BitReader;
#[doc = "Field `GPID17` reader - Generic Clock 17 Status"]
pub type Gpid17R = crate::BitReader;
#[doc = "Field `GPID18` reader - Generic Clock 18 Status"]
pub type Gpid18R = crate::BitReader;
#[doc = "Field `GPID19` reader - Generic Clock 19 Status"]
pub type Gpid19R = crate::BitReader;
#[doc = "Field `GPID20` reader - Generic Clock 20 Status"]
pub type Gpid20R = crate::BitReader;
#[doc = "Field `GPID21` reader - Generic Clock 21 Status"]
pub type Gpid21R = crate::BitReader;
#[doc = "Field `GPID22` reader - Generic Clock 22 Status"]
pub type Gpid22R = crate::BitReader;
#[doc = "Field `GPID23` reader - Generic Clock 23 Status"]
pub type Gpid23R = crate::BitReader;
#[doc = "Field `GPID24` reader - Generic Clock 24 Status"]
pub type Gpid24R = crate::BitReader;
#[doc = "Field `GPID25` reader - Generic Clock 25 Status"]
pub type Gpid25R = crate::BitReader;
#[doc = "Field `GPID26` reader - Generic Clock 26 Status"]
pub type Gpid26R = crate::BitReader;
#[doc = "Field `GPID27` reader - Generic Clock 27 Status"]
pub type Gpid27R = crate::BitReader;
#[doc = "Field `GPID28` reader - Generic Clock 28 Status"]
pub type Gpid28R = crate::BitReader;
#[doc = "Field `GPID29` reader - Generic Clock 29 Status"]
pub type Gpid29R = crate::BitReader;
#[doc = "Field `GPID30` reader - Generic Clock 30 Status"]
pub type Gpid30R = crate::BitReader;
#[doc = "Field `GPID31` reader - Generic Clock 31 Status"]
pub type Gpid31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic Clock 0 Status"]
    #[inline(always)]
    pub fn gpid0(&self) -> Gpid0R {
        Gpid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic Clock 1 Status"]
    #[inline(always)]
    pub fn gpid1(&self) -> Gpid1R {
        Gpid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock 2 Status"]
    #[inline(always)]
    pub fn gpid2(&self) -> Gpid2R {
        Gpid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock 3 Status"]
    #[inline(always)]
    pub fn gpid3(&self) -> Gpid3R {
        Gpid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock 4 Status"]
    #[inline(always)]
    pub fn gpid4(&self) -> Gpid4R {
        Gpid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock 5 Status"]
    #[inline(always)]
    pub fn gpid5(&self) -> Gpid5R {
        Gpid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock 6 Status"]
    #[inline(always)]
    pub fn gpid6(&self) -> Gpid6R {
        Gpid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock 7 Status"]
    #[inline(always)]
    pub fn gpid7(&self) -> Gpid7R {
        Gpid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock 8 Status"]
    #[inline(always)]
    pub fn gpid8(&self) -> Gpid8R {
        Gpid8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock 9 Status"]
    #[inline(always)]
    pub fn gpid9(&self) -> Gpid9R {
        Gpid9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock 10 Status"]
    #[inline(always)]
    pub fn gpid10(&self) -> Gpid10R {
        Gpid10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic Clock 11 Status"]
    #[inline(always)]
    pub fn gpid11(&self) -> Gpid11R {
        Gpid11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic Clock 12 Status"]
    #[inline(always)]
    pub fn gpid12(&self) -> Gpid12R {
        Gpid12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic Clock 13 Status"]
    #[inline(always)]
    pub fn gpid13(&self) -> Gpid13R {
        Gpid13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generic Clock 14 Status"]
    #[inline(always)]
    pub fn gpid14(&self) -> Gpid14R {
        Gpid14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock 15 Status"]
    #[inline(always)]
    pub fn gpid15(&self) -> Gpid15R {
        Gpid15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Generic Clock 16 Status"]
    #[inline(always)]
    pub fn gpid16(&self) -> Gpid16R {
        Gpid16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic Clock 17 Status"]
    #[inline(always)]
    pub fn gpid17(&self) -> Gpid17R {
        Gpid17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic Clock 18 Status"]
    #[inline(always)]
    pub fn gpid18(&self) -> Gpid18R {
        Gpid18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Generic Clock 19 Status"]
    #[inline(always)]
    pub fn gpid19(&self) -> Gpid19R {
        Gpid19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Generic Clock 20 Status"]
    #[inline(always)]
    pub fn gpid20(&self) -> Gpid20R {
        Gpid20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Generic Clock 21 Status"]
    #[inline(always)]
    pub fn gpid21(&self) -> Gpid21R {
        Gpid21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Generic Clock 22 Status"]
    #[inline(always)]
    pub fn gpid22(&self) -> Gpid22R {
        Gpid22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Generic Clock 23 Status"]
    #[inline(always)]
    pub fn gpid23(&self) -> Gpid23R {
        Gpid23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Generic Clock 24 Status"]
    #[inline(always)]
    pub fn gpid24(&self) -> Gpid24R {
        Gpid24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Generic Clock 25 Status"]
    #[inline(always)]
    pub fn gpid25(&self) -> Gpid25R {
        Gpid25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Generic Clock 26 Status"]
    #[inline(always)]
    pub fn gpid26(&self) -> Gpid26R {
        Gpid26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Generic Clock 27 Status"]
    #[inline(always)]
    pub fn gpid27(&self) -> Gpid27R {
        Gpid27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Generic Clock 28 Status"]
    #[inline(always)]
    pub fn gpid28(&self) -> Gpid28R {
        Gpid28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock 29 Status"]
    #[inline(always)]
    pub fn gpid29(&self) -> Gpid29R {
        Gpid29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Generic Clock 30 Status"]
    #[inline(always)]
    pub fn gpid30(&self) -> Gpid30R {
        Gpid30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Generic Clock 31 Status"]
    #[inline(always)]
    pub fn gpid31(&self) -> Gpid31R {
        Gpid31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Generic Clock Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcsr0Spec;
impl crate::RegisterSpec for Gcsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcsr0::R`](R) reader structure"]
impl crate::Readable for Gcsr0Spec {}
#[doc = "`reset()` method sets GCSR0 to value 0"]
impl crate::Resettable for Gcsr0Spec {}
