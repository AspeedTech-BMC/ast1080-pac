#[doc = "Register `SCU89C` reader"]
pub type R = crate::R<Scu89cSpec>;
#[doc = "Register `SCU89C` writer"]
pub type W = crate::W<Scu89cSpec>;
#[doc = "Field `SCUSCRATCHMCU39` reader - SCU_SCRATCH_MCU_39"]
pub type Scuscratchmcu39R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU39` writer - SCU_SCRATCH_MCU_39"]
pub type Scuscratchmcu39W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_39"]
    #[inline(always)]
    pub fn scuscratchmcu39(&self) -> Scuscratchmcu39R {
        Scuscratchmcu39R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_39"]
    #[inline(always)]
    pub fn scuscratchmcu39(&mut self) -> Scuscratchmcu39W<Scu89cSpec> {
        Scuscratchmcu39W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 39\n\nYou can [`read`](crate::Reg::read) this register and get [`scu89c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu89c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu89cSpec;
impl crate::RegisterSpec for Scu89cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu89c::R`](R) reader structure"]
impl crate::Readable for Scu89cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu89c::W`](W) writer structure"]
impl crate::Writable for Scu89cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU89C to value 0"]
impl crate::Resettable for Scu89cSpec {}
