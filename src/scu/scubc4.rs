#[doc = "Register `SCUBC4` reader"]
pub type R = crate::R<Scubc4Spec>;
#[doc = "Register `SCUBC4` writer"]
pub type W = crate::W<Scubc4Spec>;
#[doc = "Field `SCUHWPUF17` reader - SCU_HW_PUF_17"]
pub type Scuhwpuf17R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_17"]
    #[inline(always)]
    pub fn scuhwpuf17(&self) -> Scuhwpuf17R {
        Scuhwpuf17R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubc4Spec;
impl crate::RegisterSpec for Scubc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubc4::R`](R) reader structure"]
impl crate::Readable for Scubc4Spec {}
#[doc = "`write(|w| ..)` method takes [`scubc4::W`](W) writer structure"]
impl crate::Writable for Scubc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBC4 to value 0"]
impl crate::Resettable for Scubc4Spec {}
