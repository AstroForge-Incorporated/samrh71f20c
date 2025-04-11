#[doc = "Register `LINK2_DISTINTPI_RM` reader"]
pub type R = crate::R<Link2DistintpiRmSpec>;
#[doc = "Field `DI0` reader - Distributed Interrupt"]
pub type Di0R = crate::BitReader;
#[doc = "Field `DI1` reader - Distributed Interrupt"]
pub type Di1R = crate::BitReader;
#[doc = "Field `DI2` reader - Distributed Interrupt"]
pub type Di2R = crate::BitReader;
#[doc = "Field `DI3` reader - Distributed Interrupt"]
pub type Di3R = crate::BitReader;
#[doc = "Field `DI4` reader - Distributed Interrupt"]
pub type Di4R = crate::BitReader;
#[doc = "Field `DI5` reader - Distributed Interrupt"]
pub type Di5R = crate::BitReader;
#[doc = "Field `DI6` reader - Distributed Interrupt"]
pub type Di6R = crate::BitReader;
#[doc = "Field `DI7` reader - Distributed Interrupt"]
pub type Di7R = crate::BitReader;
#[doc = "Field `DI8` reader - Distributed Interrupt"]
pub type Di8R = crate::BitReader;
#[doc = "Field `DI9` reader - Distributed Interrupt"]
pub type Di9R = crate::BitReader;
#[doc = "Field `DI10` reader - Distributed Interrupt"]
pub type Di10R = crate::BitReader;
#[doc = "Field `DI11` reader - Distributed Interrupt"]
pub type Di11R = crate::BitReader;
#[doc = "Field `DI12` reader - Distributed Interrupt"]
pub type Di12R = crate::BitReader;
#[doc = "Field `DI13` reader - Distributed Interrupt"]
pub type Di13R = crate::BitReader;
#[doc = "Field `DI14` reader - Distributed Interrupt"]
pub type Di14R = crate::BitReader;
#[doc = "Field `DI15` reader - Distributed Interrupt"]
pub type Di15R = crate::BitReader;
#[doc = "Field `DI16` reader - Distributed Interrupt"]
pub type Di16R = crate::BitReader;
#[doc = "Field `DI17` reader - Distributed Interrupt"]
pub type Di17R = crate::BitReader;
#[doc = "Field `DI18` reader - Distributed Interrupt"]
pub type Di18R = crate::BitReader;
#[doc = "Field `DI19` reader - Distributed Interrupt"]
pub type Di19R = crate::BitReader;
#[doc = "Field `DI20` reader - Distributed Interrupt"]
pub type Di20R = crate::BitReader;
#[doc = "Field `DI21` reader - Distributed Interrupt"]
pub type Di21R = crate::BitReader;
#[doc = "Field `DI22` reader - Distributed Interrupt"]
pub type Di22R = crate::BitReader;
#[doc = "Field `DI23` reader - Distributed Interrupt"]
pub type Di23R = crate::BitReader;
#[doc = "Field `DI24` reader - Distributed Interrupt"]
pub type Di24R = crate::BitReader;
#[doc = "Field `DI25` reader - Distributed Interrupt"]
pub type Di25R = crate::BitReader;
#[doc = "Field `DI26` reader - Distributed Interrupt"]
pub type Di26R = crate::BitReader;
#[doc = "Field `DI27` reader - Distributed Interrupt"]
pub type Di27R = crate::BitReader;
#[doc = "Field `DI28` reader - Distributed Interrupt"]
pub type Di28R = crate::BitReader;
#[doc = "Field `DI29` reader - Distributed Interrupt"]
pub type Di29R = crate::BitReader;
#[doc = "Field `DI30` reader - Distributed Interrupt"]
pub type Di30R = crate::BitReader;
#[doc = "Field `DI31` reader - Distributed Interrupt"]
pub type Di31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di0(&self) -> Di0R {
        Di0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di1(&self) -> Di1R {
        Di1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di2(&self) -> Di2R {
        Di2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di3(&self) -> Di3R {
        Di3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di4(&self) -> Di4R {
        Di4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di5(&self) -> Di5R {
        Di5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di6(&self) -> Di6R {
        Di6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di7(&self) -> Di7R {
        Di7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di8(&self) -> Di8R {
        Di8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di9(&self) -> Di9R {
        Di9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di10(&self) -> Di10R {
        Di10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di11(&self) -> Di11R {
        Di11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di12(&self) -> Di12R {
        Di12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di13(&self) -> Di13R {
        Di13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di14(&self) -> Di14R {
        Di14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di15(&self) -> Di15R {
        Di15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di16(&self) -> Di16R {
        Di16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di17(&self) -> Di17R {
        Di17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di18(&self) -> Di18R {
        Di18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di19(&self) -> Di19R {
        Di19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di20(&self) -> Di20R {
        Di20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di21(&self) -> Di21R {
        Di21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di22(&self) -> Di22R {
        Di22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di23(&self) -> Di23R {
        Di23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di24(&self) -> Di24R {
        Di24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di25(&self) -> Di25R {
        Di25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di26(&self) -> Di26R {
        Di26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di27(&self) -> Di27R {
        Di27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di28(&self) -> Di28R {
        Di28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di29(&self) -> Di29R {
        Di29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di30(&self) -> Di30R {
        Di30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di31(&self) -> Di31R {
        Di31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SpW Link 2 Distributed Interrupt Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintpi_rm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2DistintpiRmSpec;
impl crate::RegisterSpec for Link2DistintpiRmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link2_distintpi_rm::R`](R) reader structure"]
impl crate::Readable for Link2DistintpiRmSpec {}
#[doc = "`reset()` method sets LINK2_DISTINTPI_RM to value 0"]
impl crate::Resettable for Link2DistintpiRmSpec {}
