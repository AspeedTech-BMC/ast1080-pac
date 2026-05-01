#[doc = "Register `SCUB18` reader"]
pub type R = crate::R<Scub18Spec>;
#[doc = "Register `SCUB18` writer"]
pub type W = crate::W<Scub18Spec>;
#[doc = "Field `SCUSWPUF6` reader - SCU_SW_PUF_6"]
pub type Scuswpuf6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_6"]
    #[inline(always)]
    pub fn scuswpuf6(&self) -> Scuswpuf6R {
        Scuswpuf6R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scub18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub18Spec;
impl crate::RegisterSpec for Scub18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub18::R`](R) reader structure"]
impl crate::Readable for Scub18Spec {}
#[doc = "`write(|w| ..)` method takes [`scub18::W`](W) writer structure"]
impl crate::Writable for Scub18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB18 to value 0"]
impl crate::Resettable for Scub18Spec {}
