#[doc = "Register `GSRAM0FC` reader"]
pub type R = crate::R<Gsram0fcSpec>;
#[doc = "Register `GSRAM0FC` writer"]
pub type W = crate::W<Gsram0fcSpec>;
#[doc = "Field `WLOCK31` reader - WLOCK31"]
pub type Wlock31R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK31` writer - WLOCK31"]
pub type Wlock31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK31"]
    #[inline(always)]
    pub fn wlock31(&self) -> Wlock31R {
        Wlock31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK31"]
    #[inline(always)]
    pub fn wlock31(&mut self) -> Wlock31W<Gsram0fcSpec> {
        Wlock31W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK31\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0fcSpec;
impl crate::RegisterSpec for Gsram0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0fc::R`](R) reader structure"]
impl crate::Readable for Gsram0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0fc::W`](W) writer structure"]
impl crate::Writable for Gsram0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0FC to value 0"]
impl crate::Resettable for Gsram0fcSpec {}
