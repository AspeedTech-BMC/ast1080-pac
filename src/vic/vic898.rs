#[doc = "Register `VIC898` reader"]
pub type R = crate::R<Vic898Spec>;
#[doc = "Register `VIC898` writer"]
pub type W = crate::W<Vic898Spec>;
#[doc = "Field `VICMCUINTREN6` reader - VIC_MCU_INTR_EN_6"]
pub type Vicmcuintren6R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN6` writer - VIC_MCU_INTR_EN_6"]
pub type Vicmcuintren6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_6"]
    #[inline(always)]
    pub fn vicmcuintren6(&self) -> Vicmcuintren6R {
        Vicmcuintren6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_6"]
    #[inline(always)]
    pub fn vicmcuintren6(&mut self) -> Vicmcuintren6W<Vic898Spec> {
        Vicmcuintren6W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic898::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic898::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic898Spec;
impl crate::RegisterSpec for Vic898Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic898::R`](R) reader structure"]
impl crate::Readable for Vic898Spec {}
#[doc = "`write(|w| ..)` method takes [`vic898::W`](W) writer structure"]
impl crate::Writable for Vic898Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC898 to value 0"]
impl crate::Resettable for Vic898Spec {}
