#[doc = "Register `VIC804` reader"]
pub type R = crate::R<Vic804Spec>;
#[doc = "Register `VIC804` writer"]
pub type W = crate::W<Vic804Spec>;
#[doc = "Field `VICMCUINTREVT1` reader - VIC_MCU_INTR_EVT_1"]
pub type Vicmcuintrevt1R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT1` writer - VIC_MCU_INTR_EVT_1"]
pub type Vicmcuintrevt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_1"]
    #[inline(always)]
    pub fn vicmcuintrevt1(&self) -> Vicmcuintrevt1R {
        Vicmcuintrevt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_1"]
    #[inline(always)]
    pub fn vicmcuintrevt1(&mut self) -> Vicmcuintrevt1W<Vic804Spec> {
        Vicmcuintrevt1W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic804::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic804::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic804Spec;
impl crate::RegisterSpec for Vic804Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic804::R`](R) reader structure"]
impl crate::Readable for Vic804Spec {}
#[doc = "`write(|w| ..)` method takes [`vic804::W`](W) writer structure"]
impl crate::Writable for Vic804Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC804 to value 0"]
impl crate::Resettable for Vic804Spec {}
