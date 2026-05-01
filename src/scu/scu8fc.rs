#[doc = "Register `SCU8FC` reader"]
pub type R = crate::R<Scu8fcSpec>;
#[doc = "Register `SCU8FC` writer"]
pub type W = crate::W<Scu8fcSpec>;
#[doc = "Field `SCUSCRATCHMCU63` reader - SCU_SCRATCH_MCU_63"]
pub type Scuscratchmcu63R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU63` writer - SCU_SCRATCH_MCU_63"]
pub type Scuscratchmcu63W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_63"]
    #[inline(always)]
    pub fn scuscratchmcu63(&self) -> Scuscratchmcu63R {
        Scuscratchmcu63R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_63"]
    #[inline(always)]
    pub fn scuscratchmcu63(&mut self) -> Scuscratchmcu63W<Scu8fcSpec> {
        Scuscratchmcu63W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 63\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8fcSpec;
impl crate::RegisterSpec for Scu8fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8fc::R`](R) reader structure"]
impl crate::Readable for Scu8fcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8fc::W`](W) writer structure"]
impl crate::Writable for Scu8fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8FC to value 0"]
impl crate::Resettable for Scu8fcSpec {}
