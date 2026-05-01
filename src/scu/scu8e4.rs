#[doc = "Register `SCU8E4` reader"]
pub type R = crate::R<Scu8e4Spec>;
#[doc = "Register `SCU8E4` writer"]
pub type W = crate::W<Scu8e4Spec>;
#[doc = "Field `SCUSCRATCHMCU57` reader - SCU_SCRATCH_MCU_57"]
pub type Scuscratchmcu57R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU57` writer - SCU_SCRATCH_MCU_57"]
pub type Scuscratchmcu57W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_57"]
    #[inline(always)]
    pub fn scuscratchmcu57(&self) -> Scuscratchmcu57R {
        Scuscratchmcu57R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_57"]
    #[inline(always)]
    pub fn scuscratchmcu57(&mut self) -> Scuscratchmcu57W<Scu8e4Spec> {
        Scuscratchmcu57W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 57\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8e4Spec;
impl crate::RegisterSpec for Scu8e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8e4::R`](R) reader structure"]
impl crate::Readable for Scu8e4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8e4::W`](W) writer structure"]
impl crate::Writable for Scu8e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8E4 to value 0"]
impl crate::Resettable for Scu8e4Spec {}
