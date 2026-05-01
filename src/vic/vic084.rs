#[doc = "Register `VIC084` reader"]
pub type R = crate::R<Vic084Spec>;
#[doc = "Register `VIC084` writer"]
pub type W = crate::W<Vic084Spec>;
#[doc = "Field `VICMCUINTRSRC1` reader - VIC_MCU_INTR_SRC_1"]
pub type Vicmcuintrsrc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_1"]
    #[inline(always)]
    pub fn vicmcuintrsrc1(&self) -> Vicmcuintrsrc1R {
        Vicmcuintrsrc1R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic084Spec;
impl crate::RegisterSpec for Vic084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic084::R`](R) reader structure"]
impl crate::Readable for Vic084Spec {}
#[doc = "`write(|w| ..)` method takes [`vic084::W`](W) writer structure"]
impl crate::Writable for Vic084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC084 to value 0"]
impl crate::Resettable for Vic084Spec {}
