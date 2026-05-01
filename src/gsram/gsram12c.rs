#[doc = "Register `GSRAM12C` reader"]
pub type R = crate::R<Gsram12cSpec>;
#[doc = "Register `GSRAM12C` writer"]
pub type W = crate::W<Gsram12cSpec>;
#[doc = "Field `WLOCK43` reader - WLOCK43"]
pub type Wlock43R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK43` writer - WLOCK43"]
pub type Wlock43W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK43"]
    #[inline(always)]
    pub fn wlock43(&self) -> Wlock43R {
        Wlock43R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK43"]
    #[inline(always)]
    pub fn wlock43(&mut self) -> Wlock43W<Gsram12cSpec> {
        Wlock43W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK43\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram12cSpec;
impl crate::RegisterSpec for Gsram12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram12c::R`](R) reader structure"]
impl crate::Readable for Gsram12cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram12c::W`](W) writer structure"]
impl crate::Writable for Gsram12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM12C to value 0"]
impl crate::Resettable for Gsram12cSpec {}
