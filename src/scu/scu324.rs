#[doc = "Register `SCU324` reader"]
pub type R = crate::R<Scu324Spec>;
#[doc = "Register `SCU324` writer"]
pub type W = crate::W<Scu324Spec>;
#[doc = "Field `SCUDIPLLFHMS` reader - SCU_DIPLL_FH_MS"]
pub type ScudipllfhmsR = crate::FieldReader<u32>;
#[doc = "Field `SCUDIPLLFHMS` writer - SCU_DIPLL_FH_MS"]
pub type ScudipllfhmsW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - SCU_DIPLL_FH_MS"]
    #[inline(always)]
    pub fn scudipllfhms(&self) -> ScudipllfhmsR {
        ScudipllfhmsR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - SCU_DIPLL_FH_MS"]
    #[inline(always)]
    pub fn scudipllfhms(&mut self) -> ScudipllfhmsW<Scu324Spec> {
        ScudipllfhmsW::new(self, 0)
    }
}
#[doc = "DIPLL Parameter Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu324::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu324::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu324Spec;
impl crate::RegisterSpec for Scu324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu324::R`](R) reader structure"]
impl crate::Readable for Scu324Spec {}
#[doc = "`write(|w| ..)` method takes [`scu324::W`](W) writer structure"]
impl crate::Writable for Scu324Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU324 to value 0"]
impl crate::Resettable for Scu324Spec {}
