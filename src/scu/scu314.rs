#[doc = "Register `SCU314` reader"]
pub type R = crate::R<Scu314Spec>;
#[doc = "Register `SCU314` writer"]
pub type W = crate::W<Scu314Spec>;
#[doc = "Field `SCUDIPLLMP` reader - SCU_DIPLL_MP"]
pub type ScudipllmpR = crate::FieldReader<u16>;
#[doc = "Field `SCUDIPLLMP` writer - SCU_DIPLL_MP"]
pub type ScudipllmpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SCUDIPLLMS` reader - SCU_DIPLL_MS"]
pub type ScudipllmsR = crate::FieldReader<u16>;
#[doc = "Field `SCUDIPLLMS` writer - SCU_DIPLL_MS"]
pub type ScudipllmsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - SCU_DIPLL_MP"]
    #[inline(always)]
    pub fn scudipllmp(&self) -> ScudipllmpR {
        ScudipllmpR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - SCU_DIPLL_MS"]
    #[inline(always)]
    pub fn scudipllms(&self) -> ScudipllmsR {
        ScudipllmsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - SCU_DIPLL_MP"]
    #[inline(always)]
    pub fn scudipllmp(&mut self) -> ScudipllmpW<Scu314Spec> {
        ScudipllmpW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SCU_DIPLL_MS"]
    #[inline(always)]
    pub fn scudipllms(&mut self) -> ScudipllmsW<Scu314Spec> {
        ScudipllmsW::new(self, 16)
    }
}
#[doc = "DIPLL Parameter Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu314::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu314::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu314Spec;
impl crate::RegisterSpec for Scu314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu314::R`](R) reader structure"]
impl crate::Readable for Scu314Spec {}
#[doc = "`write(|w| ..)` method takes [`scu314::W`](W) writer structure"]
impl crate::Writable for Scu314Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU314 to value 0"]
impl crate::Resettable for Scu314Spec {}
