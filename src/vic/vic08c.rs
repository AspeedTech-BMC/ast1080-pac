#[doc = "Register `VIC08C` reader"]
pub type R = crate::R<Vic08cSpec>;
#[doc = "Register `VIC08C` writer"]
pub type W = crate::W<Vic08cSpec>;
#[doc = "Field `VICMCUINTRSRC3` reader - VIC_MCU_INTR_SRC_3"]
pub type Vicmcuintrsrc3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_3"]
    #[inline(always)]
    pub fn vicmcuintrsrc3(&self) -> Vicmcuintrsrc3R {
        Vicmcuintrsrc3R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic08cSpec;
impl crate::RegisterSpec for Vic08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic08c::R`](R) reader structure"]
impl crate::Readable for Vic08cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic08c::W`](W) writer structure"]
impl crate::Writable for Vic08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC08C to value 0"]
impl crate::Resettable for Vic08cSpec {}
