#[doc = "Register `GSRAM164` reader"]
pub type R = crate::R<Gsram164Spec>;
#[doc = "Register `GSRAM164` writer"]
pub type W = crate::W<Gsram164Spec>;
#[doc = "Field `WLOCK57` reader - WLOCK57"]
pub type Wlock57R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK57` writer - WLOCK57"]
pub type Wlock57W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK57"]
    #[inline(always)]
    pub fn wlock57(&self) -> Wlock57R {
        Wlock57R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK57"]
    #[inline(always)]
    pub fn wlock57(&mut self) -> Wlock57W<Gsram164Spec> {
        Wlock57W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK57\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram164Spec;
impl crate::RegisterSpec for Gsram164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram164::R`](R) reader structure"]
impl crate::Readable for Gsram164Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram164::W`](W) writer structure"]
impl crate::Writable for Gsram164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM164 to value 0"]
impl crate::Resettable for Gsram164Spec {}
