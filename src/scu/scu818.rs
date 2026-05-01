#[doc = "Register `SCU818` reader"]
pub type R = crate::R<Scu818Spec>;
#[doc = "Register `SCU818` writer"]
pub type W = crate::W<Scu818Spec>;
#[doc = "Field `SCUSCRATCHMCU6` reader - SCU_SCRATCH_MCU_6"]
pub type Scuscratchmcu6R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU6` writer - SCU_SCRATCH_MCU_6"]
pub type Scuscratchmcu6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_6"]
    #[inline(always)]
    pub fn scuscratchmcu6(&self) -> Scuscratchmcu6R {
        Scuscratchmcu6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_6"]
    #[inline(always)]
    pub fn scuscratchmcu6(&mut self) -> Scuscratchmcu6W<Scu818Spec> {
        Scuscratchmcu6W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu818::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu818::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu818Spec;
impl crate::RegisterSpec for Scu818Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu818::R`](R) reader structure"]
impl crate::Readable for Scu818Spec {}
#[doc = "`write(|w| ..)` method takes [`scu818::W`](W) writer structure"]
impl crate::Writable for Scu818Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU818 to value 0"]
impl crate::Resettable for Scu818Spec {}
