#[doc = "Register `SPIF04C` reader"]
pub type R = crate::R<Spif04cSpec>;
#[doc = "Register `SPIF04C` writer"]
pub type W = crate::W<Spif04cSpec>;
#[doc = "Field `ELOG03` reader - ELOG03"]
pub type Elog03R = crate::FieldReader<u32>;
#[doc = "Field `ELOG03` writer - ELOG03"]
pub type Elog03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG03"]
    #[inline(always)]
    pub fn elog03(&self) -> Elog03R {
        Elog03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG03"]
    #[inline(always)]
    pub fn elog03(&mut self) -> Elog03W<Spif04cSpec> {
        Elog03W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif04cSpec;
impl crate::RegisterSpec for Spif04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif04c::R`](R) reader structure"]
impl crate::Readable for Spif04cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif04c::W`](W) writer structure"]
impl crate::Writable for Spif04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF04C to value 0"]
impl crate::Resettable for Spif04cSpec {}
