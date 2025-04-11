#[doc = "Register `GFCS` reader"]
pub type R = crate::R<GfcsSpec>;
#[doc = "Field `RDY0` reader - Filter 0 Configuration Ready"]
pub type Rdy0R = crate::BitReader;
#[doc = "Field `RDY1` reader - Filter 1 Configuration Ready"]
pub type Rdy1R = crate::BitReader;
#[doc = "Field `RDY2` reader - Filter 2 Configuration Ready"]
pub type Rdy2R = crate::BitReader;
#[doc = "Field `RDY3` reader - Filter 3 Configuration Ready"]
pub type Rdy3R = crate::BitReader;
#[doc = "Field `RDY4` reader - Filter 4 Configuration Ready"]
pub type Rdy4R = crate::BitReader;
#[doc = "Field `RDY5` reader - Filter 5 Configuration Ready"]
pub type Rdy5R = crate::BitReader;
#[doc = "Field `RDY6` reader - Filter 6 Configuration Ready"]
pub type Rdy6R = crate::BitReader;
#[doc = "Field `RDY7` reader - Filter 7 Configuration Ready"]
pub type Rdy7R = crate::BitReader;
#[doc = "Field `RDY8` reader - Filter 8 Configuration Ready"]
pub type Rdy8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Filter 0 Configuration Ready"]
    #[inline(always)]
    pub fn rdy0(&self) -> Rdy0R {
        Rdy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter 1 Configuration Ready"]
    #[inline(always)]
    pub fn rdy1(&self) -> Rdy1R {
        Rdy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter 2 Configuration Ready"]
    #[inline(always)]
    pub fn rdy2(&self) -> Rdy2R {
        Rdy2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter 3 Configuration Ready"]
    #[inline(always)]
    pub fn rdy3(&self) -> Rdy3R {
        Rdy3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter 4 Configuration Ready"]
    #[inline(always)]
    pub fn rdy4(&self) -> Rdy4R {
        Rdy4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter 5 Configuration Ready"]
    #[inline(always)]
    pub fn rdy5(&self) -> Rdy5R {
        Rdy5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter 6 Configuration Ready"]
    #[inline(always)]
    pub fn rdy6(&self) -> Rdy6R {
        Rdy6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter 7 Configuration Ready"]
    #[inline(always)]
    pub fn rdy7(&self) -> Rdy7R {
        Rdy7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter 8 Configuration Ready"]
    #[inline(always)]
    pub fn rdy8(&self) -> Rdy8R {
        Rdy8R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Glitch Filter Configuration Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gfcs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfcsSpec;
impl crate::RegisterSpec for GfcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfcs::R`](R) reader structure"]
impl crate::Readable for GfcsSpec {}
#[doc = "`reset()` method sets GFCS to value 0"]
impl crate::Resettable for GfcsSpec {}
