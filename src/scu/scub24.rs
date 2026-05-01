#[doc = "Register `SCUB24` reader"]
pub type R = crate::R<Scub24Spec>;
#[doc = "Register `SCUB24` writer"]
pub type W = crate::W<Scub24Spec>;
#[doc = "Field `SCUSWPUF9` reader - SCU_SW_PUF_9"]
pub type Scuswpuf9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_9"]
    #[inline(always)]
    pub fn scuswpuf9(&self) -> Scuswpuf9R {
        Scuswpuf9R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scub24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub24Spec;
impl crate::RegisterSpec for Scub24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub24::R`](R) reader structure"]
impl crate::Readable for Scub24Spec {}
#[doc = "`write(|w| ..)` method takes [`scub24::W`](W) writer structure"]
impl crate::Writable for Scub24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB24 to value 0"]
impl crate::Resettable for Scub24Spec {}
