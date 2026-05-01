#[doc = "Register `SCUB40` reader"]
pub type R = crate::R<Scub40Spec>;
#[doc = "Register `SCUB40` writer"]
pub type W = crate::W<Scub40Spec>;
#[doc = "Field `SCUSWPUF16` reader - SCU_SW_PUF_16"]
pub type Scuswpuf16R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_16"]
    #[inline(always)]
    pub fn scuswpuf16(&self) -> Scuswpuf16R {
        Scuswpuf16R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scub40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub40Spec;
impl crate::RegisterSpec for Scub40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub40::R`](R) reader structure"]
impl crate::Readable for Scub40Spec {}
#[doc = "`write(|w| ..)` method takes [`scub40::W`](W) writer structure"]
impl crate::Writable for Scub40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB40 to value 0"]
impl crate::Resettable for Scub40Spec {}
