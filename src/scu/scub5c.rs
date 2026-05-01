#[doc = "Register `SCUB5C` reader"]
pub type R = crate::R<Scub5cSpec>;
#[doc = "Register `SCUB5C` writer"]
pub type W = crate::W<Scub5cSpec>;
#[doc = "Field `SCUSWPUF23` reader - SCU_SW_PUF_23"]
pub type Scuswpuf23R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_23"]
    #[inline(always)]
    pub fn scuswpuf23(&self) -> Scuswpuf23R {
        Scuswpuf23R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scub5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub5cSpec;
impl crate::RegisterSpec for Scub5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub5c::R`](R) reader structure"]
impl crate::Readable for Scub5cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub5c::W`](W) writer structure"]
impl crate::Writable for Scub5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB5C to value 0"]
impl crate::Resettable for Scub5cSpec {}
