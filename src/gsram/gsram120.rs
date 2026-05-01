#[doc = "Register `GSRAM120` reader"]
pub type R = crate::R<Gsram120Spec>;
#[doc = "Register `GSRAM120` writer"]
pub type W = crate::W<Gsram120Spec>;
#[doc = "Field `WLOCK40` reader - WLOCK40"]
pub type Wlock40R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK40` writer - WLOCK40"]
pub type Wlock40W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK40"]
    #[inline(always)]
    pub fn wlock40(&self) -> Wlock40R {
        Wlock40R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK40"]
    #[inline(always)]
    pub fn wlock40(&mut self) -> Wlock40W<Gsram120Spec> {
        Wlock40W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK40\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram120Spec;
impl crate::RegisterSpec for Gsram120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram120::R`](R) reader structure"]
impl crate::Readable for Gsram120Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram120::W`](W) writer structure"]
impl crate::Writable for Gsram120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM120 to value 0"]
impl crate::Resettable for Gsram120Spec {}
