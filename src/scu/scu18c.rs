#[doc = "Register `SCU18C` reader"]
pub type R = crate::R<Scu18cSpec>;
#[doc = "Register `SCU18C` writer"]
pub type W = crate::W<Scu18cSpec>;
#[doc = "Field `SCUSCRATCH4` reader - SCU_SCRATCH_4"]
pub type Scuscratch4R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH4` writer - SCU_SCRATCH_4"]
pub type Scuscratch4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_4"]
    #[inline(always)]
    pub fn scuscratch4(&self) -> Scuscratch4R {
        Scuscratch4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_4"]
    #[inline(always)]
    pub fn scuscratch4(&mut self) -> Scuscratch4W<Scu18cSpec> {
        Scuscratch4W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu18c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu18c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu18cSpec;
impl crate::RegisterSpec for Scu18cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu18c::R`](R) reader structure"]
impl crate::Readable for Scu18cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu18c::W`](W) writer structure"]
impl crate::Writable for Scu18cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU18C to value 0"]
impl crate::Resettable for Scu18cSpec {}
