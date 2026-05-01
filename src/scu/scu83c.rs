#[doc = "Register `SCU83C` reader"]
pub type R = crate::R<Scu83cSpec>;
#[doc = "Register `SCU83C` writer"]
pub type W = crate::W<Scu83cSpec>;
#[doc = "Field `SCUSCRATCHMCU15` reader - SCU_SCRATCH_MCU_15"]
pub type Scuscratchmcu15R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU15` writer - SCU_SCRATCH_MCU_15"]
pub type Scuscratchmcu15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_15"]
    #[inline(always)]
    pub fn scuscratchmcu15(&self) -> Scuscratchmcu15R {
        Scuscratchmcu15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_15"]
    #[inline(always)]
    pub fn scuscratchmcu15(&mut self) -> Scuscratchmcu15W<Scu83cSpec> {
        Scuscratchmcu15W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu83c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu83c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu83cSpec;
impl crate::RegisterSpec for Scu83cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu83c::R`](R) reader structure"]
impl crate::Readable for Scu83cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu83c::W`](W) writer structure"]
impl crate::Writable for Scu83cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU83C to value 0"]
impl crate::Resettable for Scu83cSpec {}
