#[doc = "Register `GSRAM0AC` reader"]
pub type R = crate::R<Gsram0acSpec>;
#[doc = "Register `GSRAM0AC` writer"]
pub type W = crate::W<Gsram0acSpec>;
#[doc = "Field `WLOCK11` reader - WLOCK11"]
pub type Wlock11R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK11` writer - WLOCK11"]
pub type Wlock11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK11"]
    #[inline(always)]
    pub fn wlock11(&self) -> Wlock11R {
        Wlock11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK11"]
    #[inline(always)]
    pub fn wlock11(&mut self) -> Wlock11W<Gsram0acSpec> {
        Wlock11W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK11\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0acSpec;
impl crate::RegisterSpec for Gsram0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0ac::R`](R) reader structure"]
impl crate::Readable for Gsram0acSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0ac::W`](W) writer structure"]
impl crate::Writable for Gsram0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0AC to value 0"]
impl crate::Resettable for Gsram0acSpec {}
