#[doc = "Register `VIC800` reader"]
pub type R = crate::R<Vic800Spec>;
#[doc = "Register `VIC800` writer"]
pub type W = crate::W<Vic800Spec>;
#[doc = "Field `VICMCUINTREVT0` reader - VIC_MCU_INTR_EVT_0"]
pub type Vicmcuintrevt0R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT0` writer - VIC_MCU_INTR_EVT_0"]
pub type Vicmcuintrevt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_0"]
    #[inline(always)]
    pub fn vicmcuintrevt0(&self) -> Vicmcuintrevt0R {
        Vicmcuintrevt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_0"]
    #[inline(always)]
    pub fn vicmcuintrevt0(&mut self) -> Vicmcuintrevt0W<Vic800Spec> {
        Vicmcuintrevt0W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic800::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic800::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic800Spec;
impl crate::RegisterSpec for Vic800Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic800::R`](R) reader structure"]
impl crate::Readable for Vic800Spec {}
#[doc = "`write(|w| ..)` method takes [`vic800::W`](W) writer structure"]
impl crate::Writable for Vic800Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC800 to value 0"]
impl crate::Resettable for Vic800Spec {}
