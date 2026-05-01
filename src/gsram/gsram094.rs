#[doc = "Register `GSRAM094` reader"]
pub type R = crate::R<Gsram094Spec>;
#[doc = "Register `GSRAM094` writer"]
pub type W = crate::W<Gsram094Spec>;
#[doc = "Field `WLOCK05` reader - WLOCK05"]
pub type Wlock05R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK05` writer - WLOCK05"]
pub type Wlock05W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK05"]
    #[inline(always)]
    pub fn wlock05(&self) -> Wlock05R {
        Wlock05R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK05"]
    #[inline(always)]
    pub fn wlock05(&mut self) -> Wlock05W<Gsram094Spec> {
        Wlock05W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK05\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram094Spec;
impl crate::RegisterSpec for Gsram094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram094::R`](R) reader structure"]
impl crate::Readable for Gsram094Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram094::W`](W) writer structure"]
impl crate::Writable for Gsram094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM094 to value 0"]
impl crate::Resettable for Gsram094Spec {}
