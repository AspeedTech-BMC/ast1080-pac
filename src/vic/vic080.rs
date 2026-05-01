#[doc = "Register `VIC080` reader"]
pub type R = crate::R<Vic080Spec>;
#[doc = "Register `VIC080` writer"]
pub type W = crate::W<Vic080Spec>;
#[doc = "Field `VICMCUINTRSRC0` reader - VIC_MCU_INTR_SRC_0"]
pub type Vicmcuintrsrc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_0"]
    #[inline(always)]
    pub fn vicmcuintrsrc0(&self) -> Vicmcuintrsrc0R {
        Vicmcuintrsrc0R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic080Spec;
impl crate::RegisterSpec for Vic080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic080::R`](R) reader structure"]
impl crate::Readable for Vic080Spec {}
#[doc = "`write(|w| ..)` method takes [`vic080::W`](W) writer structure"]
impl crate::Writable for Vic080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC080 to value 0"]
impl crate::Resettable for Vic080Spec {}
