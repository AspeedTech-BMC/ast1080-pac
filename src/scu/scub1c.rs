#[doc = "Register `SCUB1C` reader"]
pub type R = crate::R<Scub1cSpec>;
#[doc = "Register `SCUB1C` writer"]
pub type W = crate::W<Scub1cSpec>;
#[doc = "Field `SCUSWPUF7` reader - SCU_SW_PUF_7"]
pub type Scuswpuf7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_7"]
    #[inline(always)]
    pub fn scuswpuf7(&self) -> Scuswpuf7R {
        Scuswpuf7R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scub1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub1cSpec;
impl crate::RegisterSpec for Scub1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub1c::R`](R) reader structure"]
impl crate::Readable for Scub1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub1c::W`](W) writer structure"]
impl crate::Writable for Scub1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB1C to value 0"]
impl crate::Resettable for Scub1cSpec {}
