#[doc = "Register `SCUB60` reader"]
pub type R = crate::R<Scub60Spec>;
#[doc = "Register `SCUB60` writer"]
pub type W = crate::W<Scub60Spec>;
#[doc = "Field `SCUSWPUF24` reader - SCU_SW_PUF_24"]
pub type Scuswpuf24R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_24"]
    #[inline(always)]
    pub fn scuswpuf24(&self) -> Scuswpuf24R {
        Scuswpuf24R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scub60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub60Spec;
impl crate::RegisterSpec for Scub60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub60::R`](R) reader structure"]
impl crate::Readable for Scub60Spec {}
#[doc = "`write(|w| ..)` method takes [`scub60::W`](W) writer structure"]
impl crate::Writable for Scub60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB60 to value 0"]
impl crate::Resettable for Scub60Spec {}
