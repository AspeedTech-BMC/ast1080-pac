#[doc = "Register `VIC808` reader"]
pub type R = crate::R<Vic808Spec>;
#[doc = "Register `VIC808` writer"]
pub type W = crate::W<Vic808Spec>;
#[doc = "Field `VICMCUINTREVT2` reader - VIC_MCU_INTR_EVT_2"]
pub type Vicmcuintrevt2R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT2` writer - VIC_MCU_INTR_EVT_2"]
pub type Vicmcuintrevt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_2"]
    #[inline(always)]
    pub fn vicmcuintrevt2(&self) -> Vicmcuintrevt2R {
        Vicmcuintrevt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_2"]
    #[inline(always)]
    pub fn vicmcuintrevt2(&mut self) -> Vicmcuintrevt2W<Vic808Spec> {
        Vicmcuintrevt2W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic808::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic808::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic808Spec;
impl crate::RegisterSpec for Vic808Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic808::R`](R) reader structure"]
impl crate::Readable for Vic808Spec {}
#[doc = "`write(|w| ..)` method takes [`vic808::W`](W) writer structure"]
impl crate::Writable for Vic808Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC808 to value 0"]
impl crate::Resettable for Vic808Spec {}
