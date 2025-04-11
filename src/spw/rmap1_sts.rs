#[doc = "Register `RMAP1_STS` reader"]
pub type R = crate::R<Rmap1StsSpec>;
#[doc = "Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Errcodeselect {
    #[doc = "0: No error detected"]
    Noerror = 0,
    #[doc = "1: Error while DMA accessing the internal bus, e.g. illegal address."]
    Dmaerror = 1,
    #[doc = "2: Unused RMAP command according to \\[RMAP\\]"]
    Rmaperror = 2,
    #[doc = "3: Destination key error"]
    Destkeyerror = 3,
    #[doc = "4: Data CRC error"]
    Datacrcerror = 4,
    #[doc = "5: Early EOP in header or data, i.e. EOP has been received with less data than expected from the RMAP command header."]
    Eoperror = 5,
    #[doc = "6: Cargo too large. Late EOP or EEP in data, i.e. EOP/EEP has been received with more data than expected from the RMAP command header."]
    Cargoerror = 6,
    #[doc = "7: Early EEP in data for RMAP commands. EEP has been received with less data or exactly as much as expected from the RMAP command header."]
    Eeperror = 7,
    #[doc = "10: Authorisation error:Invalid or unsupported command."]
    Cmderror = 10,
    #[doc = "12: Non-matching Target Logical Address."]
    Tlaerror = 12,
    #[doc = "16: Incorrect header CRC."]
    Headercrcerror = 16,
    #[doc = "17: Protocol Identifier not supported."]
    Protocoliderror = 17,
    #[doc = "18: Unsupported reply address length"]
    Replyadderror = 18,
}
impl From<Errcodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Errcodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Errcodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Errcodeselect {}
#[doc = "Field `ERRCODE` reader - Error Code"]
pub type ErrcodeR = crate::FieldReader<Errcodeselect>;
impl ErrcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Errcodeselect> {
        match self.bits {
            0 => Some(Errcodeselect::Noerror),
            1 => Some(Errcodeselect::Dmaerror),
            2 => Some(Errcodeselect::Rmaperror),
            3 => Some(Errcodeselect::Destkeyerror),
            4 => Some(Errcodeselect::Datacrcerror),
            5 => Some(Errcodeselect::Eoperror),
            6 => Some(Errcodeselect::Cargoerror),
            7 => Some(Errcodeselect::Eeperror),
            10 => Some(Errcodeselect::Cmderror),
            12 => Some(Errcodeselect::Tlaerror),
            16 => Some(Errcodeselect::Headercrcerror),
            17 => Some(Errcodeselect::Protocoliderror),
            18 => Some(Errcodeselect::Replyadderror),
            _ => None,
        }
    }
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Errcodeselect::Noerror
    }
    #[doc = "Error while DMA accessing the internal bus, e.g. illegal address."]
    #[inline(always)]
    pub fn is_dmaerror(&self) -> bool {
        *self == Errcodeselect::Dmaerror
    }
    #[doc = "Unused RMAP command according to \\[RMAP\\]"]
    #[inline(always)]
    pub fn is_rmaperror(&self) -> bool {
        *self == Errcodeselect::Rmaperror
    }
    #[doc = "Destination key error"]
    #[inline(always)]
    pub fn is_destkeyerror(&self) -> bool {
        *self == Errcodeselect::Destkeyerror
    }
    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn is_datacrcerror(&self) -> bool {
        *self == Errcodeselect::Datacrcerror
    }
    #[doc = "Early EOP in header or data, i.e. EOP has been received with less data than expected from the RMAP command header."]
    #[inline(always)]
    pub fn is_eoperror(&self) -> bool {
        *self == Errcodeselect::Eoperror
    }
    #[doc = "Cargo too large. Late EOP or EEP in data, i.e. EOP/EEP has been received with more data than expected from the RMAP command header."]
    #[inline(always)]
    pub fn is_cargoerror(&self) -> bool {
        *self == Errcodeselect::Cargoerror
    }
    #[doc = "Early EEP in data for RMAP commands. EEP has been received with less data or exactly as much as expected from the RMAP command header."]
    #[inline(always)]
    pub fn is_eeperror(&self) -> bool {
        *self == Errcodeselect::Eeperror
    }
    #[doc = "Authorisation error:Invalid or unsupported command."]
    #[inline(always)]
    pub fn is_cmderror(&self) -> bool {
        *self == Errcodeselect::Cmderror
    }
    #[doc = "Non-matching Target Logical Address."]
    #[inline(always)]
    pub fn is_tlaerror(&self) -> bool {
        *self == Errcodeselect::Tlaerror
    }
    #[doc = "Incorrect header CRC."]
    #[inline(always)]
    pub fn is_headercrcerror(&self) -> bool {
        *self == Errcodeselect::Headercrcerror
    }
    #[doc = "Protocol Identifier not supported."]
    #[inline(always)]
    pub fn is_protocoliderror(&self) -> bool {
        *self == Errcodeselect::Protocoliderror
    }
    #[doc = "Unsupported reply address length"]
    #[inline(always)]
    pub fn is_replyadderror(&self) -> bool {
        *self == Errcodeselect::Replyadderror
    }
}
#[doc = "Field `VALID` reader - Valid"]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Error Code"]
    #[inline(always)]
    pub fn errcode(&self) -> ErrcodeR {
        ErrcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "SpW RMAP 1 Read Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap1_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rmap1StsSpec;
impl crate::RegisterSpec for Rmap1StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmap1_sts::R`](R) reader structure"]
impl crate::Readable for Rmap1StsSpec {}
#[doc = "`reset()` method sets RMAP1_STS to value 0"]
impl crate::Resettable for Rmap1StsSpec {}
