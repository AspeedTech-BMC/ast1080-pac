#[doc = "Register `SCUB3C` reader"]
pub type R = crate::R<Scub3cSpec>;
#[doc = "Register `SCUB3C` writer"]
pub type W = crate::W<Scub3cSpec>;
#[doc = "Field `SCUSWPUF15` reader - SCU_SW_PUF_15"]
pub type Scuswpuf15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_15"]
    #[inline(always)]
    pub fn scuswpuf15(&self) -> Scuswpuf15R {
        Scuswpuf15R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scub3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub3cSpec;
impl crate::RegisterSpec for Scub3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub3c::R`](R) reader structure"]
impl crate::Readable for Scub3cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub3c::W`](W) writer structure"]
impl crate::Writable for Scub3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB3C to value 0"]
impl crate::Resettable for Scub3cSpec {}
