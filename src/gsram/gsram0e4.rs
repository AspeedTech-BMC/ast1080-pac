#[doc = "Register `GSRAM0E4` reader"]
pub type R = crate::R<Gsram0e4Spec>;
#[doc = "Register `GSRAM0E4` writer"]
pub type W = crate::W<Gsram0e4Spec>;
#[doc = "Field `WLOCK25` reader - WLOCK25"]
pub type Wlock25R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK25` writer - WLOCK25"]
pub type Wlock25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK25"]
    #[inline(always)]
    pub fn wlock25(&self) -> Wlock25R {
        Wlock25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK25"]
    #[inline(always)]
    pub fn wlock25(&mut self) -> Wlock25W<Gsram0e4Spec> {
        Wlock25W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK25\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0e4Spec;
impl crate::RegisterSpec for Gsram0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0e4::R`](R) reader structure"]
impl crate::Readable for Gsram0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0e4::W`](W) writer structure"]
impl crate::Writable for Gsram0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0E4 to value 0"]
impl crate::Resettable for Gsram0e4Spec {}
