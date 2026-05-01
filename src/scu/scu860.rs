#[doc = "Register `SCU860` reader"]
pub type R = crate::R<Scu860Spec>;
#[doc = "Register `SCU860` writer"]
pub type W = crate::W<Scu860Spec>;
#[doc = "Field `SCUSCRATCHMCU24` reader - SCU_SCRATCH_MCU_24"]
pub type Scuscratchmcu24R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU24` writer - SCU_SCRATCH_MCU_24"]
pub type Scuscratchmcu24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_24"]
    #[inline(always)]
    pub fn scuscratchmcu24(&self) -> Scuscratchmcu24R {
        Scuscratchmcu24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_24"]
    #[inline(always)]
    pub fn scuscratchmcu24(&mut self) -> Scuscratchmcu24W<Scu860Spec> {
        Scuscratchmcu24W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu860::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu860::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu860Spec;
impl crate::RegisterSpec for Scu860Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu860::R`](R) reader structure"]
impl crate::Readable for Scu860Spec {}
#[doc = "`write(|w| ..)` method takes [`scu860::W`](W) writer structure"]
impl crate::Writable for Scu860Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU860 to value 0"]
impl crate::Resettable for Scu860Spec {}
