#[doc = "Register `GIS` reader"]
pub type R = crate::R<GisSpec>;
#[doc = "Field `IS0` reader - XDMAC Channel 0 Interrupt Status Bit"]
pub type Is0R = crate::BitReader;
#[doc = "Field `IS1` reader - XDMAC Channel 1 Interrupt Status Bit"]
pub type Is1R = crate::BitReader;
#[doc = "Field `IS2` reader - XDMAC Channel 2 Interrupt Status Bit"]
pub type Is2R = crate::BitReader;
#[doc = "Field `IS3` reader - XDMAC Channel 3 Interrupt Status Bit"]
pub type Is3R = crate::BitReader;
#[doc = "Field `IS4` reader - XDMAC Channel 4 Interrupt Status Bit"]
pub type Is4R = crate::BitReader;
#[doc = "Field `IS5` reader - XDMAC Channel 5 Interrupt Status Bit"]
pub type Is5R = crate::BitReader;
#[doc = "Field `IS6` reader - XDMAC Channel 6 Interrupt Status Bit"]
pub type Is6R = crate::BitReader;
#[doc = "Field `IS7` reader - XDMAC Channel 7 Interrupt Status Bit"]
pub type Is7R = crate::BitReader;
#[doc = "Field `IS8` reader - XDMAC Channel 8 Interrupt Status Bit"]
pub type Is8R = crate::BitReader;
#[doc = "Field `IS9` reader - XDMAC Channel 9 Interrupt Status Bit"]
pub type Is9R = crate::BitReader;
#[doc = "Field `IS10` reader - XDMAC Channel 10 Interrupt Status Bit"]
pub type Is10R = crate::BitReader;
#[doc = "Field `IS11` reader - XDMAC Channel 11 Interrupt Status Bit"]
pub type Is11R = crate::BitReader;
#[doc = "Field `IS12` reader - XDMAC Channel 12 Interrupt Status Bit"]
pub type Is12R = crate::BitReader;
#[doc = "Field `IS13` reader - XDMAC Channel 13 Interrupt Status Bit"]
pub type Is13R = crate::BitReader;
#[doc = "Field `IS14` reader - XDMAC Channel 14 Interrupt Status Bit"]
pub type Is14R = crate::BitReader;
#[doc = "Field `IS15` reader - XDMAC Channel 15 Interrupt Status Bit"]
pub type Is15R = crate::BitReader;
#[doc = "Field `IS16` reader - XDMAC Channel 16 Interrupt Status Bit"]
pub type Is16R = crate::BitReader;
#[doc = "Field `IS17` reader - XDMAC Channel 17 Interrupt Status Bit"]
pub type Is17R = crate::BitReader;
#[doc = "Field `IS18` reader - XDMAC Channel 18 Interrupt Status Bit"]
pub type Is18R = crate::BitReader;
#[doc = "Field `IS19` reader - XDMAC Channel 19 Interrupt Status Bit"]
pub type Is19R = crate::BitReader;
#[doc = "Field `IS20` reader - XDMAC Channel 20 Interrupt Status Bit"]
pub type Is20R = crate::BitReader;
#[doc = "Field `IS21` reader - XDMAC Channel 21 Interrupt Status Bit"]
pub type Is21R = crate::BitReader;
#[doc = "Field `IS22` reader - XDMAC Channel 22 Interrupt Status Bit"]
pub type Is22R = crate::BitReader;
#[doc = "Field `IS23` reader - XDMAC Channel 23 Interrupt Status Bit"]
pub type Is23R = crate::BitReader;
#[doc = "Field `IS24` reader - XDMAC Channel 24 Interrupt Status Bit"]
pub type Is24R = crate::BitReader;
#[doc = "Field `IS25` reader - XDMAC Channel 25 Interrupt Status Bit"]
pub type Is25R = crate::BitReader;
#[doc = "Field `IS26` reader - XDMAC Channel 26 Interrupt Status Bit"]
pub type Is26R = crate::BitReader;
#[doc = "Field `IS27` reader - XDMAC Channel 27 Interrupt Status Bit"]
pub type Is27R = crate::BitReader;
#[doc = "Field `IS28` reader - XDMAC Channel 28 Interrupt Status Bit"]
pub type Is28R = crate::BitReader;
#[doc = "Field `IS29` reader - XDMAC Channel 29 Interrupt Status Bit"]
pub type Is29R = crate::BitReader;
#[doc = "Field `IS30` reader - XDMAC Channel 30 Interrupt Status Bit"]
pub type Is30R = crate::BitReader;
#[doc = "Field `IS31` reader - XDMAC Channel 31 Interrupt Status Bit"]
pub type Is31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is0(&self) -> Is0R {
        Is0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is1(&self) -> Is1R {
        Is1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is2(&self) -> Is2R {
        Is2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is3(&self) -> Is3R {
        Is3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is4(&self) -> Is4R {
        Is4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is5(&self) -> Is5R {
        Is5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is6(&self) -> Is6R {
        Is6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is7(&self) -> Is7R {
        Is7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is8(&self) -> Is8R {
        Is8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is9(&self) -> Is9R {
        Is9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is10(&self) -> Is10R {
        Is10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is11(&self) -> Is11R {
        Is11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is12(&self) -> Is12R {
        Is12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is13(&self) -> Is13R {
        Is13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is14(&self) -> Is14R {
        Is14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is15(&self) -> Is15R {
        Is15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is16(&self) -> Is16R {
        Is16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is17(&self) -> Is17R {
        Is17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is18(&self) -> Is18R {
        Is18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is19(&self) -> Is19R {
        Is19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is20(&self) -> Is20R {
        Is20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is21(&self) -> Is21R {
        Is21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is22(&self) -> Is22R {
        Is22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is23(&self) -> Is23R {
        Is23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - XDMAC Channel 24 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is24(&self) -> Is24R {
        Is24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - XDMAC Channel 25 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is25(&self) -> Is25R {
        Is25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - XDMAC Channel 26 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is26(&self) -> Is26R {
        Is26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - XDMAC Channel 27 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is27(&self) -> Is27R {
        Is27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - XDMAC Channel 28 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is28(&self) -> Is28R {
        Is28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - XDMAC Channel 29 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is29(&self) -> Is29R {
        Is29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - XDMAC Channel 30 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is30(&self) -> Is30R {
        Is30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - XDMAC Channel 31 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is31(&self) -> Is31R {
        Is31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GisSpec;
impl crate::RegisterSpec for GisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gis::R`](R) reader structure"]
impl crate::Readable for GisSpec {}
#[doc = "`reset()` method sets GIS to value 0"]
impl crate::Resettable for GisSpec {}
