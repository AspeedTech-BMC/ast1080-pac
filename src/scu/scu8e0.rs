#[doc = "Register `SCU8E0` reader"]
pub type R = crate::R<Scu8e0Spec>;
#[doc = "Register `SCU8E0` writer"]
pub type W = crate::W<Scu8e0Spec>;
#[doc = "Field `SCUSCRATCHMCU56` reader - SCU_SCRATCH_MCU_56"]
pub type Scuscratchmcu56R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU56` writer - SCU_SCRATCH_MCU_56"]
pub type Scuscratchmcu56W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_56"]
    #[inline(always)]
    pub fn scuscratchmcu56(&self) -> Scuscratchmcu56R {
        Scuscratchmcu56R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_56"]
    #[inline(always)]
    pub fn scuscratchmcu56(&mut self) -> Scuscratchmcu56W<Scu8e0Spec> {
        Scuscratchmcu56W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 56\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8e0Spec;
impl crate::RegisterSpec for Scu8e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8e0::R`](R) reader structure"]
impl crate::Readable for Scu8e0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8e0::W`](W) writer structure"]
impl crate::Writable for Scu8e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8E0 to value 0"]
impl crate::Resettable for Scu8e0Spec {}
