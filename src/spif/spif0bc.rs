#[doc = "Register `SPIF0BC` reader"]
pub type R = crate::R<Spif0bcSpec>;
#[doc = "Register `SPIF0BC` writer"]
pub type W = crate::W<Spif0bcSpec>;
#[doc = "Field `WTABLE15` reader - WTABLE15"]
pub type Wtable15R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE15` writer - WTABLE15"]
pub type Wtable15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE15"]
    #[inline(always)]
    pub fn wtable15(&self) -> Wtable15R {
        Wtable15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE15"]
    #[inline(always)]
    pub fn wtable15(&mut self) -> Wtable15W<Spif0bcSpec> {
        Wtable15W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0bcSpec;
impl crate::RegisterSpec for Spif0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0bc::R`](R) reader structure"]
impl crate::Readable for Spif0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0bc::W`](W) writer structure"]
impl crate::Writable for Spif0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0BC to value 0"]
impl crate::Resettable for Spif0bcSpec {}
