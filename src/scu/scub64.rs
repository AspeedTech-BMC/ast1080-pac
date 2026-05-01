#[doc = "Register `SCUB64` reader"]
pub type R = crate::R<Scub64Spec>;
#[doc = "Register `SCUB64` writer"]
pub type W = crate::W<Scub64Spec>;
#[doc = "Field `SCUSWPUF25` reader - SCU_SW_PUF_25"]
pub type Scuswpuf25R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_25"]
    #[inline(always)]
    pub fn scuswpuf25(&self) -> Scuswpuf25R {
        Scuswpuf25R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`scub64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub64Spec;
impl crate::RegisterSpec for Scub64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub64::R`](R) reader structure"]
impl crate::Readable for Scub64Spec {}
#[doc = "`write(|w| ..)` method takes [`scub64::W`](W) writer structure"]
impl crate::Writable for Scub64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB64 to value 0"]
impl crate::Resettable for Scub64Spec {}
