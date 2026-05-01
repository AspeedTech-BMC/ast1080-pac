#[doc = "Register `SCUB0C` reader"]
pub type R = crate::R<Scub0cSpec>;
#[doc = "Register `SCUB0C` writer"]
pub type W = crate::W<Scub0cSpec>;
#[doc = "Field `SCUSWPUF3` reader - SCU_SW_PUF_3"]
pub type Scuswpuf3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_3"]
    #[inline(always)]
    pub fn scuswpuf3(&self) -> Scuswpuf3R {
        Scuswpuf3R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scub0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub0cSpec;
impl crate::RegisterSpec for Scub0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub0c::R`](R) reader structure"]
impl crate::Readable for Scub0cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub0c::W`](W) writer structure"]
impl crate::Writable for Scub0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB0C to value 0"]
impl crate::Resettable for Scub0cSpec {}
