#[doc = "Register `SCU840` reader"]
pub type R = crate::R<Scu840Spec>;
#[doc = "Register `SCU840` writer"]
pub type W = crate::W<Scu840Spec>;
#[doc = "Field `SCUSCRATCHMCU16` reader - SCU_SCRATCH_MCU_16"]
pub type Scuscratchmcu16R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU16` writer - SCU_SCRATCH_MCU_16"]
pub type Scuscratchmcu16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_16"]
    #[inline(always)]
    pub fn scuscratchmcu16(&self) -> Scuscratchmcu16R {
        Scuscratchmcu16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_16"]
    #[inline(always)]
    pub fn scuscratchmcu16(&mut self) -> Scuscratchmcu16W<Scu840Spec> {
        Scuscratchmcu16W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu840::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu840::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu840Spec;
impl crate::RegisterSpec for Scu840Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu840::R`](R) reader structure"]
impl crate::Readable for Scu840Spec {}
#[doc = "`write(|w| ..)` method takes [`scu840::W`](W) writer structure"]
impl crate::Writable for Scu840Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU840 to value 0"]
impl crate::Resettable for Scu840Spec {}
