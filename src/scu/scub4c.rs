#[doc = "Register `SCUB4C` reader"]
pub type R = crate::R<Scub4cSpec>;
#[doc = "Register `SCUB4C` writer"]
pub type W = crate::W<Scub4cSpec>;
#[doc = "Field `SCUSWPUF19` reader - SCU_SW_PUF_19"]
pub type Scuswpuf19R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_19"]
    #[inline(always)]
    pub fn scuswpuf19(&self) -> Scuswpuf19R {
        Scuswpuf19R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scub4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub4cSpec;
impl crate::RegisterSpec for Scub4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub4c::R`](R) reader structure"]
impl crate::Readable for Scub4cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub4c::W`](W) writer structure"]
impl crate::Writable for Scub4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB4C to value 0"]
impl crate::Resettable for Scub4cSpec {}
