#[doc = "Register `GSRAM09C` reader"]
pub type R = crate::R<Gsram09cSpec>;
#[doc = "Register `GSRAM09C` writer"]
pub type W = crate::W<Gsram09cSpec>;
#[doc = "Field `WLOCK07` reader - WLOCK07"]
pub type Wlock07R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK07` writer - WLOCK07"]
pub type Wlock07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK07"]
    #[inline(always)]
    pub fn wlock07(&self) -> Wlock07R {
        Wlock07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK07"]
    #[inline(always)]
    pub fn wlock07(&mut self) -> Wlock07W<Gsram09cSpec> {
        Wlock07W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK07\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram09cSpec;
impl crate::RegisterSpec for Gsram09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram09c::R`](R) reader structure"]
impl crate::Readable for Gsram09cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram09c::W`](W) writer structure"]
impl crate::Writable for Gsram09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM09C to value 0"]
impl crate::Resettable for Gsram09cSpec {}
