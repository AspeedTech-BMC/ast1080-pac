#[doc = "Register `VIC810` reader"]
pub type R = crate::R<Vic810Spec>;
#[doc = "Register `VIC810` writer"]
pub type W = crate::W<Vic810Spec>;
#[doc = "Field `VICMCUINTREVT4` reader - VIC_MCU_INTR_EVT_4"]
pub type Vicmcuintrevt4R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT4` writer - VIC_MCU_INTR_EVT_4"]
pub type Vicmcuintrevt4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_4"]
    #[inline(always)]
    pub fn vicmcuintrevt4(&self) -> Vicmcuintrevt4R {
        Vicmcuintrevt4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_4"]
    #[inline(always)]
    pub fn vicmcuintrevt4(&mut self) -> Vicmcuintrevt4W<Vic810Spec> {
        Vicmcuintrevt4W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic810::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic810::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic810Spec;
impl crate::RegisterSpec for Vic810Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic810::R`](R) reader structure"]
impl crate::Readable for Vic810Spec {}
#[doc = "`write(|w| ..)` method takes [`vic810::W`](W) writer structure"]
impl crate::Writable for Vic810Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC810 to value 0"]
impl crate::Resettable for Vic810Spec {}
