#[doc = "Register `SCUBB4` reader"]
pub type R = crate::R<Scubb4Spec>;
#[doc = "Register `SCUBB4` writer"]
pub type W = crate::W<Scubb4Spec>;
#[doc = "Field `SCUHWPUF13` reader - SCU_HW_PUF_13"]
pub type Scuhwpuf13R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_13"]
    #[inline(always)]
    pub fn scuhwpuf13(&self) -> Scuhwpuf13R {
        Scuhwpuf13R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubb4Spec;
impl crate::RegisterSpec for Scubb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubb4::R`](R) reader structure"]
impl crate::Readable for Scubb4Spec {}
#[doc = "`write(|w| ..)` method takes [`scubb4::W`](W) writer structure"]
impl crate::Writable for Scubb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBB4 to value 0"]
impl crate::Resettable for Scubb4Spec {}
