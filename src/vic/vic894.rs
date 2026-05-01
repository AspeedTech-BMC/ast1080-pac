#[doc = "Register `VIC894` reader"]
pub type R = crate::R<Vic894Spec>;
#[doc = "Register `VIC894` writer"]
pub type W = crate::W<Vic894Spec>;
#[doc = "Field `VICMCUINTREN5` reader - VIC_MCU_INTR_EN_5"]
pub type Vicmcuintren5R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN5` writer - VIC_MCU_INTR_EN_5"]
pub type Vicmcuintren5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_5"]
    #[inline(always)]
    pub fn vicmcuintren5(&self) -> Vicmcuintren5R {
        Vicmcuintren5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_5"]
    #[inline(always)]
    pub fn vicmcuintren5(&mut self) -> Vicmcuintren5W<Vic894Spec> {
        Vicmcuintren5W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic894::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic894::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic894Spec;
impl crate::RegisterSpec for Vic894Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic894::R`](R) reader structure"]
impl crate::Readable for Vic894Spec {}
#[doc = "`write(|w| ..)` method takes [`vic894::W`](W) writer structure"]
impl crate::Writable for Vic894Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC894 to value 0"]
impl crate::Resettable for Vic894Spec {}
