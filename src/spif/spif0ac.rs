#[doc = "Register `SPIF0AC` reader"]
pub type R = crate::R<Spif0acSpec>;
#[doc = "Register `SPIF0AC` writer"]
pub type W = crate::W<Spif0acSpec>;
#[doc = "Field `WTABLE11` reader - WTABLE11"]
pub type Wtable11R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE11` writer - WTABLE11"]
pub type Wtable11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE11"]
    #[inline(always)]
    pub fn wtable11(&self) -> Wtable11R {
        Wtable11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE11"]
    #[inline(always)]
    pub fn wtable11(&mut self) -> Wtable11W<Spif0acSpec> {
        Wtable11W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0acSpec;
impl crate::RegisterSpec for Spif0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0ac::R`](R) reader structure"]
impl crate::Readable for Spif0acSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0ac::W`](W) writer structure"]
impl crate::Writable for Spif0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0AC to value 0"]
impl crate::Resettable for Spif0acSpec {}
