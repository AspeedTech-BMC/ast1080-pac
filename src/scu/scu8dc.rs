#[doc = "Register `SCU8DC` reader"]
pub type R = crate::R<Scu8dcSpec>;
#[doc = "Register `SCU8DC` writer"]
pub type W = crate::W<Scu8dcSpec>;
#[doc = "Field `SCUSCRATCHMCU55` reader - SCU_SCRATCH_MCU_55"]
pub type Scuscratchmcu55R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU55` writer - SCU_SCRATCH_MCU_55"]
pub type Scuscratchmcu55W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_55"]
    #[inline(always)]
    pub fn scuscratchmcu55(&self) -> Scuscratchmcu55R {
        Scuscratchmcu55R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_55"]
    #[inline(always)]
    pub fn scuscratchmcu55(&mut self) -> Scuscratchmcu55W<Scu8dcSpec> {
        Scuscratchmcu55W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 55\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8dcSpec;
impl crate::RegisterSpec for Scu8dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8dc::R`](R) reader structure"]
impl crate::Readable for Scu8dcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8dc::W`](W) writer structure"]
impl crate::Writable for Scu8dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8DC to value 0"]
impl crate::Resettable for Scu8dcSpec {}
