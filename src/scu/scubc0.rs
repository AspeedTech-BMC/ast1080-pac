#[doc = "Register `SCUBC0` reader"]
pub type R = crate::R<Scubc0Spec>;
#[doc = "Register `SCUBC0` writer"]
pub type W = crate::W<Scubc0Spec>;
#[doc = "Field `SCUHWPUF16` reader - SCU_HW_PUF_16"]
pub type Scuhwpuf16R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_16"]
    #[inline(always)]
    pub fn scuhwpuf16(&self) -> Scuhwpuf16R {
        Scuhwpuf16R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubc0Spec;
impl crate::RegisterSpec for Scubc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubc0::R`](R) reader structure"]
impl crate::Readable for Scubc0Spec {}
#[doc = "`write(|w| ..)` method takes [`scubc0::W`](W) writer structure"]
impl crate::Writable for Scubc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBC0 to value 0"]
impl crate::Resettable for Scubc0Spec {}
