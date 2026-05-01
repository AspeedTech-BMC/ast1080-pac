#[doc = "Register `GSRAM14C` reader"]
pub type R = crate::R<Gsram14cSpec>;
#[doc = "Register `GSRAM14C` writer"]
pub type W = crate::W<Gsram14cSpec>;
#[doc = "Field `WLOCK51` reader - WLOCK51"]
pub type Wlock51R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK51` writer - WLOCK51"]
pub type Wlock51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK51"]
    #[inline(always)]
    pub fn wlock51(&self) -> Wlock51R {
        Wlock51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK51"]
    #[inline(always)]
    pub fn wlock51(&mut self) -> Wlock51W<Gsram14cSpec> {
        Wlock51W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK51\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram14cSpec;
impl crate::RegisterSpec for Gsram14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram14c::R`](R) reader structure"]
impl crate::Readable for Gsram14cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram14c::W`](W) writer structure"]
impl crate::Writable for Gsram14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM14C to value 0"]
impl crate::Resettable for Gsram14cSpec {}
