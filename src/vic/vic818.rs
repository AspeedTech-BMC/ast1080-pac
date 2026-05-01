#[doc = "Register `VIC818` reader"]
pub type R = crate::R<Vic818Spec>;
#[doc = "Register `VIC818` writer"]
pub type W = crate::W<Vic818Spec>;
#[doc = "Field `VICMCUINTREVT6` reader - VIC_MCU_INTR_EVT_6"]
pub type Vicmcuintrevt6R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT6` writer - VIC_MCU_INTR_EVT_6"]
pub type Vicmcuintrevt6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_6"]
    #[inline(always)]
    pub fn vicmcuintrevt6(&self) -> Vicmcuintrevt6R {
        Vicmcuintrevt6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_6"]
    #[inline(always)]
    pub fn vicmcuintrevt6(&mut self) -> Vicmcuintrevt6W<Vic818Spec> {
        Vicmcuintrevt6W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic818::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic818::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic818Spec;
impl crate::RegisterSpec for Vic818Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic818::R`](R) reader structure"]
impl crate::Readable for Vic818Spec {}
#[doc = "`write(|w| ..)` method takes [`vic818::W`](W) writer structure"]
impl crate::Writable for Vic818Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC818 to value 0"]
impl crate::Resettable for Vic818Spec {}
