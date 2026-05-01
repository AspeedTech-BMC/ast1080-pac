#[doc = "Register `SCU82C` reader"]
pub type R = crate::R<Scu82cSpec>;
#[doc = "Register `SCU82C` writer"]
pub type W = crate::W<Scu82cSpec>;
#[doc = "Field `SCUSCRATCHMCU11` reader - SCU_SCRATCH_MCU_11"]
pub type Scuscratchmcu11R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU11` writer - SCU_SCRATCH_MCU_11"]
pub type Scuscratchmcu11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_11"]
    #[inline(always)]
    pub fn scuscratchmcu11(&self) -> Scuscratchmcu11R {
        Scuscratchmcu11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_11"]
    #[inline(always)]
    pub fn scuscratchmcu11(&mut self) -> Scuscratchmcu11W<Scu82cSpec> {
        Scuscratchmcu11W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu82c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu82c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu82cSpec;
impl crate::RegisterSpec for Scu82cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu82c::R`](R) reader structure"]
impl crate::Readable for Scu82cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu82c::W`](W) writer structure"]
impl crate::Writable for Scu82cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU82C to value 0"]
impl crate::Resettable for Scu82cSpec {}
