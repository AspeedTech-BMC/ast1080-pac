#[doc = "Register `GSRAM08C` reader"]
pub type R = crate::R<Gsram08cSpec>;
#[doc = "Register `GSRAM08C` writer"]
pub type W = crate::W<Gsram08cSpec>;
#[doc = "Field `WLOCK03` reader - WLOCK03"]
pub type Wlock03R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK03` writer - WLOCK03"]
pub type Wlock03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK03"]
    #[inline(always)]
    pub fn wlock03(&self) -> Wlock03R {
        Wlock03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK03"]
    #[inline(always)]
    pub fn wlock03(&mut self) -> Wlock03W<Gsram08cSpec> {
        Wlock03W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK03\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram08cSpec;
impl crate::RegisterSpec for Gsram08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram08c::R`](R) reader structure"]
impl crate::Readable for Gsram08cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram08c::W`](W) writer structure"]
impl crate::Writable for Gsram08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM08C to value 0"]
impl crate::Resettable for Gsram08cSpec {}
