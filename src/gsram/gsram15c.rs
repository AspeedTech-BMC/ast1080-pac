#[doc = "Register `GSRAM15C` reader"]
pub type R = crate::R<Gsram15cSpec>;
#[doc = "Register `GSRAM15C` writer"]
pub type W = crate::W<Gsram15cSpec>;
#[doc = "Field `WLOCK55` reader - WLOCK55"]
pub type Wlock55R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK55` writer - WLOCK55"]
pub type Wlock55W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK55"]
    #[inline(always)]
    pub fn wlock55(&self) -> Wlock55R {
        Wlock55R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK55"]
    #[inline(always)]
    pub fn wlock55(&mut self) -> Wlock55W<Gsram15cSpec> {
        Wlock55W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK55\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram15cSpec;
impl crate::RegisterSpec for Gsram15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram15c::R`](R) reader structure"]
impl crate::Readable for Gsram15cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram15c::W`](W) writer structure"]
impl crate::Writable for Gsram15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM15C to value 0"]
impl crate::Resettable for Gsram15cSpec {}
