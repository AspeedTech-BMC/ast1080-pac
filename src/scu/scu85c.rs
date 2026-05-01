#[doc = "Register `SCU85C` reader"]
pub type R = crate::R<Scu85cSpec>;
#[doc = "Register `SCU85C` writer"]
pub type W = crate::W<Scu85cSpec>;
#[doc = "Field `SCUSCRATCHMCU23` reader - SCU_SCRATCH_MCU_23"]
pub type Scuscratchmcu23R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU23` writer - SCU_SCRATCH_MCU_23"]
pub type Scuscratchmcu23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_23"]
    #[inline(always)]
    pub fn scuscratchmcu23(&self) -> Scuscratchmcu23R {
        Scuscratchmcu23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_23"]
    #[inline(always)]
    pub fn scuscratchmcu23(&mut self) -> Scuscratchmcu23W<Scu85cSpec> {
        Scuscratchmcu23W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu85c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu85c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu85cSpec;
impl crate::RegisterSpec for Scu85cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu85c::R`](R) reader structure"]
impl crate::Readable for Scu85cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu85c::W`](W) writer structure"]
impl crate::Writable for Scu85cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU85C to value 0"]
impl crate::Resettable for Scu85cSpec {}
