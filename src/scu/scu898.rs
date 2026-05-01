#[doc = "Register `SCU898` reader"]
pub type R = crate::R<Scu898Spec>;
#[doc = "Register `SCU898` writer"]
pub type W = crate::W<Scu898Spec>;
#[doc = "Field `SCUSCRATCHMCU38` reader - SCU_SCRATCH_MCU_38"]
pub type Scuscratchmcu38R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU38` writer - SCU_SCRATCH_MCU_38"]
pub type Scuscratchmcu38W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_38"]
    #[inline(always)]
    pub fn scuscratchmcu38(&self) -> Scuscratchmcu38R {
        Scuscratchmcu38R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_38"]
    #[inline(always)]
    pub fn scuscratchmcu38(&mut self) -> Scuscratchmcu38W<Scu898Spec> {
        Scuscratchmcu38W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 38\n\nYou can [`read`](crate::Reg::read) this register and get [`scu898::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu898::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu898Spec;
impl crate::RegisterSpec for Scu898Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu898::R`](R) reader structure"]
impl crate::Readable for Scu898Spec {}
#[doc = "`write(|w| ..)` method takes [`scu898::W`](W) writer structure"]
impl crate::Writable for Scu898Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU898 to value 0"]
impl crate::Resettable for Scu898Spec {}
