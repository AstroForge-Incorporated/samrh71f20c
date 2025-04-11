#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "I/O Line Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Funcselect {
    #[doc = "0: Select the PIO mode for the selected I/O lines."]
    Gpio = 0,
    #[doc = "1: Select the peripheral A for the selected I/O lines."]
    PeriphA = 1,
    #[doc = "2: Select the peripheral B for the selected I/O lines."]
    PeriphB = 2,
    #[doc = "3: Select the peripheral C for the selected I/O lines."]
    PeriphC = 3,
    #[doc = "4: Select the peripheral D for the selected I/O lines."]
    PeriphD = 4,
    #[doc = "5: Select the peripheral E for the selected I/O lines."]
    PeriphE = 5,
    #[doc = "6: Select the peripheral F for the selected I/O lines."]
    PeriphF = 6,
    #[doc = "7: Select the peripheral G for the selected I/O lines."]
    PeriphG = 7,
}
impl From<Funcselect> for u8 {
    #[inline(always)]
    fn from(variant: Funcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Funcselect {
    type Ux = u8;
}
impl crate::IsEnum for Funcselect {}
#[doc = "Field `FUNC` reader - I/O Line Function"]
pub type FuncR = crate::FieldReader<Funcselect>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Funcselect {
        match self.bits {
            0 => Funcselect::Gpio,
            1 => Funcselect::PeriphA,
            2 => Funcselect::PeriphB,
            3 => Funcselect::PeriphC,
            4 => Funcselect::PeriphD,
            5 => Funcselect::PeriphE,
            6 => Funcselect::PeriphF,
            7 => Funcselect::PeriphG,
            _ => unreachable!(),
        }
    }
    #[doc = "Select the PIO mode for the selected I/O lines."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Funcselect::Gpio
    }
    #[doc = "Select the peripheral A for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_a(&self) -> bool {
        *self == Funcselect::PeriphA
    }
    #[doc = "Select the peripheral B for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_b(&self) -> bool {
        *self == Funcselect::PeriphB
    }
    #[doc = "Select the peripheral C for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_c(&self) -> bool {
        *self == Funcselect::PeriphC
    }
    #[doc = "Select the peripheral D for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_d(&self) -> bool {
        *self == Funcselect::PeriphD
    }
    #[doc = "Select the peripheral E for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_e(&self) -> bool {
        *self == Funcselect::PeriphE
    }
    #[doc = "Select the peripheral F for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_f(&self) -> bool {
        *self == Funcselect::PeriphF
    }
    #[doc = "Select the peripheral G for the selected I/O lines."]
    #[inline(always)]
    pub fn is_periph_g(&self) -> bool {
        *self == Funcselect::PeriphG
    }
}
#[doc = "Field `FUNC` writer - I/O Line Function"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Funcselect, crate::Safe>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the PIO mode for the selected I/O lines."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::Gpio)
    }
    #[doc = "Select the peripheral A for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_a(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphA)
    }
    #[doc = "Select the peripheral B for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_b(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphB)
    }
    #[doc = "Select the peripheral C for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_c(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphC)
    }
    #[doc = "Select the peripheral D for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_d(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphD)
    }
    #[doc = "Select the peripheral E for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_e(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphE)
    }
    #[doc = "Select the peripheral F for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_f(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphF)
    }
    #[doc = "Select the peripheral G for the selected I/O lines."]
    #[inline(always)]
    pub fn periph_g(self) -> &'a mut crate::W<REG> {
        self.variant(Funcselect::PeriphG)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirselect {
    #[doc = "0: The selected I/O lines are pure inputs."]
    Input = 0,
    #[doc = "1: The selected I/O lines are enabled in output."]
    Output = 1,
}
impl From<Dirselect> for bool {
    #[inline(always)]
    fn from(variant: Dirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader<Dirselect>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dirselect {
        match self.bits {
            false => Dirselect::Input,
            true => Dirselect::Output,
        }
    }
    #[doc = "The selected I/O lines are pure inputs."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Dirselect::Input
    }
    #[doc = "The selected I/O lines are enabled in output."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Dirselect::Output
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dirselect>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected I/O lines are pure inputs."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Dirselect::Input)
    }
    #[doc = "The selected I/O lines are enabled in output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Dirselect::Output)
    }
}
#[doc = "Pull-Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puenselect {
    #[doc = "0: Pull-Up is disabled for the selected I/O lines."]
    Disabled = 0,
    #[doc = "1: Pull-Up is enabled for the selected I/O lines."]
    Enabled = 1,
}
impl From<Puenselect> for bool {
    #[inline(always)]
    fn from(variant: Puenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUEN` reader - Pull-Up Enable"]
pub type PuenR = crate::BitReader<Puenselect>;
impl PuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Puenselect {
        match self.bits {
            false => Puenselect::Disabled,
            true => Puenselect::Enabled,
        }
    }
    #[doc = "Pull-Up is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Puenselect::Disabled
    }
    #[doc = "Pull-Up is enabled for the selected I/O lines."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Puenselect::Enabled
    }
}
#[doc = "Field `PUEN` writer - Pull-Up Enable"]
pub type PuenW<'a, REG> = crate::BitWriter<'a, REG, Puenselect>;
impl<'a, REG> PuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-Up is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Puenselect::Disabled)
    }
    #[doc = "Pull-Up is enabled for the selected I/O lines."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Puenselect::Enabled)
    }
}
#[doc = "Pull-Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdenselect {
    #[doc = "0: Pull-Down is disabled for the selected I/O lines."]
    Disabled = 0,
    #[doc = "1: Pull-Down is enabled for the selected I/O lines only if PUEN is 0."]
    Enabled = 1,
}
impl From<Pdenselect> for bool {
    #[inline(always)]
    fn from(variant: Pdenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN` reader - Pull-Down Enable"]
pub type PdenR = crate::BitReader<Pdenselect>;
impl PdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdenselect {
        match self.bits {
            false => Pdenselect::Disabled,
            true => Pdenselect::Enabled,
        }
    }
    #[doc = "Pull-Down is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pdenselect::Disabled
    }
    #[doc = "Pull-Down is enabled for the selected I/O lines only if PUEN is 0."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pdenselect::Enabled
    }
}
#[doc = "Field `PDEN` writer - Pull-Down Enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG, Pdenselect>;
impl<'a, REG> PdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-Down is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pdenselect::Disabled)
    }
    #[doc = "Pull-Down is enabled for the selected I/O lines only if PUEN is 0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pdenselect::Enabled)
    }
}
#[doc = "Open-Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opdselect {
    #[doc = "0: The open-drain is disabled for the selected I/O lines. I/O lines are driven at high- and low-level."]
    Disabled = 0,
    #[doc = "1: The open-drain is enabled for the selected I/O lines. I/O lines are driven at low-level only."]
    Enabled = 1,
}
impl From<Opdselect> for bool {
    #[inline(always)]
    fn from(variant: Opdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPD` reader - Open-Drain"]
pub type OpdR = crate::BitReader<Opdselect>;
impl OpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opdselect {
        match self.bits {
            false => Opdselect::Disabled,
            true => Opdselect::Enabled,
        }
    }
    #[doc = "The open-drain is disabled for the selected I/O lines. I/O lines are driven at high- and low-level."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opdselect::Disabled
    }
    #[doc = "The open-drain is enabled for the selected I/O lines. I/O lines are driven at low-level only."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opdselect::Enabled
    }
}
#[doc = "Field `OPD` writer - Open-Drain"]
pub type OpdW<'a, REG> = crate::BitWriter<'a, REG, Opdselect>;
impl<'a, REG> OpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The open-drain is disabled for the selected I/O lines. I/O lines are driven at high- and low-level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opdselect::Disabled)
    }
    #[doc = "The open-drain is enabled for the selected I/O lines. I/O lines are driven at low-level only."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opdselect::Enabled)
    }
}
#[doc = "Schmitt Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Schmittselect {
    #[doc = "0: Schmitt trigger is enabled for the selected I/O lines."]
    Enabled = 0,
    #[doc = "1: Schmitt trigger is disabled for the selected I/O lines."]
    Disabled = 1,
}
impl From<Schmittselect> for bool {
    #[inline(always)]
    fn from(variant: Schmittselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCHMITT` reader - Schmitt Trigger"]
pub type SchmittR = crate::BitReader<Schmittselect>;
impl SchmittR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Schmittselect {
        match self.bits {
            false => Schmittselect::Enabled,
            true => Schmittselect::Disabled,
        }
    }
    #[doc = "Schmitt trigger is enabled for the selected I/O lines."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Schmittselect::Enabled
    }
    #[doc = "Schmitt trigger is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Schmittselect::Disabled
    }
}
#[doc = "Field `SCHMITT` writer - Schmitt Trigger"]
pub type SchmittW<'a, REG> = crate::BitWriter<'a, REG, Schmittselect>;
impl<'a, REG> SchmittW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Schmitt trigger is enabled for the selected I/O lines."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Schmittselect::Enabled)
    }
    #[doc = "Schmitt trigger is disabled for the selected I/O lines."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Schmittselect::Disabled)
    }
}
#[doc = "Drive Strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drvstrselect {
    #[doc = "0: Output drive is 2mA"]
    Out2m = 0,
    #[doc = "1: Output drive is 4mA"]
    Out4m = 1,
    #[doc = "2: Output drive is 8mA"]
    Out8m = 2,
    #[doc = "3: Output drive is 16mA"]
    Out16m = 3,
    #[doc = "4: Output drive is 24mA"]
    Out24m = 4,
    #[doc = "5: Output drive is 32mA"]
    Out32m = 5,
    #[doc = "6: Output drive is 40mA"]
    Out40m = 6,
    #[doc = "7: Output drive is 48mA"]
    Out48m = 7,
}
impl From<Drvstrselect> for u8 {
    #[inline(always)]
    fn from(variant: Drvstrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drvstrselect {
    type Ux = u8;
}
impl crate::IsEnum for Drvstrselect {}
#[doc = "Field `DRVSTR` reader - Drive Strength"]
pub type DrvstrR = crate::FieldReader<Drvstrselect>;
impl DrvstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvstrselect {
        match self.bits {
            0 => Drvstrselect::Out2m,
            1 => Drvstrselect::Out4m,
            2 => Drvstrselect::Out8m,
            3 => Drvstrselect::Out16m,
            4 => Drvstrselect::Out24m,
            5 => Drvstrselect::Out32m,
            6 => Drvstrselect::Out40m,
            7 => Drvstrselect::Out48m,
            _ => unreachable!(),
        }
    }
    #[doc = "Output drive is 2mA"]
    #[inline(always)]
    pub fn is_out_2m(&self) -> bool {
        *self == Drvstrselect::Out2m
    }
    #[doc = "Output drive is 4mA"]
    #[inline(always)]
    pub fn is_out_4m(&self) -> bool {
        *self == Drvstrselect::Out4m
    }
    #[doc = "Output drive is 8mA"]
    #[inline(always)]
    pub fn is_out_8m(&self) -> bool {
        *self == Drvstrselect::Out8m
    }
    #[doc = "Output drive is 16mA"]
    #[inline(always)]
    pub fn is_out_16m(&self) -> bool {
        *self == Drvstrselect::Out16m
    }
    #[doc = "Output drive is 24mA"]
    #[inline(always)]
    pub fn is_out_24m(&self) -> bool {
        *self == Drvstrselect::Out24m
    }
    #[doc = "Output drive is 32mA"]
    #[inline(always)]
    pub fn is_out_32m(&self) -> bool {
        *self == Drvstrselect::Out32m
    }
    #[doc = "Output drive is 40mA"]
    #[inline(always)]
    pub fn is_out_40m(&self) -> bool {
        *self == Drvstrselect::Out40m
    }
    #[doc = "Output drive is 48mA"]
    #[inline(always)]
    pub fn is_out_48m(&self) -> bool {
        *self == Drvstrselect::Out48m
    }
}
#[doc = "Field `DRVSTR` writer - Drive Strength"]
pub type DrvstrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drvstrselect, crate::Safe>;
impl<'a, REG> DrvstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output drive is 2mA"]
    #[inline(always)]
    pub fn out_2m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out2m)
    }
    #[doc = "Output drive is 4mA"]
    #[inline(always)]
    pub fn out_4m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out4m)
    }
    #[doc = "Output drive is 8mA"]
    #[inline(always)]
    pub fn out_8m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out8m)
    }
    #[doc = "Output drive is 16mA"]
    #[inline(always)]
    pub fn out_16m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out16m)
    }
    #[doc = "Output drive is 24mA"]
    #[inline(always)]
    pub fn out_24m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out24m)
    }
    #[doc = "Output drive is 32mA"]
    #[inline(always)]
    pub fn out_32m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out32m)
    }
    #[doc = "Output drive is 40mA"]
    #[inline(always)]
    pub fn out_40m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out40m)
    }
    #[doc = "Output drive is 48mA"]
    #[inline(always)]
    pub fn out_48m(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstrselect::Out48m)
    }
}
#[doc = "Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evtselselect {
    #[doc = "0: Event detection on input falling edge"]
    Falling = 0,
    #[doc = "1: Event detection on input rising edge"]
    Rising = 1,
    #[doc = "2: Event detection on input both edge"]
    Both = 2,
    #[doc = "3: Event detection on low level input"]
    Low = 3,
    #[doc = "4: Event detection on high level input"]
    High = 4,
}
impl From<Evtselselect> for u8 {
    #[inline(always)]
    fn from(variant: Evtselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evtselselect {
    type Ux = u8;
}
impl crate::IsEnum for Evtselselect {}
#[doc = "Field `EVTSEL` reader - Event Selection"]
pub type EvtselR = crate::FieldReader<Evtselselect>;
impl EvtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Evtselselect> {
        match self.bits {
            0 => Some(Evtselselect::Falling),
            1 => Some(Evtselselect::Rising),
            2 => Some(Evtselselect::Both),
            3 => Some(Evtselselect::Low),
            4 => Some(Evtselselect::High),
            _ => None,
        }
    }
    #[doc = "Event detection on input falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Evtselselect::Falling
    }
    #[doc = "Event detection on input rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Evtselselect::Rising
    }
    #[doc = "Event detection on input both edge"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Evtselselect::Both
    }
    #[doc = "Event detection on low level input"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Evtselselect::Low
    }
    #[doc = "Event detection on high level input"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Evtselselect::High
    }
}
#[doc = "Field `EVTSEL` writer - Event Selection"]
pub type EvtselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Evtselselect>;
impl<'a, REG> EvtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event detection on input falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Evtselselect::Falling)
    }
    #[doc = "Event detection on input rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Evtselselect::Rising)
    }
    #[doc = "Event detection on input both edge"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Evtselselect::Both)
    }
    #[doc = "Event detection on low level input"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Evtselselect::Low)
    }
    #[doc = "Event detection on high level input"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Evtselselect::High)
    }
}
#[doc = "Physical Configuration Freeze Status (read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcfsselect {
    #[doc = "0: The fields are not frozen and can be written for this I/O line."]
    NotFrozen = 0,
    #[doc = "1: The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    Frozen = 1,
}
impl From<Pcfsselect> for bool {
    #[inline(always)]
    fn from(variant: Pcfsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCFS` reader - Physical Configuration Freeze Status (read-only)"]
pub type PcfsR = crate::BitReader<Pcfsselect>;
impl PcfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcfsselect {
        match self.bits {
            false => Pcfsselect::NotFrozen,
            true => Pcfsselect::Frozen,
        }
    }
    #[doc = "The fields are not frozen and can be written for this I/O line."]
    #[inline(always)]
    pub fn is_not_frozen(&self) -> bool {
        *self == Pcfsselect::NotFrozen
    }
    #[doc = "The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Pcfsselect::Frozen
    }
}
#[doc = "Field `PCFS` writer - Physical Configuration Freeze Status (read-only)"]
pub type PcfsW<'a, REG> = crate::BitWriter<'a, REG, Pcfsselect>;
impl<'a, REG> PcfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fields are not frozen and can be written for this I/O line."]
    #[inline(always)]
    pub fn not_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Pcfsselect::NotFrozen)
    }
    #[doc = "The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Pcfsselect::Frozen)
    }
}
#[doc = "Interrupt Configuration Freeze Status (read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icfsselect {
    #[doc = "0: The fields are not frozen and can be written for this I/O line."]
    NotFrozen = 0,
    #[doc = "1: The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    Frozen = 1,
}
impl From<Icfsselect> for bool {
    #[inline(always)]
    fn from(variant: Icfsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICFS` reader - Interrupt Configuration Freeze Status (read-only)"]
pub type IcfsR = crate::BitReader<Icfsselect>;
impl IcfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icfsselect {
        match self.bits {
            false => Icfsselect::NotFrozen,
            true => Icfsselect::Frozen,
        }
    }
    #[doc = "The fields are not frozen and can be written for this I/O line."]
    #[inline(always)]
    pub fn is_not_frozen(&self) -> bool {
        *self == Icfsselect::NotFrozen
    }
    #[doc = "The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == Icfsselect::Frozen
    }
}
#[doc = "Field `ICFS` writer - Interrupt Configuration Freeze Status (read-only)"]
pub type IcfsW<'a, REG> = crate::BitWriter<'a, REG, Icfsselect>;
impl<'a, REG> IcfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The fields are not frozen and can be written for this I/O line."]
    #[inline(always)]
    pub fn not_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Icfsselect::NotFrozen)
    }
    #[doc = "The fields are frozen and cannot be written for this I/O line. Only a hardware reset can release these fields."]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(Icfsselect::Frozen)
    }
}
impl R {
    #[doc = "Bits 0:2 - I/O Line Function"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull-Up Enable"]
    #[inline(always)]
    pub fn puen(&self) -> PuenR {
        PuenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Open-Drain"]
    #[inline(always)]
    pub fn opd(&self) -> OpdR {
        OpdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Schmitt Trigger"]
    #[inline(always)]
    pub fn schmitt(&self) -> SchmittR {
        SchmittR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Drive Strength"]
    #[inline(always)]
    pub fn drvstr(&self) -> DrvstrR {
        DrvstrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Event Selection"]
    #[inline(always)]
    pub fn evtsel(&self) -> EvtselR {
        EvtselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Physical Configuration Freeze Status (read-only)"]
    #[inline(always)]
    pub fn pcfs(&self) -> PcfsR {
        PcfsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Configuration Freeze Status (read-only)"]
    #[inline(always)]
    pub fn icfs(&self) -> IcfsR {
        IcfsR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - I/O Line Function"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<CfgrSpec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bit 8 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<CfgrSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bit 9 - Pull-Up Enable"]
    #[inline(always)]
    pub fn puen(&mut self) -> PuenW<CfgrSpec> {
        PuenW::new(self, 9)
    }
    #[doc = "Bit 10 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<CfgrSpec> {
        PdenW::new(self, 10)
    }
    #[doc = "Bit 14 - Open-Drain"]
    #[inline(always)]
    pub fn opd(&mut self) -> OpdW<CfgrSpec> {
        OpdW::new(self, 14)
    }
    #[doc = "Bit 15 - Schmitt Trigger"]
    #[inline(always)]
    pub fn schmitt(&mut self) -> SchmittW<CfgrSpec> {
        SchmittW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Drive Strength"]
    #[inline(always)]
    pub fn drvstr(&mut self) -> DrvstrW<CfgrSpec> {
        DrvstrW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Event Selection"]
    #[inline(always)]
    pub fn evtsel(&mut self) -> EvtselW<CfgrSpec> {
        EvtselW::new(self, 24)
    }
    #[doc = "Bit 29 - Physical Configuration Freeze Status (read-only)"]
    #[inline(always)]
    pub fn pcfs(&mut self) -> PcfsW<CfgrSpec> {
        PcfsW::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt Configuration Freeze Status (read-only)"]
    #[inline(always)]
    pub fn icfs(&mut self) -> IcfsW<CfgrSpec> {
        IcfsW::new(self, 30)
    }
}
#[doc = "PIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
