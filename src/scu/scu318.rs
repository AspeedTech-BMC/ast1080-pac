#[doc = "Register `SCU318` reader"]
pub type R = crate::R<Scu318Spec>;
#[doc = "Register `SCU318` writer"]
pub type W = crate::W<Scu318Spec>;
#[doc = "Field `SCUDIPLLFOFF` reader - SCU_DIPLL_FOFF"]
pub type ScudipllfoffR = crate::FieldReader<u32>;
#[doc = "Field `SCUDIPLLFOFF` writer - SCU_DIPLL_FOFF"]
pub type ScudipllfoffW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - SCU_DIPLL_FOFF"]
    #[inline(always)]
    pub fn scudipllfoff(&self) -> ScudipllfoffR {
        ScudipllfoffR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - SCU_DIPLL_FOFF"]
    #[inline(always)]
    pub fn scudipllfoff(&mut self) -> ScudipllfoffW<Scu318Spec> {
        ScudipllfoffW::new(self, 0)
    }
}
#[doc = "DIPLL Parameter Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu318::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu318::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu318Spec;
impl crate::RegisterSpec for Scu318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu318::R`](R) reader structure"]
impl crate::Readable for Scu318Spec {}
#[doc = "`write(|w| ..)` method takes [`scu318::W`](W) writer structure"]
impl crate::Writable for Scu318Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU318 to value 0"]
impl crate::Resettable for Scu318Spec {}
