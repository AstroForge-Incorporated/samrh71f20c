#[doc = "Register `LINK1_STATUS` reader"]
pub type R = crate::R<Link1StatusSpec>;
#[doc = "LinkState\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Linkstateselect {
    #[doc = "0: CODEC link state machine in ErrorReset state"]
    Errorreset = 0,
    #[doc = "1: CODEC link state machine in ErrorWait state"]
    Errorwait = 1,
    #[doc = "2: CODEC link state machine in Ready state"]
    Ready = 2,
    #[doc = "3: CODEC link state machine in Started state"]
    Started = 3,
    #[doc = "4: CODEC link state machine in Connecting state"]
    Connecting = 4,
    #[doc = "5: CODEC link state machine in Run state"]
    Run = 5,
}
impl From<Linkstateselect> for u8 {
    #[inline(always)]
    fn from(variant: Linkstateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Linkstateselect {
    type Ux = u8;
}
impl crate::IsEnum for Linkstateselect {}
#[doc = "Field `LINKSTATE` reader - LinkState"]
pub type LinkstateR = crate::FieldReader<Linkstateselect>;
impl LinkstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Linkstateselect> {
        match self.bits {
            0 => Some(Linkstateselect::Errorreset),
            1 => Some(Linkstateselect::Errorwait),
            2 => Some(Linkstateselect::Ready),
            3 => Some(Linkstateselect::Started),
            4 => Some(Linkstateselect::Connecting),
            5 => Some(Linkstateselect::Run),
            _ => None,
        }
    }
    #[doc = "CODEC link state machine in ErrorReset state"]
    #[inline(always)]
    pub fn is_errorreset(&self) -> bool {
        *self == Linkstateselect::Errorreset
    }
    #[doc = "CODEC link state machine in ErrorWait state"]
    #[inline(always)]
    pub fn is_errorwait(&self) -> bool {
        *self == Linkstateselect::Errorwait
    }
    #[doc = "CODEC link state machine in Ready state"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Linkstateselect::Ready
    }
    #[doc = "CODEC link state machine in Started state"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Linkstateselect::Started
    }
    #[doc = "CODEC link state machine in Connecting state"]
    #[inline(always)]
    pub fn is_connecting(&self) -> bool {
        *self == Linkstateselect::Connecting
    }
    #[doc = "CODEC link state machine in Run state"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Linkstateselect::Run
    }
}
#[doc = "Field `TXDEFDIV` reader - TxDefDiv"]
pub type TxdefdivR = crate::FieldReader;
#[doc = "Field `TXEMPTY` reader - TxEmpty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `GOTNULL` reader - GotNull"]
pub type GotnullR = crate::BitReader;
#[doc = "Field `GOTFCT` reader - GotFCT"]
pub type GotfctR = crate::BitReader;
#[doc = "Field `GOTNCHAR` reader - GotNChar"]
pub type GotncharR = crate::BitReader;
#[doc = "Field `SEEN0` reader - SEEN0"]
pub type Seen0R = crate::BitReader;
#[doc = "Field `SEEN1` reader - SEEN1"]
pub type Seen1R = crate::BitReader;
#[doc = "Field `SEEN2` reader - SEEN2"]
pub type Seen2R = crate::BitReader;
#[doc = "Field `SEEN3` reader - SEEN3"]
pub type Seen3R = crate::BitReader;
#[doc = "Field `SEEN4` reader - SEEN4"]
pub type Seen4R = crate::BitReader;
#[doc = "Field `SEEN5` reader - SEEN5"]
pub type Seen5R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - LinkState"]
    #[inline(always)]
    pub fn linkstate(&self) -> LinkstateR {
        LinkstateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - TxDefDiv"]
    #[inline(always)]
    pub fn txdefdiv(&self) -> TxdefdivR {
        TxdefdivR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - TxEmpty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GotNull"]
    #[inline(always)]
    pub fn gotnull(&self) -> GotnullR {
        GotnullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GotFCT"]
    #[inline(always)]
    pub fn gotfct(&self) -> GotfctR {
        GotfctR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GotNChar"]
    #[inline(always)]
    pub fn gotnchar(&self) -> GotncharR {
        GotncharR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SEEN0"]
    #[inline(always)]
    pub fn seen0(&self) -> Seen0R {
        Seen0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SEEN1"]
    #[inline(always)]
    pub fn seen1(&self) -> Seen1R {
        Seen1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SEEN2"]
    #[inline(always)]
    pub fn seen2(&self) -> Seen2R {
        Seen2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SEEN3"]
    #[inline(always)]
    pub fn seen3(&self) -> Seen3R {
        Seen3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SEEN4"]
    #[inline(always)]
    pub fn seen4(&self) -> Seen4R {
        Seen4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SEEN5"]
    #[inline(always)]
    pub fn seen5(&self) -> Seen5R {
        Seen5R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "SpW Link 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1StatusSpec;
impl crate::RegisterSpec for Link1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_status::R`](R) reader structure"]
impl crate::Readable for Link1StatusSpec {}
#[doc = "`reset()` method sets LINK1_STATUS to value 0"]
impl crate::Resettable for Link1StatusSpec {}
