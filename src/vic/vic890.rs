#[doc = "Register `VIC890` reader"]
pub type R = crate::R<Vic890Spec>;
#[doc = "Register `VIC890` writer"]
pub type W = crate::W<Vic890Spec>;
#[doc = "Field `VICMCUINTREN4` reader - VIC_MCU_INTR_EN_4"]
pub type Vicmcuintren4R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN4` writer - VIC_MCU_INTR_EN_4"]
pub type Vicmcuintren4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_4"]
    #[inline(always)]
    pub fn vicmcuintren4(&self) -> Vicmcuintren4R {
        Vicmcuintren4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_4"]
    #[inline(always)]
    pub fn vicmcuintren4(&mut self) -> Vicmcuintren4W<Vic890Spec> {
        Vicmcuintren4W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic890::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic890::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic890Spec;
impl crate::RegisterSpec for Vic890Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic890::R`](R) reader structure"]
impl crate::Readable for Vic890Spec {}
#[doc = "`write(|w| ..)` method takes [`vic890::W`](W) writer structure"]
impl crate::Writable for Vic890Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC890 to value 0"]
impl crate::Resettable for Vic890Spec {}
