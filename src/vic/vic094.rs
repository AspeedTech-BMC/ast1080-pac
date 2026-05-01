#[doc = "Register `VIC094` reader"]
pub type R = crate::R<Vic094Spec>;
#[doc = "Register `VIC094` writer"]
pub type W = crate::W<Vic094Spec>;
#[doc = "Field `VICMCUINTRSRC5` reader - VIC_MCU_INTR_SRC_5"]
pub type Vicmcuintrsrc5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_5"]
    #[inline(always)]
    pub fn vicmcuintrsrc5(&self) -> Vicmcuintrsrc5R {
        Vicmcuintrsrc5R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic094Spec;
impl crate::RegisterSpec for Vic094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic094::R`](R) reader structure"]
impl crate::Readable for Vic094Spec {}
#[doc = "`write(|w| ..)` method takes [`vic094::W`](W) writer structure"]
impl crate::Writable for Vic094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC094 to value 0"]
impl crate::Resettable for Vic094Spec {}
