#[doc = "Register `VIC880` reader"]
pub type R = crate::R<Vic880Spec>;
#[doc = "Register `VIC880` writer"]
pub type W = crate::W<Vic880Spec>;
#[doc = "Field `VICMCUINTREN0` reader - VIC_MCU_INTR_EN_0"]
pub type Vicmcuintren0R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN0` writer - VIC_MCU_INTR_EN_0"]
pub type Vicmcuintren0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_0"]
    #[inline(always)]
    pub fn vicmcuintren0(&self) -> Vicmcuintren0R {
        Vicmcuintren0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_0"]
    #[inline(always)]
    pub fn vicmcuintren0(&mut self) -> Vicmcuintren0W<Vic880Spec> {
        Vicmcuintren0W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic880::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic880::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic880Spec;
impl crate::RegisterSpec for Vic880Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic880::R`](R) reader structure"]
impl crate::Readable for Vic880Spec {}
#[doc = "`write(|w| ..)` method takes [`vic880::W`](W) writer structure"]
impl crate::Writable for Vic880Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC880 to value 0"]
impl crate::Resettable for Vic880Spec {}
