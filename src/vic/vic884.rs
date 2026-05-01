#[doc = "Register `VIC884` reader"]
pub type R = crate::R<Vic884Spec>;
#[doc = "Register `VIC884` writer"]
pub type W = crate::W<Vic884Spec>;
#[doc = "Field `VICMCUINTREN1` reader - VIC_MCU_INTR_EN_1"]
pub type Vicmcuintren1R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN1` writer - VIC_MCU_INTR_EN_1"]
pub type Vicmcuintren1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_1"]
    #[inline(always)]
    pub fn vicmcuintren1(&self) -> Vicmcuintren1R {
        Vicmcuintren1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_1"]
    #[inline(always)]
    pub fn vicmcuintren1(&mut self) -> Vicmcuintren1W<Vic884Spec> {
        Vicmcuintren1W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic884::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic884::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic884Spec;
impl crate::RegisterSpec for Vic884Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic884::R`](R) reader structure"]
impl crate::Readable for Vic884Spec {}
#[doc = "`write(|w| ..)` method takes [`vic884::W`](W) writer structure"]
impl crate::Writable for Vic884Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC884 to value 0"]
impl crate::Resettable for Vic884Spec {}
