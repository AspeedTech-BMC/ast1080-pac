#[doc = "Register `SCUBF4` reader"]
pub type R = crate::R<Scubf4Spec>;
#[doc = "Register `SCUBF4` writer"]
pub type W = crate::W<Scubf4Spec>;
#[doc = "Field `SCUHWPUF29` reader - SCU_HW_PUF_29"]
pub type Scuhwpuf29R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_29"]
    #[inline(always)]
    pub fn scuhwpuf29(&self) -> Scuhwpuf29R {
        Scuhwpuf29R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubf4Spec;
impl crate::RegisterSpec for Scubf4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubf4::R`](R) reader structure"]
impl crate::Readable for Scubf4Spec {}
#[doc = "`write(|w| ..)` method takes [`scubf4::W`](W) writer structure"]
impl crate::Writable for Scubf4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBF4 to value 0"]
impl crate::Resettable for Scubf4Spec {}
