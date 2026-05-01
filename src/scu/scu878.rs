#[doc = "Register `SCU878` reader"]
pub type R = crate::R<Scu878Spec>;
#[doc = "Register `SCU878` writer"]
pub type W = crate::W<Scu878Spec>;
#[doc = "Field `SCUSCRATCHMCU30` reader - SCU_SCRATCH_MCU_30"]
pub type Scuscratchmcu30R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU30` writer - SCU_SCRATCH_MCU_30"]
pub type Scuscratchmcu30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_30"]
    #[inline(always)]
    pub fn scuscratchmcu30(&self) -> Scuscratchmcu30R {
        Scuscratchmcu30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_30"]
    #[inline(always)]
    pub fn scuscratchmcu30(&mut self) -> Scuscratchmcu30W<Scu878Spec> {
        Scuscratchmcu30W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu878::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu878::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu878Spec;
impl crate::RegisterSpec for Scu878Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu878::R`](R) reader structure"]
impl crate::Readable for Scu878Spec {}
#[doc = "`write(|w| ..)` method takes [`scu878::W`](W) writer structure"]
impl crate::Writable for Scu878Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU878 to value 0"]
impl crate::Resettable for Scu878Spec {}
