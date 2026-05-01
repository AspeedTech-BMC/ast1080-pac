#[doc = "Register `SCUB30` reader"]
pub type R = crate::R<Scub30Spec>;
#[doc = "Register `SCUB30` writer"]
pub type W = crate::W<Scub30Spec>;
#[doc = "Field `SCUSWPUF12` reader - SCU_SW_PUF_12"]
pub type Scuswpuf12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_12"]
    #[inline(always)]
    pub fn scuswpuf12(&self) -> Scuswpuf12R {
        Scuswpuf12R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scub30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub30Spec;
impl crate::RegisterSpec for Scub30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub30::R`](R) reader structure"]
impl crate::Readable for Scub30Spec {}
#[doc = "`write(|w| ..)` method takes [`scub30::W`](W) writer structure"]
impl crate::Writable for Scub30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB30 to value 0"]
impl crate::Resettable for Scub30Spec {}
