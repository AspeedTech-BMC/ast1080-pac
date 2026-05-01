#[doc = "Register `VIC088` reader"]
pub type R = crate::R<Vic088Spec>;
#[doc = "Register `VIC088` writer"]
pub type W = crate::W<Vic088Spec>;
#[doc = "Field `VICMCUINTRSRC2` reader - VIC_MCU_INTR_SRC_2"]
pub type Vicmcuintrsrc2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_2"]
    #[inline(always)]
    pub fn vicmcuintrsrc2(&self) -> Vicmcuintrsrc2R {
        Vicmcuintrsrc2R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic088Spec;
impl crate::RegisterSpec for Vic088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic088::R`](R) reader structure"]
impl crate::Readable for Vic088Spec {}
#[doc = "`write(|w| ..)` method takes [`vic088::W`](W) writer structure"]
impl crate::Writable for Vic088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC088 to value 0"]
impl crate::Resettable for Vic088Spec {}
