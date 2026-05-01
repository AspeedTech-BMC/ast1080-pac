#[doc = "Register `SCUB68` reader"]
pub type R = crate::R<Scub68Spec>;
#[doc = "Register `SCUB68` writer"]
pub type W = crate::W<Scub68Spec>;
#[doc = "Field `SCUSWPUF26` reader - SCU_SW_PUF_26"]
pub type Scuswpuf26R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_26"]
    #[inline(always)]
    pub fn scuswpuf26(&self) -> Scuswpuf26R {
        Scuswpuf26R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scub68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub68Spec;
impl crate::RegisterSpec for Scub68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub68::R`](R) reader structure"]
impl crate::Readable for Scub68Spec {}
#[doc = "`write(|w| ..)` method takes [`scub68::W`](W) writer structure"]
impl crate::Writable for Scub68Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB68 to value 0"]
impl crate::Resettable for Scub68Spec {}
