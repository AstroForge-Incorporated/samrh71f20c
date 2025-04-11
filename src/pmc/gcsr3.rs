#[doc = "Register `GCSR3` reader"]
pub type R = crate::R<Gcsr3Spec>;
#[doc = "Field `GPID96` reader - Generic Clock 96 Status"]
pub type Gpid96R = crate::BitReader;
#[doc = "Field `GPID97` reader - Generic Clock 97 Status"]
pub type Gpid97R = crate::BitReader;
#[doc = "Field `GPID98` reader - Generic Clock 98 Status"]
pub type Gpid98R = crate::BitReader;
#[doc = "Field `GPID99` reader - Generic Clock 99 Status"]
pub type Gpid99R = crate::BitReader;
#[doc = "Field `GPID100` reader - Generic Clock 100 Status"]
pub type Gpid100R = crate::BitReader;
#[doc = "Field `GPID101` reader - Generic Clock 101 Status"]
pub type Gpid101R = crate::BitReader;
#[doc = "Field `GPID102` reader - Generic Clock 102 Status"]
pub type Gpid102R = crate::BitReader;
#[doc = "Field `GPID103` reader - Generic Clock 103 Status"]
pub type Gpid103R = crate::BitReader;
#[doc = "Field `GPID104` reader - Generic Clock 104 Status"]
pub type Gpid104R = crate::BitReader;
#[doc = "Field `GPID105` reader - Generic Clock 105 Status"]
pub type Gpid105R = crate::BitReader;
#[doc = "Field `GPID106` reader - Generic Clock 106 Status"]
pub type Gpid106R = crate::BitReader;
#[doc = "Field `GPID107` reader - Generic Clock 107 Status"]
pub type Gpid107R = crate::BitReader;
#[doc = "Field `GPID108` reader - Generic Clock 108 Status"]
pub type Gpid108R = crate::BitReader;
#[doc = "Field `GPID109` reader - Generic Clock 109 Status"]
pub type Gpid109R = crate::BitReader;
#[doc = "Field `GPID110` reader - Generic Clock 110 Status"]
pub type Gpid110R = crate::BitReader;
#[doc = "Field `GPID111` reader - Generic Clock 111 Status"]
pub type Gpid111R = crate::BitReader;
#[doc = "Field `GPID112` reader - Generic Clock 112 Status"]
pub type Gpid112R = crate::BitReader;
#[doc = "Field `GPID113` reader - Generic Clock 113 Status"]
pub type Gpid113R = crate::BitReader;
#[doc = "Field `GPID114` reader - Generic Clock 114 Status"]
pub type Gpid114R = crate::BitReader;
#[doc = "Field `GPID115` reader - Generic Clock 115 Status"]
pub type Gpid115R = crate::BitReader;
#[doc = "Field `GPID116` reader - Generic Clock 116 Status"]
pub type Gpid116R = crate::BitReader;
#[doc = "Field `GPID117` reader - Generic Clock 117 Status"]
pub type Gpid117R = crate::BitReader;
#[doc = "Field `GPID118` reader - Generic Clock 118 Status"]
pub type Gpid118R = crate::BitReader;
#[doc = "Field `GPID119` reader - Generic Clock 119 Status"]
pub type Gpid119R = crate::BitReader;
#[doc = "Field `GPID120` reader - Generic Clock 120 Status"]
pub type Gpid120R = crate::BitReader;
#[doc = "Field `GPID122` reader - Generic Clock 122 Status"]
pub type Gpid122R = crate::FieldReader;
#[doc = "Field `GPID123` reader - Generic Clock 123 Status"]
pub type Gpid123R = crate::BitReader;
#[doc = "Field `GPID124` reader - Generic Clock 124 Status"]
pub type Gpid124R = crate::BitReader;
#[doc = "Field `GPID125` reader - Generic Clock 125 Status"]
pub type Gpid125R = crate::BitReader;
#[doc = "Field `GPID126` reader - Generic Clock 126 Status"]
pub type Gpid126R = crate::BitReader;
#[doc = "Field `GPID127` reader - Generic Clock 127 Status"]
pub type Gpid127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generic Clock 96 Status"]
    #[inline(always)]
    pub fn gpid96(&self) -> Gpid96R {
        Gpid96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic Clock 97 Status"]
    #[inline(always)]
    pub fn gpid97(&self) -> Gpid97R {
        Gpid97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock 98 Status"]
    #[inline(always)]
    pub fn gpid98(&self) -> Gpid98R {
        Gpid98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock 99 Status"]
    #[inline(always)]
    pub fn gpid99(&self) -> Gpid99R {
        Gpid99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock 100 Status"]
    #[inline(always)]
    pub fn gpid100(&self) -> Gpid100R {
        Gpid100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock 101 Status"]
    #[inline(always)]
    pub fn gpid101(&self) -> Gpid101R {
        Gpid101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock 102 Status"]
    #[inline(always)]
    pub fn gpid102(&self) -> Gpid102R {
        Gpid102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock 103 Status"]
    #[inline(always)]
    pub fn gpid103(&self) -> Gpid103R {
        Gpid103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock 104 Status"]
    #[inline(always)]
    pub fn gpid104(&self) -> Gpid104R {
        Gpid104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock 105 Status"]
    #[inline(always)]
    pub fn gpid105(&self) -> Gpid105R {
        Gpid105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock 106 Status"]
    #[inline(always)]
    pub fn gpid106(&self) -> Gpid106R {
        Gpid106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic Clock 107 Status"]
    #[inline(always)]
    pub fn gpid107(&self) -> Gpid107R {
        Gpid107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic Clock 108 Status"]
    #[inline(always)]
    pub fn gpid108(&self) -> Gpid108R {
        Gpid108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generic Clock 109 Status"]
    #[inline(always)]
    pub fn gpid109(&self) -> Gpid109R {
        Gpid109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generic Clock 110 Status"]
    #[inline(always)]
    pub fn gpid110(&self) -> Gpid110R {
        Gpid110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Generic Clock 111 Status"]
    #[inline(always)]
    pub fn gpid111(&self) -> Gpid111R {
        Gpid111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Generic Clock 112 Status"]
    #[inline(always)]
    pub fn gpid112(&self) -> Gpid112R {
        Gpid112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic Clock 113 Status"]
    #[inline(always)]
    pub fn gpid113(&self) -> Gpid113R {
        Gpid113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic Clock 114 Status"]
    #[inline(always)]
    pub fn gpid114(&self) -> Gpid114R {
        Gpid114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Generic Clock 115 Status"]
    #[inline(always)]
    pub fn gpid115(&self) -> Gpid115R {
        Gpid115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Generic Clock 116 Status"]
    #[inline(always)]
    pub fn gpid116(&self) -> Gpid116R {
        Gpid116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Generic Clock 117 Status"]
    #[inline(always)]
    pub fn gpid117(&self) -> Gpid117R {
        Gpid117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Generic Clock 118 Status"]
    #[inline(always)]
    pub fn gpid118(&self) -> Gpid118R {
        Gpid118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Generic Clock 119 Status"]
    #[inline(always)]
    pub fn gpid119(&self) -> Gpid119R {
        Gpid119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Generic Clock 120 Status"]
    #[inline(always)]
    pub fn gpid120(&self) -> Gpid120R {
        Gpid120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Generic Clock 122 Status"]
    #[inline(always)]
    pub fn gpid122(&self) -> Gpid122R {
        Gpid122R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Generic Clock 123 Status"]
    #[inline(always)]
    pub fn gpid123(&self) -> Gpid123R {
        Gpid123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Generic Clock 124 Status"]
    #[inline(always)]
    pub fn gpid124(&self) -> Gpid124R {
        Gpid124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock 125 Status"]
    #[inline(always)]
    pub fn gpid125(&self) -> Gpid125R {
        Gpid125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Generic Clock 126 Status"]
    #[inline(always)]
    pub fn gpid126(&self) -> Gpid126R {
        Gpid126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Generic Clock 127 Status"]
    #[inline(always)]
    pub fn gpid127(&self) -> Gpid127R {
        Gpid127R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Generic Clock Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gcsr3Spec;
impl crate::RegisterSpec for Gcsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcsr3::R`](R) reader structure"]
impl crate::Readable for Gcsr3Spec {}
#[doc = "`reset()` method sets GCSR3 to value 0"]
impl crate::Resettable for Gcsr3Spec {}
