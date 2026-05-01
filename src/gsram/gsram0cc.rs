#[doc = "Register `GSRAM0CC` reader"]
pub type R = crate::R<Gsram0ccSpec>;
#[doc = "Register `GSRAM0CC` writer"]
pub type W = crate::W<Gsram0ccSpec>;
#[doc = "Field `WLOCK19` reader - WLOCK19"]
pub type Wlock19R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK19` writer - WLOCK19"]
pub type Wlock19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK19"]
    #[inline(always)]
    pub fn wlock19(&self) -> Wlock19R {
        Wlock19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK19"]
    #[inline(always)]
    pub fn wlock19(&mut self) -> Wlock19W<Gsram0ccSpec> {
        Wlock19W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK19\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0ccSpec;
impl crate::RegisterSpec for Gsram0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0cc::R`](R) reader structure"]
impl crate::Readable for Gsram0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0cc::W`](W) writer structure"]
impl crate::Writable for Gsram0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0CC to value 0"]
impl crate::Resettable for Gsram0ccSpec {}
