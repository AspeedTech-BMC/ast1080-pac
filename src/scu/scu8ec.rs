#[doc = "Register `SCU8EC` reader"]
pub type R = crate::R<Scu8ecSpec>;
#[doc = "Register `SCU8EC` writer"]
pub type W = crate::W<Scu8ecSpec>;
#[doc = "Field `SCUSCRATCHMCU59` reader - SCU_SCRATCH_MCU_59"]
pub type Scuscratchmcu59R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU59` writer - SCU_SCRATCH_MCU_59"]
pub type Scuscratchmcu59W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_59"]
    #[inline(always)]
    pub fn scuscratchmcu59(&self) -> Scuscratchmcu59R {
        Scuscratchmcu59R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_59"]
    #[inline(always)]
    pub fn scuscratchmcu59(&mut self) -> Scuscratchmcu59W<Scu8ecSpec> {
        Scuscratchmcu59W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 59\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8ecSpec;
impl crate::RegisterSpec for Scu8ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8ec::R`](R) reader structure"]
impl crate::Readable for Scu8ecSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8ec::W`](W) writer structure"]
impl crate::Writable for Scu8ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8EC to value 0"]
impl crate::Resettable for Scu8ecSpec {}
