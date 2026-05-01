#[doc = "Register `JTAG03C` reader"]
pub type R = crate::R<Jtag03cSpec>;
#[doc = "Register `JTAG03C` writer"]
pub type W = crate::W<Jtag03cSpec>;
#[doc = "Field `EngIdle` reader - Engine Idle"]
pub type EngIdleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Engine Idle"]
    #[inline(always)]
    pub fn eng_idle(&self) -> EngIdleR {
        EngIdleR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag03cSpec;
impl crate::RegisterSpec for Jtag03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag03c::R`](R) reader structure"]
impl crate::Readable for Jtag03cSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag03c::W`](W) writer structure"]
impl crate::Writable for Jtag03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG03C to value 0"]
impl crate::Resettable for Jtag03cSpec {}
