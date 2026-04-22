#[doc = "Register `SPIF0FC` reader"]
pub type R = crate::R<Spif0fcSpec>;
#[doc = "Register `SPIF0FC` writer"]
pub type W = crate::W<Spif0fcSpec>;
#[doc = "Field `WTABLE31` reader - WTABLE31"]
pub type Wtable31R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE31` writer - WTABLE31"]
pub type Wtable31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE31"]
    #[inline(always)]
    pub fn wtable31(&self) -> Wtable31R {
        Wtable31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE31"]
    #[inline(always)]
    pub fn wtable31(&mut self) -> Wtable31W<Spif0fcSpec> {
        Wtable31W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE31\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0fcSpec;
impl crate::RegisterSpec for Spif0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0fc::R`](R) reader structure"]
impl crate::Readable for Spif0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0fc::W`](W) writer structure"]
impl crate::Writable for Spif0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0FC to value 0"]
impl crate::Resettable for Spif0fcSpec {}
