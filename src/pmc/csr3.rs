#[doc = "Register `CSR3` reader"]
pub type R = crate::R<Csr3Spec>;
#[doc = "Field `PID96` reader - Peripheral Clock 96 Status"]
pub type Pid96R = crate::BitReader;
#[doc = "Field `PID97` reader - Peripheral Clock 97 Status"]
pub type Pid97R = crate::BitReader;
#[doc = "Field `PID98` reader - Peripheral Clock 98 Status"]
pub type Pid98R = crate::BitReader;
#[doc = "Field `PID99` reader - Peripheral Clock 99 Status"]
pub type Pid99R = crate::BitReader;
#[doc = "Field `PID100` reader - Peripheral Clock 100 Status"]
pub type Pid100R = crate::BitReader;
#[doc = "Field `PID101` reader - Peripheral Clock 101 Status"]
pub type Pid101R = crate::BitReader;
#[doc = "Field `PID102` reader - Peripheral Clock 102 Status"]
pub type Pid102R = crate::BitReader;
#[doc = "Field `PID103` reader - Peripheral Clock 103 Status"]
pub type Pid103R = crate::BitReader;
#[doc = "Field `PID104` reader - Peripheral Clock 104 Status"]
pub type Pid104R = crate::BitReader;
#[doc = "Field `PID105` reader - Peripheral Clock 105 Status"]
pub type Pid105R = crate::BitReader;
#[doc = "Field `PID106` reader - Peripheral Clock 106 Status"]
pub type Pid106R = crate::BitReader;
#[doc = "Field `PID107` reader - Peripheral Clock 107 Status"]
pub type Pid107R = crate::BitReader;
#[doc = "Field `PID108` reader - Peripheral Clock 108 Status"]
pub type Pid108R = crate::BitReader;
#[doc = "Field `PID109` reader - Peripheral Clock 109 Status"]
pub type Pid109R = crate::BitReader;
#[doc = "Field `PID110` reader - Peripheral Clock 110 Status"]
pub type Pid110R = crate::BitReader;
#[doc = "Field `PID111` reader - Peripheral Clock 111 Status"]
pub type Pid111R = crate::BitReader;
#[doc = "Field `PID112` reader - Peripheral Clock 112 Status"]
pub type Pid112R = crate::BitReader;
#[doc = "Field `PID113` reader - Peripheral Clock 113 Status"]
pub type Pid113R = crate::BitReader;
#[doc = "Field `PID114` reader - Peripheral Clock 114 Status"]
pub type Pid114R = crate::BitReader;
#[doc = "Field `PID115` reader - Peripheral Clock 115 Status"]
pub type Pid115R = crate::BitReader;
#[doc = "Field `PID116` reader - Peripheral Clock 116 Status"]
pub type Pid116R = crate::BitReader;
#[doc = "Field `PID117` reader - Peripheral Clock 117 Status"]
pub type Pid117R = crate::BitReader;
#[doc = "Field `PID118` reader - Peripheral Clock 118 Status"]
pub type Pid118R = crate::BitReader;
#[doc = "Field `PID119` reader - Peripheral Clock 119 Status"]
pub type Pid119R = crate::BitReader;
#[doc = "Field `PID120` reader - Peripheral Clock 120 Status"]
pub type Pid120R = crate::BitReader;
#[doc = "Field `PID121` reader - Peripheral Clock 121 Status"]
pub type Pid121R = crate::BitReader;
#[doc = "Field `PID122` reader - Peripheral Clock 122 Status"]
pub type Pid122R = crate::BitReader;
#[doc = "Field `PID123` reader - Peripheral Clock 123 Status"]
pub type Pid123R = crate::BitReader;
#[doc = "Field `PID124` reader - Peripheral Clock 124 Status"]
pub type Pid124R = crate::BitReader;
#[doc = "Field `PID125` reader - Peripheral Clock 125 Status"]
pub type Pid125R = crate::BitReader;
#[doc = "Field `PID126` reader - Peripheral Clock 126 Status"]
pub type Pid126R = crate::BitReader;
#[doc = "Field `PID127` reader - Peripheral Clock 127 Status"]
pub type Pid127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 96 Status"]
    #[inline(always)]
    pub fn pid96(&self) -> Pid96R {
        Pid96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 97 Status"]
    #[inline(always)]
    pub fn pid97(&self) -> Pid97R {
        Pid97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 98 Status"]
    #[inline(always)]
    pub fn pid98(&self) -> Pid98R {
        Pid98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 99 Status"]
    #[inline(always)]
    pub fn pid99(&self) -> Pid99R {
        Pid99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock 100 Status"]
    #[inline(always)]
    pub fn pid100(&self) -> Pid100R {
        Pid100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 101 Status"]
    #[inline(always)]
    pub fn pid101(&self) -> Pid101R {
        Pid101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock 102 Status"]
    #[inline(always)]
    pub fn pid102(&self) -> Pid102R {
        Pid102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 103 Status"]
    #[inline(always)]
    pub fn pid103(&self) -> Pid103R {
        Pid103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 104 Status"]
    #[inline(always)]
    pub fn pid104(&self) -> Pid104R {
        Pid104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 105 Status"]
    #[inline(always)]
    pub fn pid105(&self) -> Pid105R {
        Pid105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 106 Status"]
    #[inline(always)]
    pub fn pid106(&self) -> Pid106R {
        Pid106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 107 Status"]
    #[inline(always)]
    pub fn pid107(&self) -> Pid107R {
        Pid107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 108 Status"]
    #[inline(always)]
    pub fn pid108(&self) -> Pid108R {
        Pid108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 109 Status"]
    #[inline(always)]
    pub fn pid109(&self) -> Pid109R {
        Pid109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 110 Status"]
    #[inline(always)]
    pub fn pid110(&self) -> Pid110R {
        Pid110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 111 Status"]
    #[inline(always)]
    pub fn pid111(&self) -> Pid111R {
        Pid111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 112 Status"]
    #[inline(always)]
    pub fn pid112(&self) -> Pid112R {
        Pid112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 113 Status"]
    #[inline(always)]
    pub fn pid113(&self) -> Pid113R {
        Pid113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 114 Status"]
    #[inline(always)]
    pub fn pid114(&self) -> Pid114R {
        Pid114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 115 Status"]
    #[inline(always)]
    pub fn pid115(&self) -> Pid115R {
        Pid115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 116 Status"]
    #[inline(always)]
    pub fn pid116(&self) -> Pid116R {
        Pid116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 117 Status"]
    #[inline(always)]
    pub fn pid117(&self) -> Pid117R {
        Pid117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral Clock 118 Status"]
    #[inline(always)]
    pub fn pid118(&self) -> Pid118R {
        Pid118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral Clock 119 Status"]
    #[inline(always)]
    pub fn pid119(&self) -> Pid119R {
        Pid119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 120 Status"]
    #[inline(always)]
    pub fn pid120(&self) -> Pid120R {
        Pid120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 121 Status"]
    #[inline(always)]
    pub fn pid121(&self) -> Pid121R {
        Pid121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 122 Status"]
    #[inline(always)]
    pub fn pid122(&self) -> Pid122R {
        Pid122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 123 Status"]
    #[inline(always)]
    pub fn pid123(&self) -> Pid123R {
        Pid123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 124 Status"]
    #[inline(always)]
    pub fn pid124(&self) -> Pid124R {
        Pid124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral Clock 125 Status"]
    #[inline(always)]
    pub fn pid125(&self) -> Pid125R {
        Pid125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Clock 126 Status"]
    #[inline(always)]
    pub fn pid126(&self) -> Pid126R {
        Pid126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral Clock 127 Status"]
    #[inline(always)]
    pub fn pid127(&self) -> Pid127R {
        Pid127R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr3Spec;
impl crate::RegisterSpec for Csr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr3::R`](R) reader structure"]
impl crate::Readable for Csr3Spec {}
#[doc = "`reset()` method sets CSR3 to value 0"]
impl crate::Resettable for Csr3Spec {}
