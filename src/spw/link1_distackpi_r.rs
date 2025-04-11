#[doc = "Register `LINK1_DISTACKPI_R` reader"]
pub type R = crate::R<Link1DistackpiRSpec>;
#[doc = "Field `DA0` reader - Distributed Acknowledge"]
pub type Da0R = crate::BitReader;
#[doc = "Field `DA1` reader - Distributed Acknowledge"]
pub type Da1R = crate::BitReader;
#[doc = "Field `DA2` reader - Distributed Acknowledge"]
pub type Da2R = crate::BitReader;
#[doc = "Field `DA3` reader - Distributed Acknowledge"]
pub type Da3R = crate::BitReader;
#[doc = "Field `DA4` reader - Distributed Acknowledge"]
pub type Da4R = crate::BitReader;
#[doc = "Field `DA5` reader - Distributed Acknowledge"]
pub type Da5R = crate::BitReader;
#[doc = "Field `DA6` reader - Distributed Acknowledge"]
pub type Da6R = crate::BitReader;
#[doc = "Field `DA7` reader - Distributed Acknowledge"]
pub type Da7R = crate::BitReader;
#[doc = "Field `DA8` reader - Distributed Acknowledge"]
pub type Da8R = crate::BitReader;
#[doc = "Field `DA9` reader - Distributed Acknowledge"]
pub type Da9R = crate::BitReader;
#[doc = "Field `DA10` reader - Distributed Acknowledge"]
pub type Da10R = crate::BitReader;
#[doc = "Field `DA11` reader - Distributed Acknowledge"]
pub type Da11R = crate::BitReader;
#[doc = "Field `DA12` reader - Distributed Acknowledge"]
pub type Da12R = crate::BitReader;
#[doc = "Field `DA13` reader - Distributed Acknowledge"]
pub type Da13R = crate::BitReader;
#[doc = "Field `DA14` reader - Distributed Acknowledge"]
pub type Da14R = crate::BitReader;
#[doc = "Field `DA15` reader - Distributed Acknowledge"]
pub type Da15R = crate::BitReader;
#[doc = "Field `DA16` reader - Distributed Acknowledge"]
pub type Da16R = crate::BitReader;
#[doc = "Field `DA17` reader - Distributed Acknowledge"]
pub type Da17R = crate::BitReader;
#[doc = "Field `DA18` reader - Distributed Acknowledge"]
pub type Da18R = crate::BitReader;
#[doc = "Field `DA19` reader - Distributed Acknowledge"]
pub type Da19R = crate::BitReader;
#[doc = "Field `DA20` reader - Distributed Acknowledge"]
pub type Da20R = crate::BitReader;
#[doc = "Field `DA21` reader - Distributed Acknowledge"]
pub type Da21R = crate::BitReader;
#[doc = "Field `DA22` reader - Distributed Acknowledge"]
pub type Da22R = crate::BitReader;
#[doc = "Field `DA23` reader - Distributed Acknowledge"]
pub type Da23R = crate::BitReader;
#[doc = "Field `DA24` reader - Distributed Acknowledge"]
pub type Da24R = crate::BitReader;
#[doc = "Field `DA25` reader - Distributed Acknowledge"]
pub type Da25R = crate::BitReader;
#[doc = "Field `DA26` reader - Distributed Acknowledge"]
pub type Da26R = crate::BitReader;
#[doc = "Field `DA27` reader - Distributed Acknowledge"]
pub type Da27R = crate::BitReader;
#[doc = "Field `DA28` reader - Distributed Acknowledge"]
pub type Da28R = crate::BitReader;
#[doc = "Field `DA29` reader - Distributed Acknowledge"]
pub type Da29R = crate::BitReader;
#[doc = "Field `DA30` reader - Distributed Acknowledge"]
pub type Da30R = crate::BitReader;
#[doc = "Field `DA31` reader - Distributed Acknowledge"]
pub type Da31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da0(&self) -> Da0R {
        Da0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da1(&self) -> Da1R {
        Da1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da2(&self) -> Da2R {
        Da2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da3(&self) -> Da3R {
        Da3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da4(&self) -> Da4R {
        Da4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da5(&self) -> Da5R {
        Da5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da6(&self) -> Da6R {
        Da6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da7(&self) -> Da7R {
        Da7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da8(&self) -> Da8R {
        Da8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da9(&self) -> Da9R {
        Da9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da10(&self) -> Da10R {
        Da10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da11(&self) -> Da11R {
        Da11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da12(&self) -> Da12R {
        Da12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da13(&self) -> Da13R {
        Da13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da14(&self) -> Da14R {
        Da14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da15(&self) -> Da15R {
        Da15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da16(&self) -> Da16R {
        Da16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da17(&self) -> Da17R {
        Da17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da18(&self) -> Da18R {
        Da18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da19(&self) -> Da19R {
        Da19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da20(&self) -> Da20R {
        Da20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da21(&self) -> Da21R {
        Da21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da22(&self) -> Da22R {
        Da22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da23(&self) -> Da23R {
        Da23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da24(&self) -> Da24R {
        Da24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da25(&self) -> Da25R {
        Da25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da26(&self) -> Da26R {
        Da26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da27(&self) -> Da27R {
        Da27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da28(&self) -> Da28R {
        Da28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da29(&self) -> Da29R {
        Da29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da30(&self) -> Da30R {
        Da30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Distributed Acknowledge"]
    #[inline(always)]
    pub fn da31(&self) -> Da31R {
        Da31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackpi_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1DistackpiRSpec;
impl crate::RegisterSpec for Link1DistackpiRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_distackpi_r::R`](R) reader structure"]
impl crate::Readable for Link1DistackpiRSpec {}
#[doc = "`reset()` method sets LINK1_DISTACKPI_R to value 0"]
impl crate::Resettable for Link1DistackpiRSpec {}
