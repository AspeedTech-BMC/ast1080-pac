#[doc = "Register `SPIF05C` reader"]
pub type R = crate::R<Spif05cSpec>;
#[doc = "Register `SPIF05C` writer"]
pub type W = crate::W<Spif05cSpec>;
#[doc = "Field `ELOG07` reader - ELOG07"]
pub type Elog07R = crate::FieldReader<u32>;
#[doc = "Field `ELOG07` writer - ELOG07"]
pub type Elog07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG07"]
    #[inline(always)]
    pub fn elog07(&self) -> Elog07R {
        Elog07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG07"]
    #[inline(always)]
    pub fn elog07(&mut self) -> Elog07W<Spif05cSpec> {
        Elog07W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif05cSpec;
impl crate::RegisterSpec for Spif05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif05c::R`](R) reader structure"]
impl crate::Readable for Spif05cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif05c::W`](W) writer structure"]
impl crate::Writable for Spif05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF05C to value 0"]
impl crate::Resettable for Spif05cSpec {}
