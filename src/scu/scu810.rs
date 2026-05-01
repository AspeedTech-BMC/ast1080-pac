#[doc = "Register `SCU810` reader"]
pub type R = crate::R<Scu810Spec>;
#[doc = "Register `SCU810` writer"]
pub type W = crate::W<Scu810Spec>;
#[doc = "Field `SCUSCRATCHMCU4` reader - SCU_SCRATCH_MCU_4"]
pub type Scuscratchmcu4R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU4` writer - SCU_SCRATCH_MCU_4"]
pub type Scuscratchmcu4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_4"]
    #[inline(always)]
    pub fn scuscratchmcu4(&self) -> Scuscratchmcu4R {
        Scuscratchmcu4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_4"]
    #[inline(always)]
    pub fn scuscratchmcu4(&mut self) -> Scuscratchmcu4W<Scu810Spec> {
        Scuscratchmcu4W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu810::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu810::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu810Spec;
impl crate::RegisterSpec for Scu810Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu810::R`](R) reader structure"]
impl crate::Readable for Scu810Spec {}
#[doc = "`write(|w| ..)` method takes [`scu810::W`](W) writer structure"]
impl crate::Writable for Scu810Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU810 to value 0"]
impl crate::Resettable for Scu810Spec {}
