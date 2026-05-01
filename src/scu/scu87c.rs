#[doc = "Register `SCU87C` reader"]
pub type R = crate::R<Scu87cSpec>;
#[doc = "Register `SCU87C` writer"]
pub type W = crate::W<Scu87cSpec>;
#[doc = "Field `SCUSCRATCHMCU31` reader - SCU_SCRATCH_MCU_31"]
pub type Scuscratchmcu31R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU31` writer - SCU_SCRATCH_MCU_31"]
pub type Scuscratchmcu31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_31"]
    #[inline(always)]
    pub fn scuscratchmcu31(&self) -> Scuscratchmcu31R {
        Scuscratchmcu31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_31"]
    #[inline(always)]
    pub fn scuscratchmcu31(&mut self) -> Scuscratchmcu31W<Scu87cSpec> {
        Scuscratchmcu31W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu87c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu87c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu87cSpec;
impl crate::RegisterSpec for Scu87cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu87c::R`](R) reader structure"]
impl crate::Readable for Scu87cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu87c::W`](W) writer structure"]
impl crate::Writable for Scu87cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU87C to value 0"]
impl crate::Resettable for Scu87cSpec {}
