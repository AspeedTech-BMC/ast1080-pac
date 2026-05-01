#[doc = "Register `GPIO00C` reader"]
pub type R = crate::R<Gpio00cSpec>;
#[doc = "Register `GPIO00C` writer"]
pub type W = crate::W<Gpio00cSpec>;
#[doc = "Field `BlinkSourceClkDivisionOnlyEffectiveToPCLKAsSrcClk` reader - Blink Source Clock Division. Only effective to PCLK as source clock"]
pub type BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkR = crate::FieldReader<u32>;
#[doc = "Field `BlinkSourceClkDivisionOnlyEffectiveToPCLKAsSrcClk` writer - Blink Source Clock Division. Only effective to PCLK as source clock"]
pub type BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkW<'a, REG> =
    crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Blink Source Clock Division. Only effective to PCLK as source clock"]
    #[inline(always)]
    pub fn blink_source_clk_division_only_effective_to_pclkas_src_clk(
        &self,
    ) -> BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkR {
        BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Blink Source Clock Division. Only effective to PCLK as source clock"]
    #[inline(always)]
    pub fn blink_source_clk_division_only_effective_to_pclkas_src_clk(
        &mut self,
    ) -> BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkW<Gpio00cSpec> {
        BlinkSourceClkDivisionOnlyEffectiveToPclkasSrcClkW::new(self, 0)
    }
}
#[doc = "GPIO Blink Source Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio00cSpec;
impl crate::RegisterSpec for Gpio00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio00c::R`](R) reader structure"]
impl crate::Readable for Gpio00cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio00c::W`](W) writer structure"]
impl crate::Writable for Gpio00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO00C to value 0"]
impl crate::Resettable for Gpio00cSpec {}
