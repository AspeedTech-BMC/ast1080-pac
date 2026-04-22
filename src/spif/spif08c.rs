#[doc = "Register `SPIF08C` reader"]
pub type R = crate::R<Spif08cSpec>;
#[doc = "Register `SPIF08C` writer"]
pub type W = crate::W<Spif08cSpec>;
#[doc = "Field `WTABLE03` reader - WTABLE03"]
pub type Wtable03R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE03` writer - WTABLE03"]
pub type Wtable03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE03"]
    #[inline(always)]
    pub fn wtable03(&self) -> Wtable03R {
        Wtable03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE03"]
    #[inline(always)]
    pub fn wtable03(&mut self) -> Wtable03W<Spif08cSpec> {
        Wtable03W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif08cSpec;
impl crate::RegisterSpec for Spif08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif08c::R`](R) reader structure"]
impl crate::Readable for Spif08cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif08c::W`](W) writer structure"]
impl crate::Writable for Spif08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF08C to value 0"]
impl crate::Resettable for Spif08cSpec {}
