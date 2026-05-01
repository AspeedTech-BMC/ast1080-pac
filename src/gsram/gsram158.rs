#[doc = "Register `GSRAM158` reader"]
pub type R = crate::R<Gsram158Spec>;
#[doc = "Register `GSRAM158` writer"]
pub type W = crate::W<Gsram158Spec>;
#[doc = "Field `WLOCK54` reader - WLOCK54"]
pub type Wlock54R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK54` writer - WLOCK54"]
pub type Wlock54W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK54"]
    #[inline(always)]
    pub fn wlock54(&self) -> Wlock54R {
        Wlock54R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK54"]
    #[inline(always)]
    pub fn wlock54(&mut self) -> Wlock54W<Gsram158Spec> {
        Wlock54W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK54\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram158Spec;
impl crate::RegisterSpec for Gsram158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram158::R`](R) reader structure"]
impl crate::Readable for Gsram158Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram158::W`](W) writer structure"]
impl crate::Writable for Gsram158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM158 to value 0"]
impl crate::Resettable for Gsram158Spec {}
