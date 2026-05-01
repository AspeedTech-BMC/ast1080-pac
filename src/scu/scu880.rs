#[doc = "Register `SCU880` reader"]
pub type R = crate::R<Scu880Spec>;
#[doc = "Register `SCU880` writer"]
pub type W = crate::W<Scu880Spec>;
#[doc = "Field `SCUSCRATCHMCU32` reader - SCU_SCRATCH_MCU_32"]
pub type Scuscratchmcu32R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU32` writer - SCU_SCRATCH_MCU_32"]
pub type Scuscratchmcu32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_32"]
    #[inline(always)]
    pub fn scuscratchmcu32(&self) -> Scuscratchmcu32R {
        Scuscratchmcu32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_32"]
    #[inline(always)]
    pub fn scuscratchmcu32(&mut self) -> Scuscratchmcu32W<Scu880Spec> {
        Scuscratchmcu32W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu880::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu880::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu880Spec;
impl crate::RegisterSpec for Scu880Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu880::R`](R) reader structure"]
impl crate::Readable for Scu880Spec {}
#[doc = "`write(|w| ..)` method takes [`scu880::W`](W) writer structure"]
impl crate::Writable for Scu880Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU880 to value 0"]
impl crate::Resettable for Scu880Spec {}
