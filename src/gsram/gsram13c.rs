#[doc = "Register `GSRAM13C` reader"]
pub type R = crate::R<Gsram13cSpec>;
#[doc = "Register `GSRAM13C` writer"]
pub type W = crate::W<Gsram13cSpec>;
#[doc = "Field `WLOCK47` reader - WLOCK47"]
pub type Wlock47R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK47` writer - WLOCK47"]
pub type Wlock47W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK47"]
    #[inline(always)]
    pub fn wlock47(&self) -> Wlock47R {
        Wlock47R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK47"]
    #[inline(always)]
    pub fn wlock47(&mut self) -> Wlock47W<Gsram13cSpec> {
        Wlock47W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK47\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram13cSpec;
impl crate::RegisterSpec for Gsram13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram13c::R`](R) reader structure"]
impl crate::Readable for Gsram13cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram13c::W`](W) writer structure"]
impl crate::Writable for Gsram13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM13C to value 0"]
impl crate::Resettable for Gsram13cSpec {}
