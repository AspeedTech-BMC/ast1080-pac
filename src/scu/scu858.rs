#[doc = "Register `SCU858` reader"]
pub type R = crate::R<Scu858Spec>;
#[doc = "Register `SCU858` writer"]
pub type W = crate::W<Scu858Spec>;
#[doc = "Field `SCUSCRATCHMCU22` reader - SCU_SCRATCH_MCU_22"]
pub type Scuscratchmcu22R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU22` writer - SCU_SCRATCH_MCU_22"]
pub type Scuscratchmcu22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_22"]
    #[inline(always)]
    pub fn scuscratchmcu22(&self) -> Scuscratchmcu22R {
        Scuscratchmcu22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_22"]
    #[inline(always)]
    pub fn scuscratchmcu22(&mut self) -> Scuscratchmcu22W<Scu858Spec> {
        Scuscratchmcu22W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu858::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu858::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu858Spec;
impl crate::RegisterSpec for Scu858Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu858::R`](R) reader structure"]
impl crate::Readable for Scu858Spec {}
#[doc = "`write(|w| ..)` method takes [`scu858::W`](W) writer structure"]
impl crate::Writable for Scu858Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU858 to value 0"]
impl crate::Resettable for Scu858Spec {}
