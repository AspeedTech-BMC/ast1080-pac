#[doc = "Register `SPIF09C` reader"]
pub type R = crate::R<Spif09cSpec>;
#[doc = "Register `SPIF09C` writer"]
pub type W = crate::W<Spif09cSpec>;
#[doc = "Field `WTABLE07` reader - WTABLE07"]
pub type Wtable07R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE07` writer - WTABLE07"]
pub type Wtable07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE07"]
    #[inline(always)]
    pub fn wtable07(&self) -> Wtable07R {
        Wtable07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE07"]
    #[inline(always)]
    pub fn wtable07(&mut self) -> Wtable07W<Spif09cSpec> {
        Wtable07W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif09cSpec;
impl crate::RegisterSpec for Spif09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif09c::R`](R) reader structure"]
impl crate::Readable for Spif09cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif09c::W`](W) writer structure"]
impl crate::Writable for Spif09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF09C to value 0"]
impl crate::Resettable for Spif09cSpec {}
