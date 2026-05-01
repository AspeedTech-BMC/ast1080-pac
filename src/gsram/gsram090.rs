#[doc = "Register `GSRAM090` reader"]
pub type R = crate::R<Gsram090Spec>;
#[doc = "Register `GSRAM090` writer"]
pub type W = crate::W<Gsram090Spec>;
#[doc = "Field `WLOCK04` reader - WLOCK04"]
pub type Wlock04R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK04` writer - WLOCK04"]
pub type Wlock04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK04"]
    #[inline(always)]
    pub fn wlock04(&self) -> Wlock04R {
        Wlock04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK04"]
    #[inline(always)]
    pub fn wlock04(&mut self) -> Wlock04W<Gsram090Spec> {
        Wlock04W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK04\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram090Spec;
impl crate::RegisterSpec for Gsram090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram090::R`](R) reader structure"]
impl crate::Readable for Gsram090Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram090::W`](W) writer structure"]
impl crate::Writable for Gsram090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM090 to value 0"]
impl crate::Resettable for Gsram090Spec {}
