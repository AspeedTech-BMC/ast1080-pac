#[doc = "Register `SCU814` reader"]
pub type R = crate::R<Scu814Spec>;
#[doc = "Register `SCU814` writer"]
pub type W = crate::W<Scu814Spec>;
#[doc = "Field `SCUSCRATCHMCU5` reader - SCU_SCRATCH_MCU_5"]
pub type Scuscratchmcu5R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU5` writer - SCU_SCRATCH_MCU_5"]
pub type Scuscratchmcu5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_5"]
    #[inline(always)]
    pub fn scuscratchmcu5(&self) -> Scuscratchmcu5R {
        Scuscratchmcu5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_5"]
    #[inline(always)]
    pub fn scuscratchmcu5(&mut self) -> Scuscratchmcu5W<Scu814Spec> {
        Scuscratchmcu5W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu814::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu814::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu814Spec;
impl crate::RegisterSpec for Scu814Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu814::R`](R) reader structure"]
impl crate::Readable for Scu814Spec {}
#[doc = "`write(|w| ..)` method takes [`scu814::W`](W) writer structure"]
impl crate::Writable for Scu814Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU814 to value 0"]
impl crate::Resettable for Scu814Spec {}
