#[doc = "Register `CSR2` reader"]
pub type R = crate::R<Csr2Spec>;
#[doc = "Field `PID64` reader - Peripheral Clock 64 Status"]
pub type Pid64R = crate::BitReader;
#[doc = "Field `PID65` reader - Peripheral Clock 65 Status"]
pub type Pid65R = crate::BitReader;
#[doc = "Field `PID66` reader - Peripheral Clock 66 Status"]
pub type Pid66R = crate::BitReader;
#[doc = "Field `PID67` reader - Peripheral Clock 67 Status"]
pub type Pid67R = crate::BitReader;
#[doc = "Field `PID68` reader - Peripheral Clock 68 Status"]
pub type Pid68R = crate::BitReader;
#[doc = "Field `PID69` reader - Peripheral Clock 69 Status"]
pub type Pid69R = crate::BitReader;
#[doc = "Field `PID70` reader - Peripheral Clock 70 Status"]
pub type Pid70R = crate::BitReader;
#[doc = "Field `PID71` reader - Peripheral Clock 71 Status"]
pub type Pid71R = crate::BitReader;
#[doc = "Field `PID72` reader - Peripheral Clock 72 Status"]
pub type Pid72R = crate::BitReader;
#[doc = "Field `PID73` reader - Peripheral Clock 73 Status"]
pub type Pid73R = crate::BitReader;
#[doc = "Field `PID74` reader - Peripheral Clock 74 Status"]
pub type Pid74R = crate::BitReader;
#[doc = "Field `PID75` reader - Peripheral Clock 75 Status"]
pub type Pid75R = crate::BitReader;
#[doc = "Field `PID76` reader - Peripheral Clock 76 Status"]
pub type Pid76R = crate::BitReader;
#[doc = "Field `PID77` reader - Peripheral Clock 77 Status"]
pub type Pid77R = crate::BitReader;
#[doc = "Field `PID78` reader - Peripheral Clock 78 Status"]
pub type Pid78R = crate::BitReader;
#[doc = "Field `PID79` reader - Peripheral Clock 79 Status"]
pub type Pid79R = crate::BitReader;
#[doc = "Field `PID80` reader - Peripheral Clock 80 Status"]
pub type Pid80R = crate::BitReader;
#[doc = "Field `PID81` reader - Peripheral Clock 81 Status"]
pub type Pid81R = crate::BitReader;
#[doc = "Field `PID82` reader - Peripheral Clock 82 Status"]
pub type Pid82R = crate::BitReader;
#[doc = "Field `PID83` reader - Peripheral Clock 83 Status"]
pub type Pid83R = crate::BitReader;
#[doc = "Field `PID84` reader - Peripheral Clock 84 Status"]
pub type Pid84R = crate::BitReader;
#[doc = "Field `PID85` reader - Peripheral Clock 85 Status"]
pub type Pid85R = crate::BitReader;
#[doc = "Field `PID86` reader - Peripheral Clock 86 Status"]
pub type Pid86R = crate::BitReader;
#[doc = "Field `PID87` reader - Peripheral Clock 87 Status"]
pub type Pid87R = crate::BitReader;
#[doc = "Field `PID88` reader - Peripheral Clock 88 Status"]
pub type Pid88R = crate::BitReader;
#[doc = "Field `PID89` reader - Peripheral Clock 89 Status"]
pub type Pid89R = crate::BitReader;
#[doc = "Field `PID90` reader - Peripheral Clock 90 Status"]
pub type Pid90R = crate::BitReader;
#[doc = "Field `PID91` reader - Peripheral Clock 91 Status"]
pub type Pid91R = crate::BitReader;
#[doc = "Field `PID92` reader - Peripheral Clock 92 Status"]
pub type Pid92R = crate::BitReader;
#[doc = "Field `PID93` reader - Peripheral Clock 93 Status"]
pub type Pid93R = crate::BitReader;
#[doc = "Field `PID94` reader - Peripheral Clock 94 Status"]
pub type Pid94R = crate::BitReader;
#[doc = "Field `PID95` reader - Peripheral Clock 95 Status"]
pub type Pid95R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 64 Status"]
    #[inline(always)]
    pub fn pid64(&self) -> Pid64R {
        Pid64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 65 Status"]
    #[inline(always)]
    pub fn pid65(&self) -> Pid65R {
        Pid65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 66 Status"]
    #[inline(always)]
    pub fn pid66(&self) -> Pid66R {
        Pid66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 67 Status"]
    #[inline(always)]
    pub fn pid67(&self) -> Pid67R {
        Pid67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock 68 Status"]
    #[inline(always)]
    pub fn pid68(&self) -> Pid68R {
        Pid68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 69 Status"]
    #[inline(always)]
    pub fn pid69(&self) -> Pid69R {
        Pid69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock 70 Status"]
    #[inline(always)]
    pub fn pid70(&self) -> Pid70R {
        Pid70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 71 Status"]
    #[inline(always)]
    pub fn pid71(&self) -> Pid71R {
        Pid71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 72 Status"]
    #[inline(always)]
    pub fn pid72(&self) -> Pid72R {
        Pid72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 73 Status"]
    #[inline(always)]
    pub fn pid73(&self) -> Pid73R {
        Pid73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 74 Status"]
    #[inline(always)]
    pub fn pid74(&self) -> Pid74R {
        Pid74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 75 Status"]
    #[inline(always)]
    pub fn pid75(&self) -> Pid75R {
        Pid75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 76 Status"]
    #[inline(always)]
    pub fn pid76(&self) -> Pid76R {
        Pid76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 77 Status"]
    #[inline(always)]
    pub fn pid77(&self) -> Pid77R {
        Pid77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 78 Status"]
    #[inline(always)]
    pub fn pid78(&self) -> Pid78R {
        Pid78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 79 Status"]
    #[inline(always)]
    pub fn pid79(&self) -> Pid79R {
        Pid79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 80 Status"]
    #[inline(always)]
    pub fn pid80(&self) -> Pid80R {
        Pid80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 81 Status"]
    #[inline(always)]
    pub fn pid81(&self) -> Pid81R {
        Pid81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 82 Status"]
    #[inline(always)]
    pub fn pid82(&self) -> Pid82R {
        Pid82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 83 Status"]
    #[inline(always)]
    pub fn pid83(&self) -> Pid83R {
        Pid83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 84 Status"]
    #[inline(always)]
    pub fn pid84(&self) -> Pid84R {
        Pid84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 85 Status"]
    #[inline(always)]
    pub fn pid85(&self) -> Pid85R {
        Pid85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock 86 Status"]
    #[inline(always)]
    pub fn pid86(&self) -> Pid86R {
        Pid86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock 87 Status"]
    #[inline(always)]
    pub fn pid87(&self) -> Pid87R {
        Pid87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 88 Status"]
    #[inline(always)]
    pub fn pid88(&self) -> Pid88R {
        Pid88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 89 Status"]
    #[inline(always)]
    pub fn pid89(&self) -> Pid89R {
        Pid89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 90 Status"]
    #[inline(always)]
    pub fn pid90(&self) -> Pid90R {
        Pid90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 91 Status"]
    #[inline(always)]
    pub fn pid91(&self) -> Pid91R {
        Pid91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 92 Status"]
    #[inline(always)]
    pub fn pid92(&self) -> Pid92R {
        Pid92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock 93 Status"]
    #[inline(always)]
    pub fn pid93(&self) -> Pid93R {
        Pid93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock 94 Status"]
    #[inline(always)]
    pub fn pid94(&self) -> Pid94R {
        Pid94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock 95 Status"]
    #[inline(always)]
    pub fn pid95(&self) -> Pid95R {
        Pid95R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr2Spec;
impl crate::RegisterSpec for Csr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr2::R`](R) reader structure"]
impl crate::Readable for Csr2Spec {}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for Csr2Spec {}
