#[doc = "Register `MCFG[%s]` reader"]
pub type R = crate::R<McfgSpec>;
#[doc = "Register `MCFG[%s]` writer"]
pub type W = crate::W<McfgSpec>;
#[doc = "Undefined Length Burst Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ulbtselect {
    #[doc = "0: Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1 Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    Unlimited = 0,
    #[doc = "1: Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    Single = 1,
    #[doc = "2: 4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    _4Beat = 2,
    #[doc = "3: 8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    _8Beat = 3,
    #[doc = "4: 16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    _16Beat = 4,
    #[doc = "5: 32-beat Burst-The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    _32Beat = 5,
    #[doc = "6: 64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    _64Beat = 6,
    #[doc = "7: 128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats.Unless duly needed, the ULBT should be left at its default 0 value for power saving."]
    _128Beat = 7,
}
impl From<Ulbtselect> for u8 {
    #[inline(always)]
    fn from(variant: Ulbtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ulbtselect {
    type Ux = u8;
}
impl crate::IsEnum for Ulbtselect {}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader<Ulbtselect>;
impl UlbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbtselect {
        match self.bits {
            0 => Ulbtselect::Unlimited,
            1 => Ulbtselect::Single,
            2 => Ulbtselect::_4Beat,
            3 => Ulbtselect::_8Beat,
            4 => Ulbtselect::_16Beat,
            5 => Ulbtselect::_32Beat,
            6 => Ulbtselect::_64Beat,
            7 => Ulbtselect::_128Beat,
            _ => unreachable!(),
        }
    }
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1 Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == Ulbtselect::Unlimited
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ulbtselect::Single
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn is_4_beat(&self) -> bool {
        *self == Ulbtselect::_4Beat
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn is_8_beat(&self) -> bool {
        *self == Ulbtselect::_8Beat
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn is_16_beat(&self) -> bool {
        *self == Ulbtselect::_16Beat
    }
    #[doc = "32-beat Burst-The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn is_32_beat(&self) -> bool {
        *self == Ulbtselect::_32Beat
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn is_64_beat(&self) -> bool {
        *self == Ulbtselect::_64Beat
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats.Unless duly needed, the ULBT should be left at its default 0 value for power saving."]
    #[inline(always)]
    pub fn is_128_beat(&self) -> bool {
        *self == Ulbtselect::_128Beat
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ulbtselect, crate::Safe>;
impl<'a, REG> UlbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1 Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::Unlimited)
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::Single)
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn _4_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_4Beat)
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn _8_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_8Beat)
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn _16_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_16Beat)
    }
    #[doc = "32-beat Burst-The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn _32_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_32Beat)
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn _64_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_64Beat)
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats.Unless duly needed, the ULBT should be left at its default 0 value for power saving."]
    #[inline(always)]
    pub fn _128_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::_128Beat)
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> UlbtR {
        UlbtR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> UlbtW<McfgSpec> {
        UlbtW::new(self, 0)
    }
}
#[doc = "Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfgSpec;
impl crate::RegisterSpec for McfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for McfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for McfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCFG[%s] to value 0"]
impl crate::Resettable for McfgSpec {}
