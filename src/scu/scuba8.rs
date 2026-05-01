#[doc = "Register `SCUBA8` reader"]
pub type R = crate::R<Scuba8Spec>;
#[doc = "Register `SCUBA8` writer"]
pub type W = crate::W<Scuba8Spec>;
#[doc = "Field `SCUHWPUF10` reader - SCU_HW_PUF_10"]
pub type Scuhwpuf10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_10"]
    #[inline(always)]
    pub fn scuhwpuf10(&self) -> Scuhwpuf10R {
        Scuhwpuf10R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuba8Spec;
impl crate::RegisterSpec for Scuba8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuba8::R`](R) reader structure"]
impl crate::Readable for Scuba8Spec {}
#[doc = "`write(|w| ..)` method takes [`scuba8::W`](W) writer structure"]
impl crate::Writable for Scuba8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBA8 to value 0"]
impl crate::Resettable for Scuba8Spec {}
