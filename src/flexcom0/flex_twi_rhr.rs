#[doc = "Register `FLEX_TWI_RHR` reader"]
pub type R = crate::R<FlexTwiRhrSpec>;
#[doc = "Field `RXDATA` reader - Master or Slave Receive Holding Data"]
pub type RxdataR = crate::FieldReader;
#[doc = "Start State (Slave Sniffer Mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sstateselect {
    #[doc = "0: No START detected with the logged data"]
    Nostart = 0,
    #[doc = "1: START (S) detected with the logged data"]
    Start = 1,
    #[doc = "2: Repeated START (Sr) detected with the logged data"]
    Rstart = 2,
    #[doc = "3: Not defined"]
    Undef = 3,
}
impl From<Sstateselect> for u8 {
    #[inline(always)]
    fn from(variant: Sstateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sstateselect {
    type Ux = u8;
}
impl crate::IsEnum for Sstateselect {}
#[doc = "Field `SSTATE` reader - Start State (Slave Sniffer Mode only)"]
pub type SstateR = crate::FieldReader<Sstateselect>;
impl SstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sstateselect {
        match self.bits {
            0 => Sstateselect::Nostart,
            1 => Sstateselect::Start,
            2 => Sstateselect::Rstart,
            3 => Sstateselect::Undef,
            _ => unreachable!(),
        }
    }
    #[doc = "No START detected with the logged data"]
    #[inline(always)]
    pub fn is_nostart(&self) -> bool {
        *self == Sstateselect::Nostart
    }
    #[doc = "START (S) detected with the logged data"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Sstateselect::Start
    }
    #[doc = "Repeated START (Sr) detected with the logged data"]
    #[inline(always)]
    pub fn is_rstart(&self) -> bool {
        *self == Sstateselect::Rstart
    }
    #[doc = "Not defined"]
    #[inline(always)]
    pub fn is_undef(&self) -> bool {
        *self == Sstateselect::Undef
    }
}
#[doc = "Field `PSTATE` reader - Stop State (Slave Sniffer Mode only)"]
pub type PstateR = crate::BitReader;
#[doc = "Acknowledge State (Slave Sniffer Mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Astateselect {
    #[doc = "0: No Acknowledge or Nacknowledge detected after previously logged data"]
    None = 0,
    #[doc = "1: Acknowledge (A) detected after previously logged data"]
    Ack = 1,
    #[doc = "2: Nacknowledge (NA) detected after previously logged data"]
    Nack = 2,
    #[doc = "3: Not defined"]
    Undef = 3,
}
impl From<Astateselect> for u8 {
    #[inline(always)]
    fn from(variant: Astateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Astateselect {
    type Ux = u8;
}
impl crate::IsEnum for Astateselect {}
#[doc = "Field `ASTATE` reader - Acknowledge State (Slave Sniffer Mode only)"]
pub type AstateR = crate::FieldReader<Astateselect>;
impl AstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Astateselect {
        match self.bits {
            0 => Astateselect::None,
            1 => Astateselect::Ack,
            2 => Astateselect::Nack,
            3 => Astateselect::Undef,
            _ => unreachable!(),
        }
    }
    #[doc = "No Acknowledge or Nacknowledge detected after previously logged data"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Astateselect::None
    }
    #[doc = "Acknowledge (A) detected after previously logged data"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Astateselect::Ack
    }
    #[doc = "Nacknowledge (NA) detected after previously logged data"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Astateselect::Nack
    }
    #[doc = "Not defined"]
    #[inline(always)]
    pub fn is_undef(&self) -> bool {
        *self == Astateselect::Undef
    }
}
impl R {
    #[doc = "Bits 0:7 - Master or Slave Receive Holding Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Start State (Slave Sniffer Mode only)"]
    #[inline(always)]
    pub fn sstate(&self) -> SstateR {
        SstateR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Stop State (Slave Sniffer Mode only)"]
    #[inline(always)]
    pub fn pstate(&self) -> PstateR {
        PstateR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Acknowledge State (Slave Sniffer Mode only)"]
    #[inline(always)]
    pub fn astate(&self) -> AstateR {
        AstateR::new(((self.bits >> 11) & 3) as u8)
    }
}
#[doc = "TWI Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiRhrSpec;
impl crate::RegisterSpec for FlexTwiRhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_rhr::R`](R) reader structure"]
impl crate::Readable for FlexTwiRhrSpec {}
#[doc = "`reset()` method sets FLEX_TWI_RHR to value 0"]
impl crate::Resettable for FlexTwiRhrSpec {}
