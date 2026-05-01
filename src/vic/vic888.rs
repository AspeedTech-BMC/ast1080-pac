#[doc = "Register `VIC888` reader"]
pub type R = crate::R<Vic888Spec>;
#[doc = "Register `VIC888` writer"]
pub type W = crate::W<Vic888Spec>;
#[doc = "Field `VICMCUINTREN2` reader - VIC_MCU_INTR_EN_2"]
pub type Vicmcuintren2R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN2` writer - VIC_MCU_INTR_EN_2"]
pub type Vicmcuintren2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_2"]
    #[inline(always)]
    pub fn vicmcuintren2(&self) -> Vicmcuintren2R {
        Vicmcuintren2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_2"]
    #[inline(always)]
    pub fn vicmcuintren2(&mut self) -> Vicmcuintren2W<Vic888Spec> {
        Vicmcuintren2W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic888Spec;
impl crate::RegisterSpec for Vic888Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic888::R`](R) reader structure"]
impl crate::Readable for Vic888Spec {}
#[doc = "`write(|w| ..)` method takes [`vic888::W`](W) writer structure"]
impl crate::Writable for Vic888Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC888 to value 0"]
impl crate::Resettable for Vic888Spec {}
