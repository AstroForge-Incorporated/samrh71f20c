#[doc = "Register `PKTRX1_STATUS` reader"]
pub type R = crate::R<Pktrx1StatusSpec>;
#[doc = "Field `COUNT` reader - Packet Count"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `PACKET` reader - Packet"]
pub type PacketR = crate::BitReader;
#[doc = "Field `LOCKED` reader - Locked"]
pub type LockedR = crate::BitReader;
#[doc = "Field `ARM` reader - Armed"]
pub type ArmR = crate::BitReader;
#[doc = "Field `ACT` reader - Active"]
pub type ActR = crate::BitReader;
#[doc = "Field `PENDING` reader - Pending"]
pub type PendingR = crate::BitReader;
#[doc = "Field `DEACT` reader - Deactivating"]
pub type DeactR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Packet Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Packet"]
    #[inline(always)]
    pub fn packet(&self) -> PacketR {
        PacketR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Armed"]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Active"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Deactivating"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "PktRx Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1StatusSpec;
impl crate::RegisterSpec for Pktrx1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_status::R`](R) reader structure"]
impl crate::Readable for Pktrx1StatusSpec {}
#[doc = "`reset()` method sets PKTRX1_STATUS to value 0"]
impl crate::Resettable for Pktrx1StatusSpec {}
