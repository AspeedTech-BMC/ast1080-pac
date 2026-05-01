#[doc = "Register `SCU864` reader"]
pub type R = crate::R<Scu864Spec>;
#[doc = "Register `SCU864` writer"]
pub type W = crate::W<Scu864Spec>;
#[doc = "Field `SCUSCRATCHMCU25` reader - SCU_SCRATCH_MCU_25"]
pub type Scuscratchmcu25R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU25` writer - SCU_SCRATCH_MCU_25"]
pub type Scuscratchmcu25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_25"]
    #[inline(always)]
    pub fn scuscratchmcu25(&self) -> Scuscratchmcu25R {
        Scuscratchmcu25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_25"]
    #[inline(always)]
    pub fn scuscratchmcu25(&mut self) -> Scuscratchmcu25W<Scu864Spec> {
        Scuscratchmcu25W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu864::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu864::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu864Spec;
impl crate::RegisterSpec for Scu864Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu864::R`](R) reader structure"]
impl crate::Readable for Scu864Spec {}
#[doc = "`write(|w| ..)` method takes [`scu864::W`](W) writer structure"]
impl crate::Writable for Scu864Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU864 to value 0"]
impl crate::Resettable for Scu864Spec {}
