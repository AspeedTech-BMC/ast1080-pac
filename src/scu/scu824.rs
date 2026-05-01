#[doc = "Register `SCU824` reader"]
pub type R = crate::R<Scu824Spec>;
#[doc = "Register `SCU824` writer"]
pub type W = crate::W<Scu824Spec>;
#[doc = "Field `SCUSCRATCHMCU9` reader - SCU_SCRATCH_MCU_9"]
pub type Scuscratchmcu9R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU9` writer - SCU_SCRATCH_MCU_9"]
pub type Scuscratchmcu9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_9"]
    #[inline(always)]
    pub fn scuscratchmcu9(&self) -> Scuscratchmcu9R {
        Scuscratchmcu9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_9"]
    #[inline(always)]
    pub fn scuscratchmcu9(&mut self) -> Scuscratchmcu9W<Scu824Spec> {
        Scuscratchmcu9W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu824::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu824::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu824Spec;
impl crate::RegisterSpec for Scu824Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu824::R`](R) reader structure"]
impl crate::Readable for Scu824Spec {}
#[doc = "`write(|w| ..)` method takes [`scu824::W`](W) writer structure"]
impl crate::Writable for Scu824Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU824 to value 0"]
impl crate::Resettable for Scu824Spec {}
