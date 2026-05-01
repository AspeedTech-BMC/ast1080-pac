#[doc = "Register `SCUB2C` reader"]
pub type R = crate::R<Scub2cSpec>;
#[doc = "Register `SCUB2C` writer"]
pub type W = crate::W<Scub2cSpec>;
#[doc = "Field `SCUSWPUF11` reader - SCU_SW_PUF_11"]
pub type Scuswpuf11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_11"]
    #[inline(always)]
    pub fn scuswpuf11(&self) -> Scuswpuf11R {
        Scuswpuf11R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scub2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub2cSpec;
impl crate::RegisterSpec for Scub2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub2c::R`](R) reader structure"]
impl crate::Readable for Scub2cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub2c::W`](W) writer structure"]
impl crate::Writable for Scub2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB2C to value 0"]
impl crate::Resettable for Scub2cSpec {}
