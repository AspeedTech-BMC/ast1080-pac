#[doc = "Register `GPIO01C` reader"]
pub type R = crate::R<Gpio01cSpec>;
#[doc = "Register `GPIO01C` writer"]
pub type W = crate::W<Gpio01cSpec>;
#[doc = "Field `NumberOfSourceClkCyclesForHigh` reader - Number of Source Clock cycles for High"]
pub type NumberOfSourceClkCyclesForHighR = crate::FieldReader<u16>;
#[doc = "Field `NumberOfSourceClkCyclesForHigh` writer - Number of Source Clock cycles for High"]
pub type NumberOfSourceClkCyclesForHighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NumberOfSourceClkCyclesForLow` reader - Number of Source Clock cycles for Low"]
pub type NumberOfSourceClkCyclesForLowR = crate::FieldReader<u16>;
#[doc = "Field `NumberOfSourceClkCyclesForLow` writer - Number of Source Clock cycles for Low"]
pub type NumberOfSourceClkCyclesForLowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Source Clock cycles for High"]
    #[inline(always)]
    pub fn number_of_source_clk_cycles_for_high(&self) -> NumberOfSourceClkCyclesForHighR {
        NumberOfSourceClkCyclesForHighR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Source Clock cycles for Low"]
    #[inline(always)]
    pub fn number_of_source_clk_cycles_for_low(&self) -> NumberOfSourceClkCyclesForLowR {
        NumberOfSourceClkCyclesForLowR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Source Clock cycles for High"]
    #[inline(always)]
    pub fn number_of_source_clk_cycles_for_high(
        &mut self,
    ) -> NumberOfSourceClkCyclesForHighW<Gpio01cSpec> {
        NumberOfSourceClkCyclesForHighW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of Source Clock cycles for Low"]
    #[inline(always)]
    pub fn number_of_source_clk_cycles_for_low(
        &mut self,
    ) -> NumberOfSourceClkCyclesForLowW<Gpio01cSpec> {
        NumberOfSourceClkCyclesForLowW::new(self, 16)
    }
}
#[doc = "GPIO Blink Counter \\#3 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio01cSpec;
impl crate::RegisterSpec for Gpio01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio01c::R`](R) reader structure"]
impl crate::Readable for Gpio01cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio01c::W`](W) writer structure"]
impl crate::Writable for Gpio01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO01C to value 0"]
impl crate::Resettable for Gpio01cSpec {}
