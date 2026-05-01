#[doc = "Register `SCUB58` reader"]
pub type R = crate::R<Scub58Spec>;
#[doc = "Register `SCUB58` writer"]
pub type W = crate::W<Scub58Spec>;
#[doc = "Field `SCUSWPUF22` reader - SCU_SW_PUF_22"]
pub type Scuswpuf22R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_22"]
    #[inline(always)]
    pub fn scuswpuf22(&self) -> Scuswpuf22R {
        Scuswpuf22R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scub58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub58Spec;
impl crate::RegisterSpec for Scub58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub58::R`](R) reader structure"]
impl crate::Readable for Scub58Spec {}
#[doc = "`write(|w| ..)` method takes [`scub58::W`](W) writer structure"]
impl crate::Writable for Scub58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB58 to value 0"]
impl crate::Resettable for Scub58Spec {}
