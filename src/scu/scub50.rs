#[doc = "Register `SCUB50` reader"]
pub type R = crate::R<Scub50Spec>;
#[doc = "Register `SCUB50` writer"]
pub type W = crate::W<Scub50Spec>;
#[doc = "Field `SCUSWPUF20` reader - SCU_SW_PUF_20"]
pub type Scuswpuf20R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_20"]
    #[inline(always)]
    pub fn scuswpuf20(&self) -> Scuswpuf20R {
        Scuswpuf20R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scub50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub50Spec;
impl crate::RegisterSpec for Scub50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub50::R`](R) reader structure"]
impl crate::Readable for Scub50Spec {}
#[doc = "`write(|w| ..)` method takes [`scub50::W`](W) writer structure"]
impl crate::Writable for Scub50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB50 to value 0"]
impl crate::Resettable for Scub50Spec {}
