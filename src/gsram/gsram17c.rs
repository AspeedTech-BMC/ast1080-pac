#[doc = "Register `GSRAM17C` reader"]
pub type R = crate::R<Gsram17cSpec>;
#[doc = "Register `GSRAM17C` writer"]
pub type W = crate::W<Gsram17cSpec>;
#[doc = "Field `WLOCK63` reader - WLOCK63"]
pub type Wlock63R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK63` writer - WLOCK63"]
pub type Wlock63W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK63"]
    #[inline(always)]
    pub fn wlock63(&self) -> Wlock63R {
        Wlock63R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK63"]
    #[inline(always)]
    pub fn wlock63(&mut self) -> Wlock63W<Gsram17cSpec> {
        Wlock63W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK63\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram17cSpec;
impl crate::RegisterSpec for Gsram17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram17c::R`](R) reader structure"]
impl crate::Readable for Gsram17cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram17c::W`](W) writer structure"]
impl crate::Writable for Gsram17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM17C to value 0"]
impl crate::Resettable for Gsram17cSpec {}
