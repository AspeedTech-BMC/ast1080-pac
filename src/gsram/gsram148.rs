#[doc = "Register `GSRAM148` reader"]
pub type R = crate::R<Gsram148Spec>;
#[doc = "Register `GSRAM148` writer"]
pub type W = crate::W<Gsram148Spec>;
#[doc = "Field `WLOCK50` reader - WLOCK50"]
pub type Wlock50R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK50` writer - WLOCK50"]
pub type Wlock50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK50"]
    #[inline(always)]
    pub fn wlock50(&self) -> Wlock50R {
        Wlock50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK50"]
    #[inline(always)]
    pub fn wlock50(&mut self) -> Wlock50W<Gsram148Spec> {
        Wlock50W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK50\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram148Spec;
impl crate::RegisterSpec for Gsram148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram148::R`](R) reader structure"]
impl crate::Readable for Gsram148Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram148::W`](W) writer structure"]
impl crate::Writable for Gsram148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM148 to value 0"]
impl crate::Resettable for Gsram148Spec {}
