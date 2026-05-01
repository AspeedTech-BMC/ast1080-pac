#[doc = "Register `GPIO014` reader"]
pub type R = crate::R<Gpio014Spec>;
#[doc = "Register `GPIO014` writer"]
pub type W = crate::W<Gpio014Spec>;
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
    ) -> NumberOfSourceClkCyclesForHighW<Gpio014Spec> {
        NumberOfSourceClkCyclesForHighW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of Source Clock cycles for Low"]
    #[inline(always)]
    pub fn number_of_source_clk_cycles_for_low(
        &mut self,
    ) -> NumberOfSourceClkCyclesForLowW<Gpio014Spec> {
        NumberOfSourceClkCyclesForLowW::new(self, 16)
    }
}
#[doc = "GPIO Blink Counter \\#1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio014Spec;
impl crate::RegisterSpec for Gpio014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio014::R`](R) reader structure"]
impl crate::Readable for Gpio014Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio014::W`](W) writer structure"]
impl crate::Writable for Gpio014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO014 to value 0"]
impl crate::Resettable for Gpio014Spec {}
