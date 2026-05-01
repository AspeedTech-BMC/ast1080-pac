#[doc = "Register `VIC090` reader"]
pub type R = crate::R<Vic090Spec>;
#[doc = "Register `VIC090` writer"]
pub type W = crate::W<Vic090Spec>;
#[doc = "Field `VICMCUINTRSRC4` reader - VIC_MCU_INTR_SRC_4"]
pub type Vicmcuintrsrc4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_SRC_4"]
    #[inline(always)]
    pub fn vicmcuintrsrc4(&self) -> Vicmcuintrsrc4R {
        Vicmcuintrsrc4R::new(self.bits)
    }
}
impl W {}
#[doc = "MCU Interrupt Raw 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic090Spec;
impl crate::RegisterSpec for Vic090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic090::R`](R) reader structure"]
impl crate::Readable for Vic090Spec {}
#[doc = "`write(|w| ..)` method takes [`vic090::W`](W) writer structure"]
impl crate::Writable for Vic090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC090 to value 0"]
impl crate::Resettable for Vic090Spec {}
