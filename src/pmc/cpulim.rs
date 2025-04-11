#[doc = "Register `CPULIM` reader"]
pub type R = crate::R<CpulimSpec>;
#[doc = "Register `CPULIM` writer"]
pub type W = crate::W<CpulimSpec>;
#[doc = "Field `CPU_LOW_IT` reader - CPU Monitoring Low IT Limit"]
pub type CpuLowItR = crate::FieldReader;
#[doc = "Field `CPU_LOW_IT` writer - CPU Monitoring Low IT Limit"]
pub type CpuLowItW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_HIGH_IT` reader - CPU Monitoring High IT Limit"]
pub type CpuHighItR = crate::FieldReader;
#[doc = "Field `CPU_HIGH_IT` writer - CPU Monitoring High IT Limit"]
pub type CpuHighItW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_LOW_RES` reader - CPU Monitoring Low RESET Limit"]
pub type CpuLowResR = crate::FieldReader;
#[doc = "Field `CPU_LOW_RES` writer - CPU Monitoring Low RESET Limit"]
pub type CpuLowResW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_HIGH_RES` reader - CPU Monitoring High Reset Limit"]
pub type CpuHighResR = crate::FieldReader;
#[doc = "Field `CPU_HIGH_RES` writer - CPU Monitoring High Reset Limit"]
pub type CpuHighResW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CPU Monitoring Low IT Limit"]
    #[inline(always)]
    pub fn cpu_low_it(&self) -> CpuLowItR {
        CpuLowItR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CPU Monitoring High IT Limit"]
    #[inline(always)]
    pub fn cpu_high_it(&self) -> CpuHighItR {
        CpuHighItR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CPU Monitoring Low RESET Limit"]
    #[inline(always)]
    pub fn cpu_low_res(&self) -> CpuLowResR {
        CpuLowResR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CPU Monitoring High Reset Limit"]
    #[inline(always)]
    pub fn cpu_high_res(&self) -> CpuHighResR {
        CpuHighResR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPU Monitoring Low IT Limit"]
    #[inline(always)]
    pub fn cpu_low_it(&mut self) -> CpuLowItW<CpulimSpec> {
        CpuLowItW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CPU Monitoring High IT Limit"]
    #[inline(always)]
    pub fn cpu_high_it(&mut self) -> CpuHighItW<CpulimSpec> {
        CpuHighItW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CPU Monitoring Low RESET Limit"]
    #[inline(always)]
    pub fn cpu_low_res(&mut self) -> CpuLowResW<CpulimSpec> {
        CpuLowResW::new(self, 16)
    }
    #[doc = "Bits 24:31 - CPU Monitoring High Reset Limit"]
    #[inline(always)]
    pub fn cpu_high_res(&mut self) -> CpuHighResW<CpulimSpec> {
        CpuHighResW::new(self, 24)
    }
}
#[doc = "CPU Monitor Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpulim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpulim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpulimSpec;
impl crate::RegisterSpec for CpulimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpulim::R`](R) reader structure"]
impl crate::Readable for CpulimSpec {}
#[doc = "`write(|w| ..)` method takes [`cpulim::W`](W) writer structure"]
impl crate::Writable for CpulimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPULIM to value 0"]
impl crate::Resettable for CpulimSpec {}
