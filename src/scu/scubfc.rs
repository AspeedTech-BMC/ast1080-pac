#[doc = "Register `SCUBFC` reader"]
pub type R = crate::R<ScubfcSpec>;
#[doc = "Register `SCUBFC` writer"]
pub type W = crate::W<ScubfcSpec>;
#[doc = "Field `SCUHWPUF31` reader - SCU_HW_PUF_31"]
pub type Scuhwpuf31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_31"]
    #[inline(always)]
    pub fn scuhwpuf31(&self) -> Scuhwpuf31R {
        Scuhwpuf31R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scubfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubfcSpec;
impl crate::RegisterSpec for ScubfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubfc::R`](R) reader structure"]
impl crate::Readable for ScubfcSpec {}
#[doc = "`write(|w| ..)` method takes [`scubfc::W`](W) writer structure"]
impl crate::Writable for ScubfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBFC to value 0"]
impl crate::Resettable for ScubfcSpec {}
