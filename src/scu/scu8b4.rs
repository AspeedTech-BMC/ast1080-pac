#[doc = "Register `SCU8B4` reader"]
pub type R = crate::R<Scu8b4Spec>;
#[doc = "Register `SCU8B4` writer"]
pub type W = crate::W<Scu8b4Spec>;
#[doc = "Field `SCUSCRATCHMCU45` reader - SCU_SCRATCH_MCU_45"]
pub type Scuscratchmcu45R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU45` writer - SCU_SCRATCH_MCU_45"]
pub type Scuscratchmcu45W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_45"]
    #[inline(always)]
    pub fn scuscratchmcu45(&self) -> Scuscratchmcu45R {
        Scuscratchmcu45R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_45"]
    #[inline(always)]
    pub fn scuscratchmcu45(&mut self) -> Scuscratchmcu45W<Scu8b4Spec> {
        Scuscratchmcu45W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 45\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8b4Spec;
impl crate::RegisterSpec for Scu8b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8b4::R`](R) reader structure"]
impl crate::Readable for Scu8b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8b4::W`](W) writer structure"]
impl crate::Writable for Scu8b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8B4 to value 0"]
impl crate::Resettable for Scu8b4Spec {}
