#[doc = "Register `VIC814` reader"]
pub type R = crate::R<Vic814Spec>;
#[doc = "Register `VIC814` writer"]
pub type W = crate::W<Vic814Spec>;
#[doc = "Field `VICMCUINTREVT5` reader - VIC_MCU_INTR_EVT_5"]
pub type Vicmcuintrevt5R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT5` writer - VIC_MCU_INTR_EVT_5"]
pub type Vicmcuintrevt5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_5"]
    #[inline(always)]
    pub fn vicmcuintrevt5(&self) -> Vicmcuintrevt5R {
        Vicmcuintrevt5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_5"]
    #[inline(always)]
    pub fn vicmcuintrevt5(&mut self) -> Vicmcuintrevt5W<Vic814Spec> {
        Vicmcuintrevt5W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic814::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic814::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic814Spec;
impl crate::RegisterSpec for Vic814Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic814::R`](R) reader structure"]
impl crate::Readable for Vic814Spec {}
#[doc = "`write(|w| ..)` method takes [`vic814::W`](W) writer structure"]
impl crate::Writable for Vic814Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC814 to value 0"]
impl crate::Resettable for Vic814Spec {}
