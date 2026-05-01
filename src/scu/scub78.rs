#[doc = "Register `SCUB78` reader"]
pub type R = crate::R<Scub78Spec>;
#[doc = "Register `SCUB78` writer"]
pub type W = crate::W<Scub78Spec>;
#[doc = "Field `SCUSWPUF30` reader - SCU_SW_PUF_30"]
pub type Scuswpuf30R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_30"]
    #[inline(always)]
    pub fn scuswpuf30(&self) -> Scuswpuf30R {
        Scuswpuf30R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scub78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub78Spec;
impl crate::RegisterSpec for Scub78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub78::R`](R) reader structure"]
impl crate::Readable for Scub78Spec {}
#[doc = "`write(|w| ..)` method takes [`scub78::W`](W) writer structure"]
impl crate::Writable for Scub78Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB78 to value 0"]
impl crate::Resettable for Scub78Spec {}
