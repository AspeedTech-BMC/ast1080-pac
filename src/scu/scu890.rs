#[doc = "Register `SCU890` reader"]
pub type R = crate::R<Scu890Spec>;
#[doc = "Register `SCU890` writer"]
pub type W = crate::W<Scu890Spec>;
#[doc = "Field `SCUSCRATCHMCU36` reader - SCU_SCRATCH_MCU_36"]
pub type Scuscratchmcu36R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU36` writer - SCU_SCRATCH_MCU_36"]
pub type Scuscratchmcu36W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_36"]
    #[inline(always)]
    pub fn scuscratchmcu36(&self) -> Scuscratchmcu36R {
        Scuscratchmcu36R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_36"]
    #[inline(always)]
    pub fn scuscratchmcu36(&mut self) -> Scuscratchmcu36W<Scu890Spec> {
        Scuscratchmcu36W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 36\n\nYou can [`read`](crate::Reg::read) this register and get [`scu890::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu890::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu890Spec;
impl crate::RegisterSpec for Scu890Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu890::R`](R) reader structure"]
impl crate::Readable for Scu890Spec {}
#[doc = "`write(|w| ..)` method takes [`scu890::W`](W) writer structure"]
impl crate::Writable for Scu890Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU890 to value 0"]
impl crate::Resettable for Scu890Spec {}
