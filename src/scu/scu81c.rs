#[doc = "Register `SCU81C` reader"]
pub type R = crate::R<Scu81cSpec>;
#[doc = "Register `SCU81C` writer"]
pub type W = crate::W<Scu81cSpec>;
#[doc = "Field `SCUSCRATCHMCU7` reader - SCU_SCRATCH_MCU_7"]
pub type Scuscratchmcu7R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU7` writer - SCU_SCRATCH_MCU_7"]
pub type Scuscratchmcu7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_7"]
    #[inline(always)]
    pub fn scuscratchmcu7(&self) -> Scuscratchmcu7R {
        Scuscratchmcu7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_7"]
    #[inline(always)]
    pub fn scuscratchmcu7(&mut self) -> Scuscratchmcu7W<Scu81cSpec> {
        Scuscratchmcu7W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu81c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu81c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu81cSpec;
impl crate::RegisterSpec for Scu81cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu81c::R`](R) reader structure"]
impl crate::Readable for Scu81cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu81c::W`](W) writer structure"]
impl crate::Writable for Scu81cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU81C to value 0"]
impl crate::Resettable for Scu81cSpec {}
